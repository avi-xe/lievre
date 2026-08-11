use crate::activity::{ActivityType, CreateActivity};
use crate::route::CreateRoute;

#[derive(Debug, Clone)]
pub struct StravaActivity {
    pub name: Option<String>,
    pub activity_type: ActivityType,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub avg_speed: Option<f64>,
    pub max_speed: Option<f64>,
    pub avg_heart_rate: Option<i32>,
    pub max_heart_rate: Option<i32>,
    pub avg_power: Option<f64>,
    pub max_power: Option<f64>,
    pub calories: Option<i32>,
    pub description: Option<String>,
    pub coordinates: Vec<Vec<f64>>,
    pub elevation_data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct StravaParser;

impl StravaParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_activities_csv(&self, csv_content: &str) -> anyhow::Result<Vec<StravaActivity>> {
        let mut activities = Vec::new();
        let mut reader = csv::Reader::from_reader(csv_content.as_bytes());

        for result in reader.records() {
            let record = result.map_err(|e| anyhow::anyhow!("CSV parse error: {}", e))?;

            let name = record.get(1).map(|s| s.to_string());
            let activity_type_str = record.get(2).unwrap_or("Ride");
            let activity_type = match activity_type_str {
                "Ride" => ActivityType::Ride,
                "Run" => ActivityType::Run,
                "Swim" => ActivityType::Swim,
                "Walk" => ActivityType::Walk,
                "Hike" => ActivityType::Hike,
                "VirtualRide" => ActivityType::VirtualRide,
                _ => ActivityType::Ride,
            };

            let start_time = record
                .get(3)
                .and_then(|s| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S UTC").ok()
                })
                .map(|ndt| ndt.and_utc())
                .unwrap_or_else(chrono::Utc::now);

            let elapsed_time = record
                .get(4)
                .and_then(|s| s.parse::<f64>().ok())
                .map(|t| t as i64);

            let distance = record.get(5).and_then(|s| s.parse::<f64>().ok());

            let elevation = record
                .get(6)
                .and_then(|s| s.parse::<f64>().ok())
                .map(|e| e * 0.3048); // Strava exports elevation in feet

            let avg_speed = record
                .get(7)
                .and_then(|s| s.parse::<f64>().ok())
                .map(|s| s * 0.44704); // Strava exports speed in mph

            let max_speed = record
                .get(8)
                .and_then(|s| s.parse::<f64>().ok())
                .map(|s| s * 0.44704);

            let avg_hr = record.get(9).and_then(|s| s.parse::<i32>().ok());

            let max_hr = record.get(10).and_then(|s| s.parse::<i32>().ok());

            let avg_power = record.get(11).and_then(|s| s.parse::<f64>().ok());

            let max_power = record.get(12).and_then(|s| s.parse::<f64>().ok());

            let calories = record.get(13).and_then(|s| s.parse::<i32>().ok());

            let description = record.get(14).map(|s| s.to_string());

            activities.push(StravaActivity {
                name,
                activity_type,
                start_time,
                duration_seconds: elapsed_time,
                distance_meters: distance,
                elevation_gain_meters: elevation,
                avg_speed,
                max_speed,
                avg_heart_rate: avg_hr,
                max_heart_rate: max_hr,
                avg_power,
                max_power,
                calories,
                description,
                coordinates: Vec::new(),
                elevation_data: Vec::new(),
            });
        }

        Ok(activities)
    }

    pub fn to_create_activity(&self, strava: &StravaActivity) -> CreateActivity {
        CreateActivity {
            activity_type: strava.activity_type.clone(),
            title: strava.name.clone(),
            description: strava.description.clone(),
            started_at: strava.start_time,
            duration_seconds: strava.duration_seconds,
            distance_meters: strava.distance_meters,
            elevation_gain_meters: strava.elevation_gain_meters,
            visibility: None,
        }
    }

    pub fn to_create_route(&self, activity_id: &str, strava: &StravaActivity) -> CreateRoute {
        CreateRoute {
            activity_id: activity_id.to_string(),
            coordinates: strava.coordinates.clone(),
            elevation_data: Some(strava.elevation_data.clone()),
        }
    }
}

impl Default for StravaParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strava_parser_new() {
        let parser = StravaParser::new();
        let strava = StravaActivity {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_speed: None,
            max_speed: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_power: None,
            max_power: None,
            calories: None,
            description: None,
            coordinates: vec![],
            elevation_data: vec![],
        };
        assert!(parser.to_create_activity(&strava).activity_type == ActivityType::Ride);
    }

    #[test]
    fn test_strava_to_create_activity() {
        let parser = StravaParser::new();
        let strava = StravaActivity {
            name: Some("Morning Ride".to_string()),
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            avg_speed: Some(14.0),
            max_speed: Some(25.0),
            avg_heart_rate: Some(150),
            max_heart_rate: Some(180),
            avg_power: Some(200.0),
            max_power: Some(400.0),
            calories: Some(1500),
            description: Some("Great ride!".to_string()),
            coordinates: vec![],
            elevation_data: vec![],
        };

        let activity = parser.to_create_activity(&strava);
        assert_eq!(activity.activity_type, ActivityType::Ride);
        assert_eq!(activity.title, Some("Morning Ride".to_string()));
        assert_eq!(activity.description, Some("Great ride!".to_string()));
        assert_eq!(activity.duration_seconds, Some(3600));
    }

    #[test]
    fn test_strava_to_create_route() {
        let parser = StravaParser::new();
        let strava = StravaActivity {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_speed: None,
            max_speed: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_power: None,
            max_power: None,
            calories: None,
            description: None,
            coordinates: vec![vec![13.405, 52.52], vec![13.406, 52.521]],
            elevation_data: vec![100.0, 110.0],
        };

        let route = parser.to_create_route("activity-123", &strava);
        assert_eq!(route.activity_id, "activity-123");
        assert_eq!(route.coordinates.len(), 2);
    }

    #[test]
    fn test_parse_strava_csv() {
        let csv_content = r#"Activity ID,Activity Name,Activity Type,Activity Date,Elapsed Time,Distance,Total Elevation Gain,Average Speed,Maximum Speed,Average Heart Rate,Maximum Heart Rate,Average Power,Maximum Power,Calories,Activity Description
12345678,Morning Ride,Ride,2024/01/15 08:00:00 UTC,3600,31.07,1640,13.5,25.0,150,180,200,400,1500,Great ride!"#;

        let parser = StravaParser::new();
        let activities = parser.parse_activities_csv(csv_content).unwrap();

        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].name, Some("Morning Ride".to_string()));
        assert_eq!(activities[0].activity_type, ActivityType::Ride);
        assert_eq!(activities[0].duration_seconds, Some(3600));
        assert!(activities[0].distance_meters.is_some());
        assert!(activities[0].elevation_gain_meters.is_some());
    }
}
