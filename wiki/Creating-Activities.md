# Creating Activities

Lièvre supports multiple ways to create activities: manual entry, GPX upload, TCX upload, and Strava import.

## Manual Entry

1. Click **+ New** in the navigation bar
2. Fill in the form:
   - **Type** — Ride, Run, Swim, Walk, or Hike
   - **Title** — Give your activity a name
   - **Start Time** — When you started
   - **Duration** — Total time in seconds
   - **Distance** — Total distance in meters
   - **Elevation Gain** — Total climbing in meters
   - **Visibility** — Public, Followers, or Private
3. Click **Create Activity**

## GPX Upload

GPX (GPS Exchange Format) is the most common format for GPS devices.

### Supported Devices

- Garmin (all models)
- Polar
- Suunto
- Wahoo
- Any device that exports GPX

### How to Upload

1. Export GPX from your device or app
2. Click **+ New** in Lièvre
3. Drag and drop the `.gpx` file onto the upload area
4. Click **Upload**
5. Lièvre extracts:
   - Route coordinates
   - Start/end timestamps
   - Total distance
   - Elevation profile

### GPX Structure

Lièvre expects standard GPX with `<trk>` and `<trkseg>` elements:

```xml
<gpx>
  <trk>
    <name>Morning Ride</name>
    <trkseg>
      <trkpt lat="52.5200" lon="13.4050">
        <ele>34.0</ele>
        <time>2026-08-14T08:00:00Z</time>
      </trkpt>
      <!-- more track points -->
    </trkseg>
  </trk>
</gpx>
```

## TCX Upload

TCX (Training Center XML) is Garmin's format with more detailed data.

### How to Upload

1. Export TCX from Garmin Connect or your device
2. Click **+ New** in Lièvre
3. Drag and drop the `.tcx` file
4. Click **Upload**

TCX includes heart rate, cadence, and power data when available.

## Strava Export Import

If you're migrating from Strava:

1. Go to Strava → Settings → My Account → Request Your Archive
2. Download the ZIP file
3. In Lièvre, click **+ New**
4. Upload the Strava ZIP
5. Lièvre imports all activities with routes and stats

### What Gets Imported

- Activity type, title, description
- Start time, duration, distance
- Route coordinates (from attached GPX files)
- Elevation data

## Activity Types

| Type | Description | Examples |
|------|-------------|----------|
| **ride** | Road cycling | Road bike, racing |
| **run** | Running | Road run, track |
| **swim** | Swimming | Pool, open water |
| **walk** | Walking | Casual, hiking |
| **hike** | Hiking | Mountain, trail |

## Visibility Settings

| Setting | Who Can See | Federated |
|---------|-------------|-----------|
| **Public** | Everyone | ✅ Yes |
| **Followers** | Only your followers | ✅ Yes (followers only) |
| **Private** | Only you | ❌ No |

## After Creation

Once created, your activity:

- Appears in your profile
- Shows in your followers' feeds (if public/followers)
- Gets federated to remote instances (if public/followers)
- Can be liked and commented on
- Shows on a Leaflet map (if route data available)

---

**See also:** [Following & Social](Following-Social.md) | [Your Feed](Your-Feed.md)
