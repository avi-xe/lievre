use crate::activity::{ActivityType, CreateActivity};
use crate::route::CreateRoute;

#[derive(Debug, Clone)]
pub struct GpxTrack {
    pub name: Option<String>,
    pub activity_type: ActivityType,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub coordinates: Vec<Vec<f64>>,
    pub elevation_data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct GpxParser;

impl GpxParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, gpx_content: &str) -> anyhow::Result<GpxTrack> {
        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(gpx_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut name: Option<String> = None;
        let mut coordinates: Vec<Vec<f64>> = Vec::new();
        let mut elevation_data: Vec<f64> = Vec::new();
        let mut times: Vec<chrono::DateTime<chrono::Utc>> = Vec::new();

        let mut in_trkpt = false;
        let mut current_lat: Option<f64> = None;
        let mut current_lon: Option<f64> = None;
        let mut current_ele: Option<f64> = None;
        let mut current_time: Option<chrono::DateTime<chrono::Utc>> = None;
        let mut in_name = false;
        let mut in_ele = false;
        let mut in_time = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"trkpt" => {
                            in_trkpt = true;
                            current_lat = None;
                            current_lon = None;
                            current_ele = None;
                            current_time = None;
                            
                            for attr in e.attributes().flatten() {
                                match attr.key.as_ref() {
                                    b"lat" => {
                                        current_lat = attr.decode_and_unescape_value(&reader)?.parse().ok();
                                    }
                                    b"lon" => {
                                        current_lon = attr.decode_and_unescape_value(&reader)?.parse().ok();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        b"name" => in_name = true,
                        b"ele" => in_ele = true,
                        b"time" => in_time = true,
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();
                    if in_name {
                        name = Some(text.clone());
                    } else if in_ele {
                        if let Ok(ele) = text.parse::<f64>() {
                            current_ele = Some(ele);
                        }
                    } else if in_time {
                        if let Ok(time) = text.parse::<chrono::DateTime<chrono::Utc>>() {
                            current_time = Some(time);
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    match e.name().as_ref() {
                        b"trkpt" => {
                            in_trkpt = false;
                            if let (Some(lat), Some(lon)) = (current_lat, current_lon) {
                                // GPX format is lat, lon - we store as [lon, lat]
                                coordinates.push(vec![lon, lat]);
                            }
                            if let Some(ele) = current_ele {
                                elevation_data.push(ele);
                            }
                            if let Some(time) = current_time {
                                times.push(time);
                            }
                        }
                        b"name" => in_name = false,
                        b"ele" => in_ele = false,
                        b"time" => in_time = false,
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow::anyhow!("GPX parse error: {}", e)),
                _ => {}
            }
            buf.clear();
        }

        let start_time = times.first().copied().unwrap_or_else(chrono::Utc::now);
        let end_time = times.last().copied().unwrap_or(start_time);
        let duration_seconds = Some((end_time - start_time).num_seconds());
        
        let distance_meters = if !coordinates.is_empty() {
            Some(self.calculate_total_distance(&coordinates))
        } else {
            None
        };

        let elevation_gain_meters = if !elevation_data.is_empty() {
            Some(self.calculate_elevation_gain(&elevation_data))
        } else {
            None
        };

        let activity_type = self.detect_activity_type(name.as_deref());

        Ok(GpxTrack {
            name,
            activity_type,
            start_time,
            duration_seconds,
            distance_meters,
            elevation_gain_meters,
            coordinates,
            elevation_data,
        })
    }

    pub fn to_create_activity(&self, track: &GpxTrack) -> CreateActivity {
        CreateActivity {
            activity_type: track.activity_type.clone(),
            title: track.name.clone(),
            description: None,
            started_at: track.start_time,
            duration_seconds: track.duration_seconds,
            distance_meters: track.distance_meters,
            elevation_gain_meters: track.elevation_gain_meters,
            visibility: None,
        }
    }

    pub fn to_create_route(&self, activity_id: &str, track: &GpxTrack) -> CreateRoute {
        CreateRoute {
            activity_id: activity_id.to_string(),
            coordinates: track.coordinates.clone(),
            elevation_data: Some(track.elevation_data.clone()),
        }
    }

    /// Calculate distance between two points using Haversine formula
    pub fn haversine_distance(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371000.0; // Earth's radius in meters
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        let a = (d_lat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        r * c
    }

    /// Calculate total distance from coordinates
    pub fn calculate_total_distance(&self, coordinates: &[Vec<f64>]) -> f64 {
        if coordinates.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..coordinates.len() {
            let prev = &coordinates[i - 1];
            let curr = &coordinates[i];
            // GPX format is [lat, lon, ele] but we store as [lon, lat]
            total += self.haversine_distance(prev[1], prev[0], curr[1], curr[0]);
        }
        total
    }

    /// Calculate elevation gain from elevation data
    pub fn calculate_elevation_gain(&self, elevations: &[f64]) -> f64 {
        if elevations.len() < 2 {
            return 0.0;
        }

        let mut gain = 0.0;
        for i in 1..elevations.len() {
            let diff = elevations[i] - elevations[i - 1];
            if diff > 0.0 {
                gain += diff;
            }
        }
        gain
    }

    /// Detect activity type from filename or track name
    pub fn detect_activity_type(&self, name: Option<&str>) -> ActivityType {
        let name_lower = name.unwrap_or("").to_lowercase();
        
        if name_lower.contains("run") || name_lower.contains("jog") {
            ActivityType::Run
        } else if name_lower.contains("swim") {
            ActivityType::Swim
        } else if name_lower.contains("walk") || name_lower.contains("hike") {
            ActivityType::Walk
        } else if name_lower.contains("virtual") || name_lower.contains("trainer") {
            ActivityType::VirtualRide
        } else {
            ActivityType::Ride // Default to ride
        }
    }
}

impl Default for GpxParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gpx_minimal() {
        let parser = GpxParser::new();
        let gpx_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="test">
  <trk>
    <name>Morning Ride</name>
    <trkseg>
      <trkpt lat="52.5200" lon="13.4050"><ele>100</ele><time>2024-01-15T08:00:00Z</time></trkpt>
      <trkpt lat="52.5210" lon="13.4060"><ele>110</ele><time>2024-01-15T08:01:00Z</time></trkpt>
      <trkpt lat="52.5220" lon="13.4070"><ele>120</ele><time>2024-01-15T08:02:00Z</time></trkpt>
    </trkseg>
  </trk>
</gpx>"#;
        let track = parser.parse(gpx_content).unwrap();
        
        assert_eq!(track.name, Some("Morning Ride".to_string()));
        assert_eq!(track.activity_type, ActivityType::Ride);
        assert_eq!(track.coordinates.len(), 3);
        assert_eq!(track.elevation_data, vec![100.0, 110.0, 120.0]);
        assert!(track.distance_meters.unwrap() > 0.0);
        assert!(track.elevation_gain_meters.unwrap() > 0.0);
    }

    #[test]
    fn test_parse_gpx_with_run_detection() {
        let parser = GpxParser::new();
        let gpx_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1">
  <trk>
    <name>Evening Run</name>
    <trkseg>
      <trkpt lat="52.5200" lon="13.4050"><time>2024-01-15T18:00:00Z</time></trkpt>
      <trkpt lat="52.5210" lon="13.4060"><time>2024-01-15T18:01:00Z</time></trkpt>
    </trkseg>
  </trk>
</gpx>"#;
        let track = parser.parse(gpx_content).unwrap();
        assert_eq!(track.activity_type, ActivityType::Run);
    }

    #[test]
    fn test_parse_gpx_empty_track() {
        let parser = GpxParser::new();
        let gpx_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1">
  <trk>
    <trkseg></trkseg>
  </trk>
</gpx>"#;
        let track = parser.parse(gpx_content).unwrap();
        assert_eq!(track.coordinates.len(), 0);
    }

    #[test]
    fn test_haversine_distance() {
        let parser = GpxParser::new();
        
        // Berlin to Potsdam (approx 27km)
        let distance = parser.haversine_distance(52.5200, 13.4050, 52.3906, 13.0645);
        assert!(distance > 25000.0 && distance < 30000.0);
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let parser = GpxParser::new();
        let distance = parser.haversine_distance(52.5200, 13.4050, 52.5200, 13.4050);
        assert!(distance < 1.0);
    }

    #[test]
    fn test_calculate_total_distance() {
        let parser = GpxParser::new();
        let coordinates = vec![
            vec![13.4050, 52.5200], // [lon, lat]
            vec![13.4060, 52.5210],
            vec![13.4070, 52.5220],
        ];
        let distance = parser.calculate_total_distance(&coordinates);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_calculate_total_distance_empty() {
        let parser = GpxParser::new();
        let coordinates = vec![];
        let distance = parser.calculate_total_distance(&coordinates);
        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_calculate_elevation_gain() {
        let parser = GpxParser::new();
        let elevations = vec![100.0, 150.0, 120.0, 200.0, 180.0];
        let gain = parser.calculate_elevation_gain(&elevations);
        assert_eq!(gain, 130.0); // 50 + 0 + 80 + 0
    }

    #[test]
    fn test_calculate_elevation_gain_descending() {
        let parser = GpxParser::new();
        let elevations = vec![200.0, 150.0, 100.0];
        let gain = parser.calculate_elevation_gain(&elevations);
        assert_eq!(gain, 0.0);
    }

    #[test]
    fn test_detect_activity_type_run() {
        let parser = GpxParser::new();
        assert_eq!(parser.detect_activity_type(Some("Morning Run")), ActivityType::Run);
        assert_eq!(parser.detect_activity_type(Some("jog_in_park")), ActivityType::Run);
    }

    #[test]
    fn test_detect_activity_type_swim() {
        let parser = GpxParser::new();
        assert_eq!(parser.detect_activity_type(Some("Pool Swim")), ActivityType::Swim);
    }

    #[test]
    fn test_detect_activity_type_ride() {
        let parser = GpxParser::new();
        assert_eq!(parser.detect_activity_type(Some("Road Bike Ride")), ActivityType::Ride);
        assert_eq!(parser.detect_activity_type(None), ActivityType::Ride);
    }

    #[test]
    fn test_detect_activity_type_virtual() {
        let parser = GpxParser::new();
        assert_eq!(parser.detect_activity_type(Some("Virtual Ride - Zwift")), ActivityType::VirtualRide);
    }

    #[test]
    fn test_to_create_activity() {
        let parser = GpxParser::new();
        let track = GpxTrack {
            name: Some("Test Ride".to_string()),
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            coordinates: vec![],
            elevation_data: vec![],
        };

        let activity = parser.to_create_activity(&track);
        assert_eq!(activity.activity_type, ActivityType::Ride);
        assert_eq!(activity.title, Some("Test Ride".to_string()));
        assert_eq!(activity.duration_seconds, Some(3600));
    }

    #[test]
    fn test_to_create_route() {
        let parser = GpxParser::new();
        let track = GpxTrack {
            name: None,
            activity_type: ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            coordinates: vec![vec![13.405, 52.52], vec![13.406, 52.521]],
            elevation_data: vec![100.0, 110.0],
        };

        let route = parser.to_create_route("activity-123", &track);
        assert_eq!(route.activity_id, "activity-123");
        assert_eq!(route.coordinates.len(), 2);
    }
}
