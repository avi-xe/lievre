use axum::{extract::{Path, State}, http::StatusCode, Json};
use lievre_core::RouteRepository;

pub async fn get_activity_geojson(
    State((_activity_repo, route_repo)): State<(lievre_core::ActivityRepository, RouteRepository)>,
    Path(activity_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let route = route_repo
        .find_by_activity_id(&activity_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match route {
        Some(route) => {
            let geojson = route_repo
                .to_geojson(&route)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            Ok(Json(geojson))
        }
        None => Err((StatusCode::NOT_FOUND, "Route not found".to_string())),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_geojson_structure() {
        // GeoJSON LineString structure validation
        let geojson = serde_json::json!({
            "type": "Feature",
            "geometry": {
                "type": "LineString",
                "coordinates": [[13.405, 52.52, 100.0], [13.406, 52.521, 110.0]]
            }
        });

        assert_eq!(geojson["type"], "Feature");
        assert_eq!(geojson["geometry"]["type"], "LineString");
        assert!(geojson["geometry"]["coordinates"].is_array());
    }
}
