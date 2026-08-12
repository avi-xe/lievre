import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { apiFetch } from '../lib/api';
import { useAuth } from '../contexts/useAuth';
import { ActivityMap } from '../components/ActivityMap';
import type { Activity, Comment } from '../lib/types';

export function ActivityDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { user } = useAuth();
  const navigate = useNavigate();
  const [activity, setActivity] = useState<Activity | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [commentText, setCommentText] = useState('');
  const [liked, setLiked] = useState(false);
  const [likeCount, setLikeCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    if (!id) return;
    Promise.all([
      apiFetch<Activity>(`/api/activities/${id}`),
      apiFetch<Comment[]>(`/api/activities/${id}/comments`).catch(() => []),
    ])
      .then(([act, cmts]) => {
        setActivity(act);
        setComments(cmts);
      })
      .catch(err => setError(err.message))
      .finally(() => setLoading(false));
  }, [id]);

  const handleDelete = async () => {
    if (!id || !confirm('Delete this activity?')) return;
    await apiFetch(`/api/activities/${id}`, { method: 'DELETE' });
    navigate('/');
  };

  const handleLike = async () => {
    if (!id) return;
    if (liked) {
      await apiFetch(`/api/activities/${id}/like`, { method: 'DELETE' });
      setLiked(false);
      setLikeCount(c => c - 1);
    } else {
      await apiFetch(`/api/activities/${id}/like`, { method: 'POST' });
      setLiked(true);
      setLikeCount(c => c + 1);
    }
  };

  const handleComment = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!id || !commentText.trim()) return;
    const comment = await apiFetch<Comment>(`/api/activities/${id}/comments`, {
      method: 'POST',
      body: JSON.stringify({ content: commentText }),
    });
    setComments([...comments, comment]);
    setCommentText('');
  };

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;
  if (error) return <div style={{ padding: 20, color: 'red' }}>{error}</div>;
  if (!activity) return <div style={{ padding: 20 }}>Activity not found</div>;

  const isOwner = user?.id === activity.user_id;

  return (
    <div style={{ maxWidth: 800, margin: '0 auto', padding: 20 }}>
      <h2>{activity.title}</h2>
      <p style={{ color: '#666' }}>
        {activity.activity_type} · {new Date(activity.started_at).toLocaleString()}
      </p>

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 16, margin: '20px 0' }}>
        <div>
          <strong>Distance</strong>
          <div>{activity.distance_meters != null ? `${(activity.distance_meters / 1000).toFixed(1)} km` : '-'}</div>
        </div>
        <div>
          <strong>Duration</strong>
          <div>{activity.duration_seconds != null ? `${Math.floor(activity.duration_seconds / 3600)}h ${Math.floor((activity.duration_seconds % 3600) / 60)}m` : '-'}</div>
        </div>
        <div>
          <strong>Elevation</strong>
          <div>{activity.elevation_gain_meters != null ? `${Math.round(activity.elevation_gain_meters)} m` : '-'}</div>
        </div>
      </div>

      {id && <ActivityMap activityId={id} />}

      <div style={{ margin: '20px 0', display: 'flex', gap: 10, alignItems: 'center' }}>
        <button onClick={handleLike}>
          {liked ? '♥ Liked' : '♡ Like'} {likeCount > 0 && `(${likeCount})`}
        </button>
        {isOwner && (
          <button onClick={handleDelete} style={{ color: 'red' }}>Delete</button>
        )}
      </div>

      <h3>Comments</h3>
      <div style={{ marginBottom: 10 }}>
        {comments.map(c => (
          <div key={c.id} style={{ padding: '8px 0', borderBottom: '1px solid #eee' }}>
            <strong>{c.username}</strong> · {new Date(c.created_at).toLocaleString()}
            <p>{c.content}</p>
          </div>
        ))}
      </div>
      <form onSubmit={handleComment} style={{ display: 'flex', gap: 8 }}>
        <input
          type="text"
          value={commentText}
          onChange={e => setCommentText(e.target.value)}
          placeholder="Add a comment..."
          style={{ flex: 1, padding: 8 }}
        />
        <button type="submit">Post</button>
      </form>
    </div>
  );
}
