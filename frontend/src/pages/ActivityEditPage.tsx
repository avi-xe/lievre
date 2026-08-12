import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { apiFetch } from '../lib/api';
import type { Activity } from '../lib/types';

export function ActivityEditPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [form, setForm] = useState({
    activity_type: 'ride',
    title: '',
    description: '',
    started_at: '',
    visibility: 'public',
  });
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (!id) return;
    apiFetch<Activity>(`/api/activities/${id}`)
      .then(act => {
        setForm({
          activity_type: act.activity_type,
          title: act.title,
          description: act.description ?? '',
          started_at: act.started_at ? act.started_at.slice(0, 16) : '',
          visibility: act.visibility,
        });
      })
      .catch(err => setError(err.message))
      .finally(() => setLoading(false));
  }, [id]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    setForm({ ...form, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!id) return;
    setError('');
    setSaving(true);
    try {
      await apiFetch(`/api/activities/${id}`, {
        method: 'PUT',
        body: JSON.stringify({
          ...form,
          started_at: form.started_at ? `${form.started_at}:00Z` : form.started_at,
        }),
      });
      navigate(`/activities/${id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update activity');
    } finally {
      setSaving(false);
    }
  };

  const inputStyle = { width: '100%', padding: 8, boxSizing: 'border-box' as const };

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;
  if (error && !saving) return <div style={{ padding: 20, color: 'red' }}>{error}</div>;

  return (
    <div style={{ maxWidth: 600, margin: '0 auto', padding: 20 }}>
      <h2>Edit Activity</h2>
      {error && <div style={{ color: 'red', marginBottom: 10 }}>{error}</div>}
      <form onSubmit={handleSubmit}>
        <div style={{ marginBottom: 10 }}>
          <label>Type</label>
          <select name="activity_type" value={form.activity_type} onChange={handleChange} style={inputStyle}>
            <option value="ride">Ride</option>
            <option value="run">Run</option>
            <option value="swim">Swim</option>
            <option value="walk">Walk</option>
            <option value="hike">Hike</option>
            <option value="virtual-ride">Virtual Ride</option>
          </select>
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Title</label>
          <input name="title" value={form.title} onChange={handleChange} required style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Description</label>
          <textarea name="description" value={form.description} onChange={handleChange} rows={3} style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Start Time</label>
          <input name="started_at" type="datetime-local" value={form.started_at} onChange={handleChange} required style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Visibility</label>
          <select name="visibility" value={form.visibility} onChange={handleChange} style={inputStyle}>
            <option value="public">Public</option>
            <option value="followers">Followers</option>
            <option value="private">Private</option>
          </select>
        </div>
        <div style={{ display: 'flex', gap: 10 }}>
          <button type="submit" disabled={saving} style={{ flex: 1, padding: 10 }}>
            {saving ? 'Saving...' : 'Save Changes'}
          </button>
          <button type="button" onClick={() => navigate(`/activities/${id}`)} style={{ flex: 1, padding: 10 }}>
            Cancel
          </button>
        </div>
      </form>
    </div>
  );
}
