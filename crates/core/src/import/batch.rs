use crate::activity::Activity;
use crate::route::Route;
use super::{gpx::GpxParser, fit::FitParser, tcx::TcxParser};

#[derive(Debug, Clone)]
pub struct BatchResult {
    pub imported: Vec<ImportedActivity>,
    pub errors: Vec<ImportError>,
}

#[derive(Debug, Clone)]
pub struct ImportedActivity {
    pub filename: String,
    pub activity: Activity,
    pub route: Option<Route>,
}

#[derive(Debug, Clone)]
pub struct ImportError {
    pub filename: String,
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct BatchImporter {
    gpx_parser: GpxParser,
    fit_parser: FitParser,
    tcx_parser: TcxParser,
}

impl BatchImporter {
    pub fn new() -> Self {
        Self {
            gpx_parser: GpxParser::new(),
            fit_parser: FitParser::new(),
            tcx_parser: TcxParser::new(),
        }
    }

    pub fn detect_format(&self, filename: &str) -> Option<&str> {
        let lower = filename.to_lowercase();
        if lower.ends_with(".gpx") {
            Some("gpx")
        } else if lower.ends_with(".fit") {
            Some("fit")
        } else if lower.ends_with(".tcx") {
            Some("tcx")
        } else {
            None
        }
    }

    pub fn parse_file(&self, filename: &str, content: &[u8]) -> anyhow::Result<ParsedActivity> {
        let format = self.detect_format(filename)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {}", filename))?;

        match format {
            "gpx" => {
                let gpx_content = String::from_utf8(content.to_vec())
                    .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))?;
                let track = self.gpx_parser.parse(&gpx_content)?;
                Ok(ParsedActivity::Gpx(track))
            }
            "fit" => {
                let session = self.fit_parser.parse(content)?;
                Ok(ParsedActivity::Fit(session))
            }
            "tcx" => {
                let tcx_content = String::from_utf8(content.to_vec())
                    .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))?;
                let activity = self.tcx_parser.parse(&tcx_content)?;
                Ok(ParsedActivity::Tcx(activity))
            }
            _ => unreachable!(),
        }
    }
}

impl Default for BatchImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ParsedActivity {
    Gpx(super::gpx::GpxTrack),
    Fit(super::fit::FitSession),
    Tcx(super::tcx::TcxActivity),
}

impl ParsedActivity {
    pub fn to_create_activity(&self) -> crate::activity::CreateActivity {
        match self {
            ParsedActivity::Gpx(track) => self.gpx_to_create(track),
            ParsedActivity::Fit(session) => self.fit_to_create(session),
            ParsedActivity::Tcx(activity) => self.tcx_to_create(activity),
        }
    }

    fn gpx_to_create(&self, track: &super::gpx::GpxTrack) -> crate::activity::CreateActivity {
        crate::activity::CreateActivity {
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

    fn fit_to_create(&self, session: &super::fit::FitSession) -> crate::activity::CreateActivity {
        crate::activity::CreateActivity {
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

    fn tcx_to_create(&self, activity: &super::tcx::TcxActivity) -> crate::activity::CreateActivity {
        crate::activity::CreateActivity {
            activity_type: activity.activity_type.clone(),
            title: activity.name.clone(),
            description: None,
            started_at: activity.start_time,
            duration_seconds: activity.duration_seconds,
            distance_meters: activity.distance_meters,
            elevation_gain_meters: activity.elevation_gain_meters,
            visibility: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_gpx() {
        let importer = BatchImporter::new();
        assert_eq!(importer.detect_format("activity.gpx"), Some("gpx"));
        assert_eq!(importer.detect_format("ACTIVITY.GPX"), Some("gpx"));
    }

    #[test]
    fn test_detect_format_fit() {
        let importer = BatchImporter::new();
        assert_eq!(importer.detect_format("activity.fit"), Some("fit"));
        assert_eq!(importer.detect_format("ACTIVITY.FIT"), Some("fit"));
    }

    #[test]
    fn test_detect_format_tcx() {
        let importer = BatchImporter::new();
        assert_eq!(importer.detect_format("activity.tcx"), Some("tcx"));
        assert_eq!(importer.detect_format("ACTIVITY.TCX"), Some("tcx"));
    }

    #[test]
    fn test_detect_format_unknown() {
        let importer = BatchImporter::new();
        assert_eq!(importer.detect_format("activity.txt"), None);
        assert_eq!(importer.detect_format("activity.json"), None);
    }

    #[test]
    fn test_parsed_activity_to_create() {
        let gpx_track = super::super::gpx::GpxTrack {
            name: Some("Test Ride".to_string()),
            activity_type: crate::activity::ActivityType::Ride,
            start_time: chrono::Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            coordinates: vec![],
            elevation_data: vec![],
        };

        let parsed = ParsedActivity::Gpx(gpx_track);
        let create = parsed.to_create_activity();
        
        assert_eq!(create.activity_type, crate::activity::ActivityType::Ride);
        assert_eq!(create.title, Some("Test Ride".to_string()));
        assert_eq!(create.duration_seconds, Some(3600));
    }
}
