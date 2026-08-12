import { BrowserRouter, Routes, Route, Link } from 'react-router-dom';
import { AuthProvider } from './contexts/AuthContext';
import { useAuth } from './contexts/useAuth';
import { ProtectedRoute } from './components/ProtectedRoute';
import { LoginPage } from './pages/LoginPage';
import { RegisterPage } from './pages/RegisterPage';
import { ActivityListPage } from './pages/ActivityListPage';
import { ActivityDetailPage } from './pages/ActivityDetailPage';
import { CreateActivityPage } from './pages/CreateActivityPage';
import { FeedPage } from './pages/FeedPage';
import { ProfilePage } from './pages/ProfilePage';

function NavBar() {
  const { isAuthenticated, logout } = useAuth();
  return (
    <nav style={{ display: 'flex', alignItems: 'center', gap: 16, padding: '12px 20px', borderBottom: '1px solid #eee' }}>
      <Link to="/" style={{ fontWeight: 'bold', fontSize: 18, textDecoration: 'none', color: '#333' }}>Lièvre</Link>
      <Link to="/feed">Feed</Link>
      <Link to="/">Activities</Link>
      {isAuthenticated && <Link to="/activities/new">+ New</Link>}
      <div style={{ flex: 1 }} />
      {isAuthenticated ? (
        <button onClick={logout} style={{ background: 'none', border: 'none', cursor: 'pointer' }}>Logout</button>
      ) : (
        <Link to="/login">Login</Link>
      )}
    </nav>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <NavBar />
        <Routes>
          <Route path="/login" element={<LoginPage />} />
          <Route path="/register" element={<RegisterPage />} />
          <Route path="/feed" element={<FeedPage />} />
          <Route element={<ProtectedRoute />}>
            <Route path="/" element={<ActivityListPage />} />
            <Route path="/activities/new" element={<CreateActivityPage />} />
            <Route path="/activities/:id" element={<ActivityDetailPage />} />
            <Route path="/users/:id" element={<ProfilePage />} />
          </Route>
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  );
}

export default App;
