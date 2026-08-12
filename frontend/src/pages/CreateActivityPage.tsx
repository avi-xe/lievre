import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { apiFetch, apiUpload } from '../lib/api';

export function CreateActivityPage() {
  const navigate = useNavigate();
  const [form, setForm] = useState({
    activity_type: 'ride',
    title: '',
    started_at: '',
    duration_seconds: '',
    distance_meters: '',
    elevation_gain_meters: '',
    visibility: 'public',
  });
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [file, setFile] = useState<File | null>(null);
  const [uploading, setUploading] = useState(false);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setForm({ ...form, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const res = await apiFetch<{ id: string }>('/api/activities', {
        method: 'POST',
        body: JSON.stringify({
          ...form,
          duration_seconds: form.duration_seconds ? parseInt(form.duration_seconds) : null,
          distance_meters: form.distance_meters ? parseFloat(form.distance_meters) : null,
          elevation_gain_meters: form.elevation_gain_meters ? parseFloat(form.elevation_gain_meters) : null,
        }),
      });
      navigate(`/activities/${res.id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create activity');
    } finally {
      setLoading(false);
    }
  };

  const handleFileUpload = async () => {
    if (!file) return;
    setUploading(true);
    setError('');
    try {
      const res = await apiUpload<{ activity_id: string }>('/api/import/gpx', file);
      navigate(`/activities/${res.activity_id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Upload failed');
    } finally {
      setUploading(false);
    }
  };

  const inputStyle = { width: '100%', padding: 8, boxSizing: 'border-box' as const };

  return (
    <div style={{ maxWidth: 600, margin: '0 auto', padding: 20 }}>
      <h2>New Activity</h2>

      <h3>Upload GPX File</h3>
      <div style={{ marginBottom: 20, padding: 20, border: '2px dashed #ccc', textAlign: 'center' }}>
        <input type="file" accept=".gpx" onChange={e => setFile(e.target.files?.[0] || null)} />
        {file && (
          <div style={{ marginTop: 10 }}>
            <span>{file.name}</span>
            <button onClick={handleFileUpload} disabled={uploading} style={{ marginLeft: 10 }}>
              {uploading ? 'Uploading...' : 'Upload'}
            </button>
          </div>
        )}
      </div>

      <h3>Or Enter Manually</h3>
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
          </select>
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Title</label>
          <input name="title" value={form.title} onChange={handleChange} required style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Start Time</label>
          <input name="started_at" type="datetime-local" value={form.started_at} onChange={handleChange} required style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Duration (seconds)</label>
          <input name="duration_seconds" type="number" value={form.duration_seconds} onChange={handleChange} style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Distance (meters)</label>
          <input name="distance_meters" type="number" step="0.1" value={form.distance_meters} onChange={handleChange} style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Elevation Gain (meters)</label>
          <input name="elevation_gain_meters" type="number" step="0.1" value={form.elevation_gain_meters} onChange={handleChange} style={inputStyle} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <label>Visibility</label>
          <select name="visibility" value={form.visibility} onChange={handleChange} style={inputStyle}>
            <option value="public">Public</option>
            <option value="followers">Followers</option>
            <option value="private">Private</option>
          </select>
        </div>
        <button type="submit" disabled={loading} style={{ width: '100%', padding: 10 }}>
          {loading ? 'Creating...' : 'Create Activity'}
        </button>
      </form>
    </div>
  );
}
