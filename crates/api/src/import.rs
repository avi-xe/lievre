use axum::{
    extract::{Multipart, State},
    Json,
};
use lievre_core::{ActivityRepository, GpxParser, RouteRepository, CreateActivity, CreateRoute};
use lievre_shared::Error;

#[derive(Debug, serde::Serialize)]
pub struct ImportResponse {
    pub activity_id: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ImportError {
    pub error: String,
    pub file: Option<String>,
}

pub async fn import_gpx(
    State((activity_repo, route_repo)): State<(ActivityRepository, RouteRepository)>,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, Error> {
    let parser = GpxParser::new();
    
    while let Some(field) = multipart.next_field().await.map_err(|e| Error::BadRequest(e.to_string()))? {
        let name = field.name().unwrap_or("unknown").to_string();
        
        if name == "file" {
            let data = field.bytes().await.map_err(|e| Error::BadRequest(e.to_string()))?;
            let gpx_content = String::from_utf8(data.to_vec()).map_err(|e| Error::BadRequest(e.to_string()))?;
            
            let track = parser.parse(&gpx_content).map_err(|e| Error::BadRequest(e.to_string()))?;
            
            // Create activity (using a placeholder user_id for now)
            let user_id = "placeholder-user-id";
            let create_activity = parser.to_create_activity(&track);
            let activity = activity_repo.create(user_id, create_activity).await.map_err(|e| Error::Internal(e))?;
            
            // Create route if coordinates exist
            if !track.coordinates.is_empty() {
                let create_route = parser.to_create_route(&activity.id, &track);
                route_repo.create(create_route).await.map_err(|e| Error::Internal(e))?;
            }
            
            return Ok(Json(ImportResponse {
                activity_id: activity.id,
                message: format!("Successfully imported activity from {}", name),
            }));
        }
    }
    
    Err(Error::BadRequest("No file provided".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_response_serialization() {
        let response = ImportResponse {
            activity_id: "test-id".to_string(),
            message: "Success".to_string(),
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("activity_id"));
        assert!(json.contains("message"));
    }

    #[test]
    fn test_import_error_serialization() {
        let error = ImportError {
            error: "Invalid file".to_string(),
            file: Some("test.gpx".to_string()),
        };
        
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("error"));
        assert!(json.contains("file"));
    }
}
