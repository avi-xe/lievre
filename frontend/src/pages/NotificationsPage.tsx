import { useEffect, useState, useCallback } from "react";
import { Link } from "react-router-dom";
import { apiFetch } from "@/lib/api";
import { useAuth } from "@/contexts/useAuth";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Heart,
  MessageCircle,
  UserPlus,
  Repeat2,
  Bell,
  BellOff,
  CheckCheck,
  RefreshCw,
  AlertCircle,
} from "lucide-react";

interface Notification {
  id: string;
  type: "like" | "comment" | "follow" | "repost";
  actor_id: string;
  actor_username: string;
  content: string;
  read: boolean;
  created_at: string;
  activity_id?: string;
}

function timeAgo(dateStr: string): string {
  const seconds = Math.floor((Date.now() - new Date(dateStr).getTime()) / 1000);
  if (seconds < 60) return "just now";
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}d ago`;
  return new Date(dateStr).toLocaleDateString();
}

function getNotificationIcon(type: Notification["type"]) {
  switch (type) {
    case "like":
      return { icon: Heart, label: "liked your activity", color: "text-rose-500" };
    case "comment":
      return { icon: MessageCircle, label: "commented on your activity", color: "text-blue-500" };
    case "follow":
      return { icon: UserPlus, label: "followed you", color: "text-emerald-500" };
    case "repost":
      return { icon: Repeat2, label: "reposted your activity", color: "text-violet-500" };
  }
}

function getNotificationLink(notification: Notification): string | null {
  if (notification.type === "follow") return `/users/${notification.actor_id}`;
  if (notification.activity_id) return `/activities/${notification.activity_id}`;
  return null;
}

function LoadingSkeleton() {
  return (
    <div className="mx-auto max-w-2xl space-y-4">
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <Skeleton className="h-6 w-[140px]" />
            <Skeleton className="h-8 w-[120px]" />
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {Array.from({ length: 4 }).map((_, i) => (
            <div key={i} className="flex items-start gap-3">
              <Skeleton className="h-10 w-10 shrink-0 rounded-full" />
              <div className="flex-1 space-y-2">
                <Skeleton className="h-4 w-[200px]" />
                <Skeleton className="h-3 w-[120px]" />
              </div>
              <Skeleton className="h-6 w-6 shrink-0 rounded" />
            </div>
          ))}
        </CardContent>
      </Card>
    </div>
  );
}

function EmptyState({ hasUnread }: { hasUnread: boolean }) {
  return (
    <div className="mx-auto max-w-2xl">
      <Card>
        <CardContent className="flex flex-col items-center justify-center py-12">
          <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
            <BellOff className="h-8 w-8 text-muted-foreground" aria-hidden="true" />
          </div>
          <h3 className="mt-4 text-lg font-semibold">No notifications</h3>
          <p className="mt-1 text-center text-sm text-muted-foreground">
            {hasUnread
              ? "You're all caught up!"
              : "When someone interacts with your activities, you'll see it here."}
          </p>
        </CardContent>
      </Card>
    </div>
  );
}

function ErrorState({
  message,
  onRetry,
}: {
  message: string;
  onRetry: () => void;
}) {
  return (
    <div className="mx-auto max-w-2xl">
      <Card>
        <CardContent className="flex flex-col items-center justify-center py-12">
          <div className="flex h-16 w-16 items-center justify-center rounded-full bg-destructive/10">
            <AlertCircle className="h-8 w-8 text-destructive" aria-hidden="true" />
          </div>
          <h3 className="mt-4 text-lg font-semibold">Something went wrong</h3>
          <p className="mt-1 text-sm text-muted-foreground">{message}</p>
          <Button variant="outline" onClick={onRetry} className="mt-4">
            <RefreshCw className="mr-2 h-4 w-4" aria-hidden="true" />
            Try Again
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}

function NotificationItem({
  notification,
  onMarkRead,
}: {
  notification: Notification;
  onMarkRead: (id: string) => void;
}) {
  const { icon: Icon, label, color } = getNotificationIcon(notification.type);
  const linkTarget = getNotificationLink(notification);

  const content = (
    <div
      className={`group flex items-start gap-3 rounded-lg p-3 transition-colors ${
        notification.read
          ? "bg-transparent hover:bg-muted/50"
          : "bg-muted/30 hover:bg-muted/60"
      } focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2 focus-within:ring-offset-background`}
    >
      {/* Avatar */}
      <Avatar size="default" className="shrink-0">
        <AvatarImage
          src={`https://api.dicebear.com/7.x/thumbs/svg?seed=${notification.actor_username}`}
          alt=""
        />
        <AvatarFallback>{notification.actor_username?.[0]?.toUpperCase() || "U"}</AvatarFallback>
      </Avatar>

      {/* Content */}
      <div className="flex-1 min-w-0">
        <p className="text-sm">
          <span className="font-semibold">{notification.actor_username}</span>{" "}
          <span className="text-muted-foreground">{notification.content || label}</span>
        </p>
        <time
          className="mt-1 block text-xs text-muted-foreground/70"
          dateTime={notification.created_at}
        >
          {timeAgo(notification.created_at)}
        </time>
      </div>

      {/* Icon + Unread indicator */}
      <div className="flex shrink-0 items-center gap-2">
        <Icon className={`h-5 w-5 ${color}`} aria-hidden="true" />
        {!notification.read && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onMarkRead(notification.id);
            }}
            className="flex h-11 w-11 items-center justify-center rounded-full bg-primary/20 transition-colors hover:bg-primary/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            aria-label={`Mark notification from ${notification.actor_username} as read`}
          >
            <CheckCheck className="h-4 w-4 text-primary" aria-hidden="true" />
          </button>
        )}
      </div>
    </div>
  );

  if (linkTarget) {
    return (
      <li>
        <Link
          to={linkTarget}
          className="block focus-visible:outline-none"
          aria-label={`${notification.actor_username} ${label}`}
        >
          {content}
        </Link>
      </li>
    );
  }

  return <li>{content}</li>;
}

export function NotificationsPage() {
  const { isAuthenticated } = useAuth();
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [markingAll, setMarkingAll] = useState(false);

  const fetchNotifications = useCallback(async () => {
    if (!isAuthenticated) {
      setLoading(false);
      return;
    }
    try {
      setLoading(true);
      setError(null);
      const data = await apiFetch<{ notifications: Notification[]; unread_count: number }>(
        "/api/notifications"
      );
      setNotifications(data.notifications || []);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load notifications");
    } finally {
      setLoading(false);
    }
  }, [isAuthenticated]);

  useEffect(() => {
    fetchNotifications();
  }, [fetchNotifications]);

  const markAllRead = async () => {
    setMarkingAll(true);
    try {
      await apiFetch("/api/notifications/read-all", { method: "PUT" });
      await fetchNotifications();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to mark all as read");
    } finally {
      setMarkingAll(false);
    }
  };

  const markRead = async (notifId: string) => {
    try {
      await apiFetch(`/api/notifications/${notifId}/read`, { method: "PUT" });
      // Optimistic update — flip the read state locally without refetching
      setNotifications((prev) =>
        prev.map((n) => (n.id === notifId ? { ...n, read: true } : n))
      );
    } catch (err) {
      // Silently ignore individual mark-read failures; refetch to stay consistent
      await fetchNotifications();
    }
  };

  // Unauthenticated state
  if (!isAuthenticated) {
    return (
      <div className="mx-auto max-w-2xl">
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
              <Bell className="h-8 w-8 text-muted-foreground" aria-hidden="true" />
            </div>
            <h3 className="mt-4 text-lg font-semibold">Sign in to see notifications</h3>
            <p className="mt-1 text-sm text-muted-foreground">
              Create an account or sign in to stay updated.
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (loading) return <LoadingSkeleton />;
  if (error) return <ErrorState message={error} onRetry={fetchNotifications} />;

  const unreadCount = notifications.filter((n) => !n.read).length;

  if (notifications.length === 0) {
    return <EmptyState hasUnread={false} />;
  }

  return (
    <div className="mx-auto max-w-2xl">
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <h1 className="flex items-center gap-2 font-heading text-base leading-snug font-medium">
              <Bell className="h-5 w-5" aria-hidden="true" />
              Notifications
              {unreadCount > 0 && (
                <Badge variant="destructive" className="ml-1">
                  {unreadCount}
                </Badge>
              )}
            </h1>
            {unreadCount > 0 && (
              <Button
                variant="outline"
                size="sm"
                onClick={markAllRead}
                disabled={markingAll}
                aria-label="Mark all notifications as read"
              >
                <CheckCheck className="mr-1.5 h-4 w-4" aria-hidden="true" />
                {markingAll ? "Marking..." : "Mark all read"}
              </Button>
            )}
          </div>
        </CardHeader>

        <Separator />

        <CardContent className="p-2 sm:max-h-[60vh] sm:overflow-auto">
          <ul className="space-y-1" role="list" aria-label="Notifications list">
            {notifications.map((notification, index) => (
              <NotificationItem
                key={notification.id}
                notification={notification}
                onMarkRead={markRead}
              />
            ))}
          </ul>
        </CardContent>
      </Card>
    </div>
  );
}
