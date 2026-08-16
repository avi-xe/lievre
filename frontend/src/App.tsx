import { BrowserRouter, Routes, Route } from "react-router-dom";
import { AuthProvider } from "./contexts/AuthContext";
import { ProtectedRoute } from "./components/ProtectedRoute";
import { Layout } from "./components/layout";
import { TooltipProvider } from "@/components/ui/tooltip";

// Pages
import { LoginPage } from "./pages/LoginPage";
import { RegisterPage } from "./pages/RegisterPage";
import { FeedPage } from "./pages/FeedPage";
import { ActivityListPage } from "./pages/ActivityListPage";
import { ActivityDetailPage } from "./pages/ActivityDetailPage";
import { ActivityEditPage } from "./pages/ActivityEditPage";
import { CreateActivityPage } from "./pages/CreateActivityPage";
import { ProfilePage } from "./pages/ProfilePage";
import { NotificationsPage } from "./pages/NotificationsPage";
import { UsersPage } from "./pages/UsersPage";
import { SettingsPage } from "./pages/SettingsPage";

function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <TooltipProvider>
          <Routes>
            {/* Public routes */}
            <Route path="/login" element={<LoginPage />} />
            <Route path="/register" element={<RegisterPage />} />

            {/* Protected routes with layout */}
            <Route element={<ProtectedRoute />}>
              <Route element={<Layout />}>
                <Route path="/" element={<FeedPage />} />
                <Route path="/activities" element={<ActivityListPage />} />
                <Route path="/activities/new" element={<CreateActivityPage />} />
                <Route path="/activities/:id" element={<ActivityDetailPage />} />
                <Route path="/activities/:id/edit" element={<ActivityEditPage />} />
                <Route path="/profile" element={<ProfilePage />} />
                <Route path="/profile/:id" element={<ProfilePage />} />
                <Route path="/notifications" element={<NotificationsPage />} />
                <Route path="/settings" element={<SettingsPage />} />
                <Route path="/users" element={<UsersPage />} />
              </Route>
            </Route>
          </Routes>
        </TooltipProvider>
      </AuthProvider>
    </BrowserRouter>
  );
}

export default App;
