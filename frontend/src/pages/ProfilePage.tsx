import { useEffect, useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { apiFetch } from '../lib/api';
import { useAuth } from '../contexts/useAuth';
import type { Activity, User } from '../lib/types';

export function ProfilePage() {
  const { id } = useParams<{ id: string }>();
  const { user: currentUser } = useAuth();
  const [profile, setProfile] = useState<User | null>(null);
  const [activities, setActivities] = useState<Activity[]>([]);
  const [isFollowing, setIsFollowing] = useState(false);
  const [loading, setLoading] = useState(true);

  const isOwn = currentUser?.id === id;

  useEffect(() => {
    if (!id) return;
    apiFetch<User>(`/api/users/${id}`)
      .then(setProfile)
      .catch(() => {})
      .finally(() => setLoading(false));
    // Fetch activities - use the list endpoint and filter
    apiFetch<Activity[]>('/api/activities')
      .then(acts => setActivities(acts.filter(a => a.user_id === id)))
      .catch(() => {});
    // Fetch follow status if viewing another user's profile
    if (currentUser && currentUser.id !== id) {
      apiFetch<{ is_following: boolean }>(`/api/users/${id}/follow-status`)
        .then(data => setIsFollowing(data.is_following))
        .catch(() => {});
    }
  }, [id, currentUser]);

  const handleFollow = async () => {
    if (!id) return;
    if (isFollowing) {
      await apiFetch(`/api/users/${id}/follow`, { method: 'DELETE' });
      setIsFollowing(false);
    } else {
      await apiFetch(`/api/users/${id}/follow`, { method: 'POST' });
      setIsFollowing(true);
    }
  };

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;
  if (!profile) return <div style={{ padding: 20 }}>User not found</div>;

  return (
    <div style={{ maxWidth: 800, margin: '0 auto', padding: 20 }}>
      <h2>{profile.username}</h2>
      <p style={{ color: '#666' }}>{profile.email}</p>

      {!isOwn && (
        <button onClick={handleFollow} style={{ marginBottom: 20 }}>
          {isFollowing ? 'Unfollow' : 'Follow'}
        </button>
      )}

      <h3>Activities ({activities.length})</h3>
      {activities.map(a => (
        <div key={a.id} style={{ padding: '8px 0', borderBottom: '1px solid #eee' }}>
          <Link to={`/activities/${a.id}`}>{a.title}</Link>
          <span style={{ color: '#666', marginLeft: 8 }}>
            {a.activity_type} · {new Date(a.started_at).toLocaleDateString()}
            {a.distance_meters != null && ` · ${(a.distance_meters / 1000).toFixed(1)} km`}
          </span>
        </div>
      ))}
    </div>
  );
}
