use crate::activity::{ActivityType, CreateActivity};
use crate::route::CreateRoute;

#[derive(Debug, Clone)]
pub struct FitSession {
    pub name: Option<String>,
    pub activity_type: ActivityType,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub avg_heart_rate: Option<i32>,
    pub max_heart_rate: Option<i32>,
    pub avg_power: Option<f64>,
    pub max_power: Option<f64>,
    pub avg_cadence: Option<f64>,
    pub avg_speed: Option<f64>,
    pub max_speed: Option<f64>,
    pub calories: Option<i32>,
    pub coordinates: Vec<Vec<f64>>,
    pub elevation_data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct FitParser;

impl FitParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, fit_data: &[u8]) -> anyhow::Result<FitSession> {
        let records = fitparser::from_bytes(fit_data)
            .map_err(|e| anyhow::anyhow!("FIT parse error: {}", e))?;

        let mut session = FitSession {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_power: None,
            max_power: None,
            avg_cadence: None,
            avg_speed: None,
            max_speed: None,
            calories: None,
            coordinates: Vec::new(),
            elevation_data: Vec::new(),
        };

        for record in &records {
            match record.kind() {
                fitparser::profile::MesgNum::Session => {
                    for field in record.fields() {
                        match field.name() {
                            "start_time" => {
                                if let fitparser::Value::Timestamp(val) = field.value() {
                                    let epoch = chrono::DateTime::parse_from_rfc3339("1989-12-31T00:00:00+00:00")
                                        .unwrap()
                                        .timestamp();
                                    // val is DateTime<Local>, convert to UTC
                                    session.start_time = val.with_timezone(&chrono::Utc);
                                }
                            }
                            "total_timer_time" => {
                                if let fitparser::Value::Float64(val) = field.value() {
                                    session.duration_seconds = Some(*val as i64);
                                }
                            }
                            "total_distance" => {
                                if let fitparser::Value::Float64(val) = field.value() {
                                    session.distance_meters = Some(*val);
                                }
                            }
                            "total_ascent" => {
                                if let fitparser::Value::UInt16(val) = field.value() {
                                    session.elevation_gain_meters = Some(*val as f64);
                                }
                            }
                            "avg_heart_rate" => {
                                if let fitparser::Value::UInt8(val) = field.value() {
                                    session.avg_heart_rate = Some(*val as i32);
                                }
                            }
                            "max_heart_rate" => {
                                if let fitparser::Value::UInt8(val) = field.value() {
                                    session.max_heart_rate = Some(*val as i32);
                                }
                            }
                            "avg_power" => {
                                if let fitparser::Value::UInt16(val) = field.value() {
                                    session.avg_power = Some(*val as f64);
                                }
                            }
                            "max_power" => {
                                if let fitparser::Value::UInt16(val) = field.value() {
                                    session.max_power = Some(*val as f64);
                                }
                            }
                            "avg_cadence" => {
                                if let fitparser::Value::UInt8(val) = field.value() {
                                    session.avg_cadence = Some(*val as f64);
                                }
                            }
                            "avg_speed" => {
                                if let fitparser::Value::Float64(val) = field.value() {
                                    session.avg_speed = Some(*val);
                                }
                            }
                            "max_speed" => {
                                if let fitparser::Value::Float64(val) = field.value() {
                                    session.max_speed = Some(*val);
                                }
                            }
                            "total_calories" => {
                                if let fitparser::Value::UInt16(val) = field.value() {
                                    session.calories = Some(*val as i32);
                                }
                            }
                            "sport" => {
                                if let fitparser::Value::Enum(val) = field.value() {
                                    session.activity_type = match val {
                                        0 => ActivityType::Run,
                                        1 => ActivityType::Run,
                                        2 => ActivityType::Ride,
                                        5 => ActivityType::Swim,
                                        11 => ActivityType::Hike,
                                        _ => ActivityType::Ride,
                                    };
                                }
                            }
                            _ => {}
                        }
                    }
                }
                fitparser::profile::MesgNum::Record => {
                    let mut position_lat: Option<f64> = None;
                    let mut position_lon: Option<f64> = None;
                    
                    for field in record.fields() {
                        match field.name() {
                            "position_lat" => {
                                if let fitparser::Value::SInt32(val) = field.value() {
                                    position_lat = Some(*val as f64 * (180.0 / 2i32.pow(31) as f64));
                                }
                            }
                            "position_long" => {
                                if let fitparser::Value::SInt32(val) = field.value() {
                                    position_lon = Some(*val as f64 * (180.0 / 2i32.pow(31) as f64));
                                }
                            }
                            "altitude" => {
                                if let fitparser::Value::SInt16(val) = field.value() {
                                    session.elevation_data.push(*val as f64 / 5.0);
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    if let (Some(lat), Some(lon)) = (position_lat, position_lon) {
                        session.coordinates.push(vec![lon, lat]);
                    }
                }
                _ => {}
            }
        }

        // Calculate elevation gain if not provided by FIT file
        if session.elevation_gain_meters.is_none() && !session.elevation_data.is_empty() {
            let mut gain = 0.0;
            for i in 1..session.elevation_data.len() {
                let diff = session.elevation_data[i] - session.elevation_data[i - 1];
                if diff > 0.0 {
                    gain += diff;
                }
            }
            session.elevation_gain_meters = Some(gain);
        }

        Ok(session)
    }

    pub fn to_create_activity(&self, session: &FitSession) -> CreateActivity {
        CreateActivity {
            activity_type: session.activity_type.clone(),
            title: session.name.clone(),
            description: None,
            started_at: session.start_time,
            duration_seconds: session.duration_seconds,
            distance_meters: session.distance_meters,
            elevation_gain_meters: session.elevation_gain_meters,
            visibility: None,
        }
    }

    pub fn to_create_route(&self, activity_id: &str, session: &FitSession) -> CreateRoute {
        CreateRoute {
            activity_id: activity_id.to_string(),
            coordinates: session.coordinates.clone(),
            elevation_data: Some(session.elevation_data.clone()),
        }
    }
}

impl Default for FitParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_parser_new() {
        let parser = FitParser::new();
        let session = FitSession {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_power: None,
            max_power: None,
            avg_cadence: None,
            avg_speed: None,
            max_speed: None,
            calories: None,
            coordinates: vec![],
            elevation_data: vec![],
        };
        assert!(parser.to_create_activity(&session).activity_type == ActivityType::Ride);
    }

    #[test]
    fn test_fit_to_create_activity() {
        let parser = FitParser::new();
        let session = FitSession {
            name: Some("Morning Ride".to_string()),
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            avg_heart_rate: Some(150),
            max_heart_rate: Some(180),
            avg_power: Some(200.0),
            max_power: Some(400.0),
            avg_cadence: Some(85.0),
            avg_speed: Some(14.0),
            max_speed: Some(25.0),
            calories: Some(1500),
            coordinates: vec![],
            elevation_data: vec![],
        };

        let activity = parser.to_create_activity(&session);
        assert_eq!(activity.activity_type, ActivityType::Ride);
        assert_eq!(activity.title, Some("Morning Ride".to_string()));
        assert_eq!(activity.duration_seconds, Some(3600));
        assert_eq!(activity.distance_meters, Some(50000.0));
    }

    #[test]
    fn test_fit_to_create_route() {
        let parser = FitParser::new();
        let session = FitSession {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_power: None,
            max_power: None,
            avg_cadence: None,
            avg_speed: None,
            max_speed: None,
            calories: None,
            coordinates: vec![vec![13.405, 52.52], vec![13.406, 52.521]],
            elevation_data: vec![100.0, 110.0],
        };

        let route = parser.to_create_route("activity-123", &session);
        assert_eq!(route.activity_id, "activity-123");
        assert_eq!(route.coordinates.len(), 2);
    }
}
