import { useEffect, useState, useCallback } from 'react';
import { apiFetch } from '../lib/api';

interface Notification {
  id: string;
  type: string;
  actor_id: string;
  actor_username: string;
  content: string;
  read: boolean;
  created_at: string;
}

function timeAgo(dateStr: string): string {
  const seconds = Math.floor((Date.now() - new Date(dateStr).getTime()) / 1000);
  if (seconds < 60) return 'just now';
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}d ago`;
  return new Date(dateStr).toLocaleDateString();
}

function typeIcon(type: string): string {
  switch (type) {
    case 'follow': return '👤';
    case 'like': return '♥';
    case 'comment': return '💬';
    default: return '•';
  }
}

export function NotificationsPage() {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [markingAll, setMarkingAll] = useState(false);

  const fetchNotifications = useCallback(async () => {
    try {
      const data = await apiFetch<Notification[]>('/api/notifications');
      setNotifications(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load notifications');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { fetchNotifications(); }, [fetchNotifications]);

  const markAllRead = async () => {
    setMarkingAll(true);
    try {
      await apiFetch('/api/notifications/read-all', { method: 'PUT' });
      await fetchNotifications();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to mark all read');
    } finally {
      setMarkingAll(false);
    }
  };

  const markRead = async (notifId: string) => {
    try {
      await apiFetch(`/api/notifications/${notifId}/read`, { method: 'PUT' });
      await fetchNotifications();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to mark read');
    }
  };

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;
  if (error) return <div style={{ padding: 20, color: 'red' }}>{error}</div>;

  return (
    <div style={{ maxWidth: 600, margin: '0 auto', padding: 20 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 20 }}>
        <h2>Notifications</h2>
        {notifications.some(n => !n.read) && (
          <button onClick={markAllRead} disabled={markingAll} style={{ padding: '6px 12px' }}>
            {markingAll ? 'Marking...' : 'Mark all read'}
          </button>
        )}
      </div>
      {notifications.length === 0 ? (
        <p style={{ color: '#666' }}>No notifications yet</p>
      ) : (
        <div>
          {notifications.map(n => (
            <div
              key={n.id}
              style={{
                padding: '12px 0',
                borderBottom: '1px solid #eee',
                fontWeight: n.read ? 'normal' : 'bold',
                display: 'flex',
                alignItems: 'flex-start',
                gap: 10,
              }}
            >
              <span style={{ fontSize: 18 }}>{typeIcon(n.type)}</span>
              <div style={{ flex: 1 }}>
                <div>
                  <strong>{n.actor_username}</strong> {n.type === 'follow' && 'followed you'}
                  {n.type === 'like' && ' liked your activity'}
                  {n.type === 'comment' && ' commented on your activity'}
                </div>
                {n.content && (
                  <div style={{ color: '#666', fontSize: 14, marginTop: 2 }}>{n.content}</div>
                )}
                <div style={{ color: '#999', fontSize: 12, marginTop: 4 }}>
                  {timeAgo(n.created_at)}
                </div>
              </div>
              {!n.read && (
                <button
                  onClick={() => markRead(n.id)}
                  style={{ background: 'none', border: 'none', cursor: 'pointer', fontSize: 12, color: '#666' }}
                  title="Mark as read"
                >
                  ✓
                </button>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
