use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityStats {
    pub activity_id: String,
    pub total_distance_meters: Option<f64>,
    pub total_duration_seconds: Option<i64>,
    pub total_elevation_gain_meters: Option<f64>,
    pub total_elevation_loss_meters: Option<f64>,
    pub avg_speed_ms: Option<f64>,
    pub max_speed_ms: Option<f64>,
    pub avg_pace_min_km: Option<f64>,
    pub total_calories: Option<i32>,
    pub avg_heart_rate: Option<i32>,
    pub max_heart_rate: Option<i32>,
    pub avg_power_watts: Option<f64>,
    pub max_power_watts: Option<f64>,
    pub avg_cadence: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct StatsComputer;

impl StatsComputer {
    pub fn new() -> Self {
        Self
    }

    /// Calculate distance between two points using Haversine formula
    pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6_371_000.0; // Earth radius in meters
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
            + lat1.to_radians().cos()
                * lat2.to_radians().cos()
                * (d_lon / 2.0).sin()
                * (d_lon / 2.0).sin();
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        r * c
    }

    /// Calculate total distance from coordinates
    pub fn calculate_distance(coordinates: &[[f64; 2]]) -> f64 {
        if coordinates.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..coordinates.len() {
            let (lat1, lon1) = (coordinates[i - 1][1], coordinates[i - 1][0]);
            let (lat2, lon2) = (coordinates[i][1], coordinates[i][0]);
            total += Self::haversine_distance(lat1, lon1, lat2, lon2);
        }
        total
    }

    /// Calculate total duration from timestamps
    pub fn calculate_duration(start: &str, end: &str) -> Option<i64> {
        let start_time = chrono::NaiveDateTime::parse_from_str(start, "%Y-%m-%dT%H:%M:%S%.fZ")
            .or_else(|_| chrono::NaiveDateTime::parse_from_str(start, "%Y-%m-%dT%H:%M:%SZ"))
            .ok()?;
        let end_time = chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%dT%H:%M:%S%.fZ")
            .or_else(|_| chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%dT%H:%M:%SZ"))
            .ok()?;
        Some((end_time - start_time).num_seconds())
    }

    /// Calculate elevation gain from elevation data
    pub fn calculate_elevation(elevations: &[f64]) -> (f64, f64) {
        let mut gain = 0.0;
        let mut loss = 0.0;

        for i in 1..elevations.len() {
            let diff = elevations[i] - elevations[i - 1];
            if diff > 0.0 {
                gain += diff;
            } else {
                loss -= diff;
            }
        }

        (gain, loss)
    }

    /// Calculate average and max speed from coordinates and timestamps
    pub fn calculate_speed(
        coordinates: &[[f64; 2]],
        timestamps: &[String],
    ) -> (Option<f64>, Option<f64>) {
        if coordinates.len() < 2 || timestamps.len() < 2 {
            return (None, None);
        }

        let mut speeds = Vec::new();

        for i in 1..coordinates.len().min(timestamps.len()) {
            if let (Some(start), Some(end)) = (
                chrono::NaiveDateTime::parse_from_str(&timestamps[i - 1], "%Y-%m-%dT%H:%M:%S%.fZ")
                    .or_else(|_| {
                        chrono::NaiveDateTime::parse_from_str(
                            &timestamps[i - 1],
                            "%Y-%m-%dT%H:%M:%SZ",
                        )
                    })
                    .ok(),
                chrono::NaiveDateTime::parse_from_str(&timestamps[i], "%Y-%m-%dT%H:%M:%S%.fZ")
                    .or_else(|_| {
                        chrono::NaiveDateTime::parse_from_str(&timestamps[i], "%Y-%m-%dT%H:%M:%SZ")
                    })
                    .ok(),
            ) {
                let dt = (end - start).num_seconds() as f64;
                if dt > 0.0 {
                    let (lat1, lon1) = (coordinates[i - 1][1], coordinates[i - 1][0]);
                    let (lat2, lon2) = (coordinates[i][1], coordinates[i][0]);
                    let distance = Self::haversine_distance(lat1, lon1, lat2, lon2);
                    speeds.push(distance / dt);
                }
            }
        }

        if speeds.is_empty() {
            return (None, None);
        }

        let avg = speeds.iter().sum::<f64>() / speeds.len() as f64;
        let max = speeds.iter().cloned().fold(f64::MIN, f64::max);
        (Some(avg), Some(max))
    }

    /// Compute full stats from route data
    pub fn compute_stats(
        &self,
        activity_id: &str,
        coordinates: &[[f64; 2]],
        elevations: &[f64],
        timestamps: &[String],
        start_time: &str,
        end_time: &str,
    ) -> ActivityStats {
        let distance = Self::calculate_distance(coordinates);
        let duration = Self::calculate_duration(start_time, end_time).unwrap_or(0);
        let (gain, loss) = Self::calculate_elevation(elevations);
        let (avg_speed, max_speed) = Self::calculate_speed(coordinates, timestamps);

        // Calculate pace (min/km) for running
        let pace = if distance > 0.0 && duration > 0 {
            Some((duration as f64 / 60.0) / (distance / 1000.0))
        } else {
            None
        };

        ActivityStats {
            activity_id: activity_id.to_string(),
            total_distance_meters: Some(distance),
            total_duration_seconds: Some(duration),
            total_elevation_gain_meters: Some(gain),
            total_elevation_loss_meters: Some(loss),
            avg_speed_ms: avg_speed,
            max_speed_ms: max_speed,
            avg_pace_min_km: pace,
            total_calories: None, // Requires heart rate / power data
            avg_heart_rate: None,
            max_heart_rate: None,
            avg_power_watts: None,
            max_power_watts: None,
            avg_cadence: None,
        }
    }
}

impl Default for StatsComputer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_same_point() {
        let dist = StatsComputer::haversine_distance(52.52, 13.405, 52.52, 13.405);
        assert!((dist - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_haversine_known_distance() {
        // Berlin to Munich ~585 km
        let dist = StatsComputer::haversine_distance(52.52, 13.405, 48.1351, 11.5820);
        assert!(dist > 500_000.0 && dist < 650_000.0);
    }

    #[test]
    fn test_calculate_distance() {
        let coords = [[13.405, 52.52], [13.406, 52.521]];
        let dist = StatsComputer::calculate_distance(&coords);
        assert!(dist > 0.0 && dist < 1000.0);
    }

    #[test]
    fn test_calculate_distance_empty() {
        let coords: [[f64; 2]; 0] = [];
        let dist = StatsComputer::calculate_distance(&coords);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_calculate_elevation_gain() {
        let elevations = vec![100.0, 150.0, 120.0, 200.0];
        let (gain, loss) = StatsComputer::calculate_elevation(&elevations);
        assert_eq!(gain, 130.0); // 50 + 80
        assert_eq!(loss, 30.0); // 30
    }

    #[test]
    fn test_calculate_duration() {
        let start = "2024-01-15T08:00:00Z";
        let end = "2024-01-15T09:00:00Z";
        let duration = StatsComputer::calculate_duration(start, end);
        assert_eq!(duration, Some(3600));
    }

    #[test]
    fn test_compute_stats() {
        let computer = StatsComputer::new();
        let coords = [[13.405, 52.52], [13.406, 52.521]];
        let elevations = vec![100.0, 110.0];
        let timestamps = vec![
            "2024-01-15T08:00:00Z".to_string(),
            "2024-01-15T08:01:00Z".to_string(),
        ];

        let stats = computer.compute_stats(
            "test-123",
            &coords,
            &elevations,
            &timestamps,
            "2024-01-15T08:00:00Z",
            "2024-01-15T08:01:00Z",
        );

        assert_eq!(stats.activity_id, "test-123");
        assert!(stats.total_distance_meters.unwrap() > 0.0);
        assert_eq!(stats.total_duration_seconds, Some(60));
        assert!(stats.total_elevation_gain_meters.unwrap() > 0.0);
    }
}
