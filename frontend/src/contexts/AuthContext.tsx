import { useState, useEffect, useCallback, type ReactNode } from 'react';
import { apiFetch } from '../lib/api';
import type { User } from '../lib/types';
import { AuthContext } from './auth-state';

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(() => localStorage.getItem('auth_token'));
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!token) {
      setLoading(false);
      return;
    }
    apiFetch<User>('/api/users/me')
      .then(setUser)
      .catch(() => {
        localStorage.removeItem('auth_token');
        setToken(null);
        setUser(null);
      })
      .finally(() => setLoading(false));
  }, [token]);

  const login = useCallback(async (email: string, password: string) => {
    const res = await apiFetch<{ token: string; user: User }>('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
    localStorage.setItem('auth_token', res.token);
    setToken(res.token);
    setUser(res.user);
  }, []);

  const register = useCallback(async (email: string, username: string, password: string) => {
    const res = await apiFetch<{ token: string; user: User }>('/api/auth/register', {
      method: 'POST',
      body: JSON.stringify({ email, username, password }),
    });
    localStorage.setItem('auth_token', res.token);
    setToken(res.token);
    setUser(res.user);
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem('auth_token');
    setToken(null);
    setUser(null);
  }, []);

  if (loading) return <div style={{ padding: 20 }}>Loading...</div>;

  return (
    <AuthContext.Provider value={{ user, token, login, register, logout, isAuthenticated: !!user }}>
      {children}
    </AuthContext.Provider>
  );
}
