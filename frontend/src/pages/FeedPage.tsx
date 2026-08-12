import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { apiFetch } from '../lib/api';
import { useAuth } from '../contexts/useAuth';
import type { FeedItem } from '../lib/types';

export function FeedPage() {
  const { isAuthenticated } = useAuth();
  const [items, setItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const endpoint = isAuthenticated ? '/api/feed' : '/api/feed/public';
    apiFetch<FeedItem[]>(endpoint)
      .then(setItems)
      .catch(() => setItems([]))
      .finally(() => setLoading(false));
  }, [isAuthenticated]);

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;

  return (
    <div style={{ maxWidth: 800, margin: '0 auto', padding: 20 }}>
      <h2>{isAuthenticated ? 'Feed' : 'Public Feed'}</h2>
      {items.length === 0 ? (
        <p>No activities yet.</p>
      ) : (
        items.map(item => (
          <div key={item.id} style={{ padding: '12px 0', borderBottom: '1px solid #eee' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between' }}>
              <div>
                {item.username && <Link to={`/users/${item.user_id}`}><strong>{item.username}</strong></Link>}
                {' · '}
                <Link to={`/activities/${item.id}`}>{item.title}</Link>
              </div>
              <span style={{ color: '#666' }}>{new Date(item.started_at).toLocaleDateString()}</span>
            </div>
            <div style={{ color: '#666', fontSize: 14, marginTop: 4 }}>
              {item.activity_type}
              {item.distance_meters != null && ` · ${(item.distance_meters / 1000).toFixed(1)} km`}
              {item.duration_seconds != null && ` · ${Math.floor(item.duration_seconds / 3600)}h ${Math.floor((item.duration_seconds % 3600) / 60)}m`}
            </div>
          </div>
        ))
      )}
    </div>
  );
}
