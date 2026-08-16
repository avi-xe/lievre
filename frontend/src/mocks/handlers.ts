import { http, HttpResponse } from "msw";
import {
  mockUser,
  mockUsers,
  mockActivities,
  mockComments,
  mockNotifications,
} from "./data";

const API_BASE = "/api";

export const handlers = [
  // Auth
  http.post(`${API_BASE}/auth/login`, async ({ request }) => {
    const body = (await request.json()) as { email: string; password: string };
    if (body.email && body.password) {
      return HttpResponse.json({
        token: "mock-jwt-token",
        user: mockUser,
      });
    }
    return new HttpResponse("Invalid credentials", { status: 401 });
  }),

  http.post(`${API_BASE}/auth/register`, async ({ request }) => {
    const body = (await request.json()) as {
      email: string;
      username: string;
      password: string;
    };
    if (body.email && body.username && body.password) {
      return HttpResponse.json({
        token: "mock-jwt-token",
        user: { ...mockUser, email: body.email, username: body.username },
      });
    }
    return new HttpResponse("Registration failed", { status: 400 });
  }),

  // Current user
  http.get(`${API_BASE}/users/me`, () => {
    return HttpResponse.json(mockUser);
  }),

  // Users
  http.get(`${API_BASE}/users`, ({ request }) => {
    const url = new URL(request.url);
    const query = url.searchParams.get("q");
    let users = [...mockUsers];

    if (query) {
      users = users.filter(
        (u) =>
          u.username.toLowerCase().includes(query.toLowerCase()) ||
          u.email.toLowerCase().includes(query.toLowerCase())
      );
    }

    return HttpResponse.json(users);
  }),

  http.get(`${API_BASE}/users/:id`, ({ params }) => {
    const user = mockUsers.find((u) => u.id === params.id);
    if (user) {
      return HttpResponse.json(user);
    }
    return new HttpResponse("User not found", { status: 404 });
  }),

  http.get(`${API_BASE}/users/:id/follow-status`, () => {
    return HttpResponse.json({ is_following: false });
  }),

  http.post(`${API_BASE}/users/:id/follow`, () => {
    return HttpResponse.json({ success: true });
  }),

  http.delete(`${API_BASE}/users/:id/follow`, () => {
    return HttpResponse.json({ success: true });
  }),

  http.get(`${API_BASE}/users/:id/activities`, () => {
    return HttpResponse.json(mockActivities.slice(0, 2));
  }),

  // Feed
  http.get(`${API_BASE}/feed`, () => {
    return HttpResponse.json(mockActivities);
  }),

  http.get(`${API_BASE}/feed/public`, () => {
    return HttpResponse.json(mockActivities);
  }),

  // Activities
  http.get(`${API_BASE}/activities`, () => {
    return HttpResponse.json(mockActivities);
  }),

  http.get(`${API_BASE}/activities/:id`, ({ params }) => {
    const activity = mockActivities.find((a) => a.id === params.id);
    if (activity) {
      return HttpResponse.json(activity);
    }
    return new HttpResponse("Activity not found", { status: 404 });
  }),

  http.post(`${API_BASE}/activities`, async ({ request }) => {
    const body = (await request.json()) as Record<string, unknown>;
    const newActivity = {
      id: `activity-${Date.now()}`,
      ...body,
      user_id: mockUser.id,
      username: mockUser.username,
      like_count: 0,
    };
    mockActivities.unshift(newActivity);
    return HttpResponse.json(newActivity, { status: 201 });
  }),

  http.put(`${API_BASE}/activities/:id`, async ({ params, request }) => {
    const body = (await request.json()) as Record<string, unknown>;
    const index = mockActivities.findIndex((a) => a.id === params.id);
    if (index !== -1) {
      mockActivities[index] = { ...mockActivities[index], ...body };
      return HttpResponse.json(mockActivities[index]);
    }
    return new HttpResponse("Activity not found", { status: 404 });
  }),

  http.delete(`${API_BASE}/activities/:id`, ({ params }) => {
    const index = mockActivities.findIndex((a) => a.id === params.id);
    if (index !== -1) {
      mockActivities.splice(index, 1);
      return new HttpResponse(null, { status: 204 });
    }
    return new HttpResponse("Activity not found", { status: 404 });
  }),

  // Likes
  http.get(`${API_BASE}/activities/:id/likes`, () => {
    return HttpResponse.json({ count: 5, liked: false });
  }),

  http.post(`${API_BASE}/activities/:id/like`, () => {
    return HttpResponse.json({ success: true });
  }),

  http.delete(`${API_BASE}/activities/:id/like`, () => {
    return HttpResponse.json({ success: true });
  }),

  // Comments
  http.get(`${API_BASE}/activities/:id/comments`, ({ params }) => {
    const comments = mockComments.filter(
      (c) => c.activity_id === params.id
    );
    return HttpResponse.json(comments);
  }),

  http.post(`${API_BASE}/activities/:id/comments`, async ({ request }) => {
    const body = (await request.json()) as { content: string };
    const newComment = {
      id: `comment-${Date.now()}`,
      activity_id: "activity-1",
      user_id: mockUser.id,
      username: mockUser.username,
      content: body.content,
      created_at: new Date().toISOString(),
    };
    return HttpResponse.json(newComment, { status: 201 });
  }),

  http.delete(`${API_BASE}/comments/:id`, () => {
    return new HttpResponse(null, { status: 204 });
  }),

  // GPX Import
  http.post(`${API_BASE}/import/gpx`, async () => {
    const id = `activity-gpx-${Date.now()}`;
    const marathonActivity = {
      id,
      user_id: mockUser.id,
      activity_type: "run",
      title: "Berlin Marathon",
      description: "42.2 km marathon course modeled on the BMW Berlin Marathon route",
      started_at: "2025-09-28T09:15:00Z",
      duration_seconds: 12600,
      distance_meters: 42195,
      elevation_gain_meters: 45,
      visibility: "public",
      like_count: 0,
      username: mockUser.username,
    };
    mockActivities.unshift(marathonActivity);
    return HttpResponse.json({ activity_id: id, ...marathonActivity }, { status: 201 });
  }),

  // Notifications
  http.get(`${API_BASE}/notifications`, () => {
    return HttpResponse.json(mockNotifications);
  }),

  http.post(`${API_BASE}/notifications/:id/read`, () => {
    return HttpResponse.json({ success: true });
  }),

  http.post(`${API_BASE}/notifications/read-all`, () => {
    return HttpResponse.json({ success: true });
  }),

  // Catch-all for unmatched requests
  http.all(`${API_BASE}/*`, () => {
    return new HttpResponse("Not found", { status: 404 });
  }),
];
