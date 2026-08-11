use crate::activity::{ActivityType, CreateActivity};
use crate::route::CreateRoute;

#[derive(Debug, Clone)]
pub struct TcxActivity {
    pub name: Option<String>,
    pub activity_type: ActivityType,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub avg_heart_rate: Option<i32>,
    pub max_heart_rate: Option<i32>,
    pub avg_speed: Option<f64>,
    pub max_speed: Option<f64>,
    pub calories: Option<i32>,
    pub coordinates: Vec<Vec<f64>>,
    pub elevation_data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct TcxParser;

impl TcxParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, tcx_content: &str) -> anyhow::Result<TcxActivity> {
        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(tcx_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut activity = TcxActivity {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_speed: None,
            max_speed: None,
            calories: None,
            coordinates: Vec::new(),
            elevation_data: Vec::new(),
        };

        let mut in_lap = false;
        let mut in_trackpoint = false;
        let mut in_time = false;
        let mut in_distance = false;
        let mut in_altitude = false;
        let mut in_heartrate = false;
        let mut in_calories = false;
        let mut in_totaltime = false;
        let mut total_distance: f64 = 0.0;
        let mut total_time: f64 = 0.0;
        let _total_elevation: f64 = 0.0;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"Lap" => in_lap = true,
                    b"Trackpoint" => in_trackpoint = true,
                    b"Time" if in_trackpoint => in_time = true,
                    b"DistanceMeters" if in_lap => in_distance = true,
                    b"AltitudeMeters" if in_trackpoint => in_altitude = true,
                    b"Heartratebpm" if in_trackpoint => in_heartrate = true,
                    b"Calories" if in_lap => in_calories = true,
                    b"TotalTimeSeconds" if in_lap => in_totaltime = true,
                    _ => {}
                },
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();

                    if in_time && in_trackpoint {
                        if let Ok(_time) = text.parse::<chrono::DateTime<chrono::Utc>>() {
                            activity.coordinates.push(Vec::new()); // Placeholder
                        }
                    } else if in_altitude && in_trackpoint {
                        if let Ok(alt) = text.parse::<f64>() {
                            activity.elevation_data.push(alt);
                        }
                    } else if in_heartrate && in_trackpoint {
                        if let Ok(hr) = text.parse::<i32>() {
                            activity.avg_heart_rate = Some(hr);
                        }
                    } else if in_distance && in_lap {
                        if let Ok(dist) = text.parse::<f64>() {
                            total_distance += dist;
                        }
                    } else if in_calories && in_lap {
                        if let Ok(cal) = text.parse::<i32>() {
                            activity.calories = Some(cal);
                        }
                    } else if in_totaltime && in_lap {
                        if let Ok(time) = text.parse::<f64>() {
                            total_time += time;
                        }
                    }
                }
                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"Lap" => in_lap = false,
                    b"Trackpoint" => in_trackpoint = false,
                    b"Time" => in_time = false,
                    b"DistanceMeters" => in_distance = false,
                    b"AltitudeMeters" => in_altitude = false,
                    b"Heartratebpm" => in_heartrate = false,
                    b"Calories" => in_calories = false,
                    b"TotalTimeSeconds" => in_totaltime = false,
                    _ => {}
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow::anyhow!("TCX parse error: {}", e)),
                _ => {}
            }
            buf.clear();
        }

        activity.distance_meters = Some(total_distance);
        activity.duration_seconds = Some(total_time as i64);

        // Calculate elevation gain
        if activity.elevation_data.len() > 1 {
            let mut gain = 0.0;
            for i in 1..activity.elevation_data.len() {
                let diff = activity.elevation_data[i] - activity.elevation_data[i - 1];
                if diff > 0.0 {
                    gain += diff;
                }
            }
            activity.elevation_gain_meters = Some(gain);
        }

        Ok(activity)
    }

    pub fn to_create_activity(&self, tcx: &TcxActivity) -> CreateActivity {
        CreateActivity {
            activity_type: tcx.activity_type.clone(),
            title: tcx.name.clone(),
            description: None,
            started_at: tcx.start_time,
            duration_seconds: tcx.duration_seconds,
            distance_meters: tcx.distance_meters,
            elevation_gain_meters: tcx.elevation_gain_meters,
            visibility: None,
        }
    }

    pub fn to_create_route(&self, activity_id: &str, tcx: &TcxActivity) -> CreateRoute {
        CreateRoute {
            activity_id: activity_id.to_string(),
            coordinates: tcx.coordinates.clone(),
            elevation_data: Some(tcx.elevation_data.clone()),
        }
    }
}

impl Default for TcxParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcx_parser_new() {
        let parser = TcxParser::new();
        let tcx = TcxActivity {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_speed: None,
            max_speed: None,
            calories: None,
            coordinates: vec![],
            elevation_data: vec![],
        };
        assert!(parser.to_create_activity(&tcx).activity_type == ActivityType::Ride);
    }

    #[test]
    fn test_tcx_to_create_activity() {
        let parser = TcxParser::new();
        let tcx = TcxActivity {
            name: Some("Morning Ride".to_string()),
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            avg_heart_rate: Some(150),
            max_heart_rate: Some(180),
            avg_speed: Some(14.0),
            max_speed: Some(25.0),
            calories: Some(1500),
            coordinates: vec![],
            elevation_data: vec![],
        };

        let activity = parser.to_create_activity(&tcx);
        assert_eq!(activity.activity_type, ActivityType::Ride);
        assert_eq!(activity.title, Some("Morning Ride".to_string()));
        assert_eq!(activity.duration_seconds, Some(3600));
        assert_eq!(activity.distance_meters, Some(50000.0));
    }

    #[test]
    fn test_tcx_to_create_route() {
        let parser = TcxParser::new();
        let tcx = TcxActivity {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_speed: None,
            max_speed: None,
            calories: None,
            coordinates: vec![vec![13.405, 52.52], vec![13.406, 52.521]],
            elevation_data: vec![100.0, 110.0],
        };

        let route = parser.to_create_route("activity-123", &tcx);
        assert_eq!(route.activity_id, "activity-123");
        assert_eq!(route.coordinates.len(), 2);
    }
}
