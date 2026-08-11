use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyZone {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_meters: f64,
}

#[derive(Debug, Clone)]
pub struct PrivacyService;

impl PrivacyService {
    pub fn new() -> Self {
        Self
    }

    /// Check if a point is within any of the user's privacy zones
    pub fn is_in_privacy_zone(
        &self,
        latitude: f64,
        longitude: f64,
        privacy_zones: &[PrivacyZone],
    ) -> bool {
        for zone in privacy_zones {
            let distance =
                Self::haversine_distance(latitude, longitude, zone.latitude, zone.longitude);
            if distance <= zone.radius_meters {
                return true;
            }
        }
        false
    }

    /// Blur a point by moving it to the edge of the nearest privacy zone
    pub fn blur_point(
        &self,
        latitude: f64,
        longitude: f64,
        privacy_zones: &[PrivacyZone],
    ) -> (f64, f64) {
        // Find the nearest privacy zone
        let mut nearest_zone: Option<&PrivacyZone> = None;
        let mut min_distance = f64::MAX;

        for zone in privacy_zones {
            let distance =
                Self::haversine_distance(latitude, longitude, zone.latitude, zone.longitude);
            if distance < min_distance {
                min_distance = distance;
                nearest_zone = Some(zone);
            }
        }

        if let Some(zone) = nearest_zone {
            if min_distance <= zone.radius_meters {
                // Point is inside the privacy zone, blur it
                return Self::move_to_edge(
                    latitude,
                    longitude,
                    zone.latitude,
                    zone.longitude,
                    zone.radius_meters,
                );
            }
        }

        // Not in any privacy zone, return original point
        (latitude, longitude)
    }

    /// Blur start and end points of a route
    pub fn blur_route_endpoints(
        &self,
        coordinates: &[[f64; 2]],
        privacy_zones: &[PrivacyZone],
    ) -> Vec<[f64; 2]> {
        if coordinates.is_empty() {
            return coordinates.to_vec();
        }

        let mut result = coordinates.to_vec();

        // Blur first point (start)
        let (blurred_lat, blurred_lon) = self.blur_point(
            coordinates[0][1], // lat
            coordinates[0][0], // lon
            privacy_zones,
        );
        result[0] = [blurred_lon, blurred_lat];

        // Blur last point (end) if different from start
        if coordinates.len() > 1 {
            let last_idx = coordinates.len() - 1;
            let (blurred_lat, blurred_lon) = self.blur_point(
                coordinates[last_idx][1], // lat
                coordinates[last_idx][0], // lon
                privacy_zones,
            );
            result[last_idx] = [blurred_lon, blurred_lat];
        }

        result
    }

    /// Calculate distance between two points using Haversine formula
    fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
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

    /// Move a point to the edge of a circle
    fn move_to_edge(
        point_lat: f64,
        point_lon: f64,
        center_lat: f64,
        center_lon: f64,
        radius: f64,
    ) -> (f64, f64) {
        let distance = Self::haversine_distance(point_lat, point_lon, center_lat, center_lon);

        if distance == 0.0 {
            // Point is at center, move north
            return Self::move_direction(center_lat, center_lon, radius, 0.0);
        }

        // Calculate bearing from center to point
        let d_lon = (point_lon - center_lon).to_radians();
        let lat1 = center_lat.to_radians();
        let lat2 = point_lat.to_radians();
        let y = d_lon.sin() * lat2.cos();
        let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos();
        let bearing = y.atan2(x).to_degrees();

        // Move to edge in the same direction
        Self::move_direction(center_lat, center_lon, radius, bearing)
    }

    /// Move a point in a given direction (bearing in degrees)
    fn move_direction(lat: f64, lon: f64, distance: f64, bearing: f64) -> (f64, f64) {
        let r = 6_371_000.0; // Earth radius in meters
        let lat1 = lat.to_radians();
        let lon1 = lon.to_radians();
        let bearing_rad = bearing.to_radians();

        let lat2 = (lat1.sin() * (distance / r).cos()
            + lat1.cos() * (distance / r).sin() * bearing_rad.cos())
        .asin();

        let lon2 = lon1
            + (bearing_rad.sin() * (distance / r).sin() * lat1.cos())
                .atan2((distance / r).cos() - lat1.sin() * lat2.sin());

        (lat2.to_degrees(), lon2.to_degrees())
    }
}

impl Default for PrivacyService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_privacy_zone() {
        let service = PrivacyService::new();
        let zones = vec![PrivacyZone {
            id: "zone-1".to_string(),
            user_id: "user-1".to_string(),
            name: "Home".to_string(),
            latitude: 52.52,
            longitude: 13.405,
            radius_meters: 200.0,
        }];

        // Inside zone
        assert!(service.is_in_privacy_zone(52.52, 13.405, &zones));

        // Outside zone
        assert!(!service.is_in_privacy_zone(52.53, 13.405, &zones));
    }

    #[test]
    fn test_blur_point_inside_zone() {
        let service = PrivacyService::new();
        let zones = vec![PrivacyZone {
            id: "zone-1".to_string(),
            user_id: "user-1".to_string(),
            name: "Home".to_string(),
            latitude: 52.52,
            longitude: 13.405,
            radius_meters: 200.0,
        }];

        // Point inside zone should be blurred
        let (blurred_lat, blurred_lon) = service.blur_point(52.52, 13.405, &zones);

        // Blurred point should be at the edge of the zone
        let distance = PrivacyService::haversine_distance(blurred_lat, blurred_lon, 52.52, 13.405);
        assert!((distance - 200.0).abs() < 1.0); // Within 1 meter
    }

    #[test]
    fn test_blur_point_outside_zone() {
        let service = PrivacyService::new();
        let zones = vec![PrivacyZone {
            id: "zone-1".to_string(),
            user_id: "user-1".to_string(),
            name: "Home".to_string(),
            latitude: 52.52,
            longitude: 13.405,
            radius_meters: 200.0,
        }];

        // Point outside zone should not be blurred
        let (lat, lon) = service.blur_point(52.53, 13.405, &zones);
        assert_eq!(lat, 52.53);
        assert_eq!(lon, 13.405);
    }

    #[test]
    fn test_blur_route_endpoints() {
        let service = PrivacyService::new();
        let zones = vec![PrivacyZone {
            id: "zone-1".to_string(),
            user_id: "user-1".to_string(),
            name: "Home".to_string(),
            latitude: 52.52,
            longitude: 13.405,
            radius_meters: 200.0,
        }];

        let coordinates = vec![
            [13.405, 52.52],  // Start (inside zone)
            [13.406, 52.521], // Middle (outside zone)
            [13.407, 52.522], // End (outside zone)
        ];

        let blurred = service.blur_route_endpoints(&coordinates, &zones);

        // Start should be blurred
        let start_distance =
            PrivacyService::haversine_distance(blurred[0][1], blurred[0][0], 52.52, 13.405);
        assert!((start_distance - 200.0).abs() < 1.0);

        // Middle should not change
        assert_eq!(blurred[1], [13.406, 52.521]);

        // End should not change (outside zone)
        assert_eq!(blurred[2], [13.407, 52.522]);
    }
}
