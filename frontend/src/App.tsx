import { useState } from 'react';
import { ActivityMap } from './components/ActivityMap';
import { ElevationProfile } from './components/ElevationProfile';

function App() {
  const [activityId, setActivityId] = useState('');

  return (
    <div style={{ padding: '20px', maxWidth: '1200px', margin: '0 auto' }}>
      <h1 style={{ marginBottom: '20px' }}>Lièvre — Activity Viewer</h1>

      <div style={{ marginBottom: '20px' }}>
        <label htmlFor="activityId" style={{ display: 'block', marginBottom: '5px' }}>
          Activity ID:
        </label>
        <input
          id="activityId"
          type="text"
          value={activityId}
          onChange={(e) => setActivityId(e.target.value)}
          placeholder="Enter activity ID"
          style={{
            padding: '8px',
            width: '300px',
            border: '1px solid #ccc',
            borderRadius: '4px',
          }}
        />
      </div>

      {activityId && (
        <>
          <div style={{ marginBottom: '20px' }}>
            <h2>Route Map</h2>
            <ActivityMap activityId={activityId} height="400px" />
          </div>

          <div>
            <h2>Elevation Profile</h2>
            <ElevationProfile activityId={activityId} height="200px" />
          </div>
        </>
      )}
    </div>
  );
}

export default App;
