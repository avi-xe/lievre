import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { apiFetch } from '../lib/api';
import type { Activity } from '../lib/types';

function formatDuration(seconds: number | null): string {
  if (seconds == null) return '-';
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m ${s}s`;
}

function formatDistance(meters: number | null): string {
  if (meters == null) return '-';
  return `${(meters / 1000).toFixed(1)} km`;
}

export function ActivityListPage() {
  const [activities, setActivities] = useState<Activity[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    apiFetch<Activity[]>('/api/activities')
      .then(setActivities)
      .catch(err => setError(err.message))
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;
  if (error) return <div style={{ padding: 20, color: 'red' }}>{error}</div>;

  return (
    <div style={{ maxWidth: 800, margin: '0 auto', padding: 20 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 20 }}>
        <h2>My Activities</h2>
        <Link to="/activities/new" style={{ padding: '8px 16px', background: '#333', color: 'white', textDecoration: 'none', borderRadius: 4 }}>
          + New Activity
        </Link>
      </div>
      {activities.length === 0 ? (
        <p>No activities yet. <Link to="/activities/new">Create your first one</Link></p>
      ) : (
        <table style={{ width: '100%', borderCollapse: 'collapse' }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #333' }}>
              <th style={{ textAlign: 'left', padding: 8 }}>Title</th>
              <th style={{ textAlign: 'left', padding: 8 }}>Type</th>
              <th style={{ textAlign: 'left', padding: 8 }}>Date</th>
              <th style={{ textAlign: 'right', padding: 8 }}>Distance</th>
              <th style={{ textAlign: 'right', padding: 8 }}>Duration</th>
            </tr>
          </thead>
          <tbody>
            {activities.map(a => (
              <tr key={a.id} style={{ borderBottom: '1px solid #eee' }}>
                <td style={{ padding: 8 }}><Link to={`/activities/${a.id}`}>{a.title}</Link></td>
                <td style={{ padding: 8 }}>{a.activity_type}</td>
                <td style={{ padding: 8 }}>{new Date(a.started_at).toLocaleDateString()}</td>
                <td style={{ padding: 8, textAlign: 'right' }}>{formatDistance(a.distance_meters)}</td>
                <td style={{ padding: 8, textAlign: 'right' }}>{formatDuration(a.duration_seconds)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
