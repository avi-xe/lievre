import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { apiFetch } from '../lib/api';
import type { User } from '../lib/types';

export function UsersPage() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    apiFetch<User[]>('/api/users')
      .then(setUsers)
      .catch(() => setUsers([]))
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;

  return (
    <div style={{ maxWidth: 800, margin: '0 auto', padding: 20 }}>
      <h2>Users</h2>
      {users.length === 0 ? (
        <p>No users found.</p>
      ) : (
        users.map(u => (
          <div key={u.id} style={{ padding: '12px 0', borderBottom: '1px solid #eee' }}>
            <Link to={`/users/${u.id}`} style={{ fontSize: 16 }}>
              <strong>{u.username}</strong>
            </Link>
            <span style={{ color: '#666', marginLeft: 8 }}>{u.email}</span>
          </div>
        ))
      )}
    </div>
  );
}
