use axum::Json;

/// Serve the fedisport JSON-LD context.
///
/// GET /ns/fedisport → application/ld+json
pub async fn fedisport_context() -> Json<serde_json::Value> {
    let context = serde_json::json!({
        "@context": {
            "fedisport": "https://fedisport.github.io/vocabulary/ns#",
            "Exercise": "fedisport:Exercise",
            "activityType": "fedisport:activityType",
            "startedAt": "fedisport:startedAt",
            "routeUrl": "fedisport:routeUrl",
            "statsUrl": "fedisport:statsUrl",
            "ride": "fedisport:ride",
            "run": "fedisport:run",
            "swim": "fedisport:swim",
            "walk": "fedisport:walk",
            "hike": "fedisport:hike",
            "virtualRide": "fedisport:virtualRide",
            "distance": "fedisport:distance",
            "duration": "fedisport:duration",
            "elevationGain": "fedisport:elevationGain",
            "avgPace": "fedisport:avgPace",
            "avgHeartRate": "fedisport:avgHeartRate",
            "maxHeartRate": "fedisport:maxHeartRate",
            "avgPower": "fedisport:avgPower",
            "maxPower": "fedisport:maxPower",
            "normalizedPower": "fedisport:normalizedPower",
            "avgCadence": "fedisport:avgCadence"
        }
    });
    Json(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fedisport_context_returns_valid_jsonld() {
        let Json(context) = fedisport_context().await;
        assert!(context.get("@context").is_some());
        let ctx = context.get("@context").unwrap();
        assert!(ctx.get("Exercise").is_some());
        assert!(ctx.get("fedisport").is_some());
        assert_eq!(
            ctx.get("fedisport").unwrap(),
            "https://fedisport.github.io/vocabulary/ns#"
        );
    }
}
