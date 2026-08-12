import { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../contexts/useAuth';

export function RegisterPage() {
  const { register } = useAuth();
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      await register(email, username, password);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Registration failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ maxWidth: 400, margin: '40px auto', padding: 20 }}>
      <h2>Register</h2>
      {error && <div style={{ color: 'red', marginBottom: 10 }}>{error}</div>}
      <form onSubmit={handleSubmit}>
        <div style={{ marginBottom: 10 }}>
          <input type="email" placeholder="Email" value={email} onChange={e => setEmail(e.target.value)} required
            style={{ width: '100%', padding: 8, boxSizing: 'border-box' }} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <input type="text" placeholder="Username" value={username} onChange={e => setUsername(e.target.value)} required
            style={{ width: '100%', padding: 8, boxSizing: 'border-box' }} />
        </div>
        <div style={{ marginBottom: 10 }}>
          <input type="password" placeholder="Password" value={password} onChange={e => setPassword(e.target.value)} required
            style={{ width: '100%', padding: 8, boxSizing: 'border-box' }} />
        </div>
        <button type="submit" disabled={loading}
          style={{ width: '100%', padding: 10 }}>
          {loading ? 'Registering...' : 'Register'}
        </button>
      </form>
      <p style={{ textAlign: 'center', marginTop: 10 }}>
        Already have an account? <Link to="/login">Login</Link>
      </p>
    </div>
  );
}
