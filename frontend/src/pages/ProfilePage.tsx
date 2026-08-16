import { useEffect, useState } from "react";
import { useParams, Link } from "react-router-dom";
import { apiFetch } from "../lib/api";
import { useAuth } from "../contexts/useAuth";
import { ActivityCard } from "@/components/activity";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import {
  UserPlus,
  UserMinus,
  AlertTriangle,
  MapPin,
} from "lucide-react";
import type { Activity, FeedItem } from "../lib/types";

export function ProfilePage() {
  const { id } = useParams<{ id: string }>();
  const { user: currentUser } = useAuth();
  const [profile, setProfile] = useState<unknown>(null);
  const [activities, setActivities] = useState<Activity[]>([]);
  const [isFollowing, setIsFollowing] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);
  const [followLoading, setFollowLoading] = useState(false);

  const isOwn = currentUser?.id === profileId;

  const profileId = id || currentUser?.id;

  useEffect(() => {
    if (!profileId) {
      setLoading(false);
      return;
    }

    setLoading(true);
    setError(false);

    apiFetch<{ username: string; email: string; id: string }>(`/api/users/${profileId}`)
      .then((data) => setProfile(data))
      .catch(() => setError(true))
      .finally(() => setLoading(false));

    apiFetch<Activity[]>(`/api/users/${profileId}/activities`)
      .then((data) => setActivities(data))
      .catch(() => setActivities([]));

    if (currentUser && currentUser.id !== profileId) {
      apiFetch<{ is_following: boolean }>(`/api/users/${profileId}/follow-status`)
        .then((data) => setIsFollowing(data.is_following))
        .catch(() => {});
    }
  }, [profileId, currentUser]);

  const handleFollow = async () => {
    if (!profileId || followLoading) return;
    setFollowLoading(true);
    try {
      if (isFollowing) {
        await apiFetch(`/api/users/${profileId}/follow`, { method: "DELETE" });
        setIsFollowing(false);
      } else {
        await apiFetch(`/api/users/${profileId}/follow`, { method: "POST" });
        setIsFollowing(true);
      }
    } catch {
      // Follow state unchanged on error
    } finally {
      setFollowLoading(false);
    }
  };

  const p = profile as { username: string; email: string; id: string } | null;

  // Loading state
  if (loading) {
    return (
      <div className="mx-auto max-w-2xl space-y-4">
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center space-x-4">
              <Skeleton className="h-16 w-16 rounded-full" />
              <div className="space-y-2">
                <Skeleton className="h-5 w-[160px]" />
                <Skeleton className="h-4 w-[200px]" />
              </div>
            </div>
          </CardContent>
        </Card>
        <Skeleton className="h-px w-full" />
        <div className="space-y-3">
          {Array.from({ length: 2 }).map((_, i) => (
            <Card key={i}>
              <CardContent className="p-6">
                <div className="flex items-center space-x-4">
                  <Skeleton className="h-10 w-10 rounded-full" />
                  <div className="space-y-2">
                    <Skeleton className="h-4 w-[150px]" />
                    <Skeleton className="h-3 w-[100px]" />
                  </div>
                </div>
                <Skeleton className="mt-4 h-6 w-[200px]" />
                <Skeleton className="mt-2 h-4 w-[300px]" />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    );
  }

  // Error state
  if (error || !p) {
    return (
      <div className="mx-auto max-w-2xl">
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <div className="flex h-16 w-16 items-center justify-center rounded-full bg-destructive/10">
              <AlertTriangle className="h-8 w-8 text-destructive" />
            </div>
            <h3 className="mt-4 text-lg font-semibold">Profile not found</h3>
            <p className="mt-1 text-sm text-muted-foreground">
              This user may not exist or the profile could not be loaded.
            </p>
            <Button asChild variant="outline" className="mt-4">
              <Link to="/">Back to feed</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-2xl space-y-4">
      {/* Profile header */}
      <Card>
        <CardContent className="p-6">
          <div className="flex items-start justify-between">
            <div className="flex items-center space-x-4">
              <Avatar className="h-16 w-16">
                <AvatarImage
                  src={`/avatars/${p.id}.jpg`}
                  alt={`${p.username}'s avatar`}
                />
                <AvatarFallback className="text-lg">
                  {p.username[0]?.toUpperCase() || "U"}
                </AvatarFallback>
              </Avatar>
              <div>
                <h2 className="text-xl font-bold">{p.username}</h2>
                <p className="text-sm text-muted-foreground">{p.email}</p>
                <div className="mt-2 flex items-center space-x-2">
                  <Badge variant="secondary">
                    <MapPin className="mr-1 h-3 w-3" aria-hidden="true" />
                    {activities.length} {activities.length === 1 ? "activity" : "activities"}
                  </Badge>
                </div>
              </div>
            </div>

            {!isOwn && currentUser && (
              <Button
                variant={isFollowing ? "outline" : "default"}
                size="sm"
                onClick={handleFollow}
                disabled={followLoading}
                aria-label={isFollowing ? `Unfollow ${p.username}` : `Follow ${p.username}`}
              >
                {followLoading ? (
                  <Skeleton className="h-4 w-4 rounded" />
                ) : isFollowing ? (
                  <>
                    <UserMinus className="mr-2 h-4 w-4" aria-hidden="true" />
                    Unfollow
                  </>
                ) : (
                  <>
                    <UserPlus className="mr-2 h-4 w-4" aria-hidden="true" />
                    Follow
                  </>
                )}
              </Button>
            )}
          </div>
        </CardContent>
      </Card>

      <Separator />

      {/* Activities section */}
      <div>
        <h3 className="mb-3 text-lg font-semibold">
          Activities
          <span className="ml-2 text-sm font-normal text-muted-foreground">
            ({activities.length})
          </span>
        </h3>

        {activities.length === 0 ? (
          <Card>
            <CardContent className="flex flex-col items-center justify-center py-12">
              <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
                <MapPin className="h-8 w-8 text-muted-foreground" />
              </div>
              <h3 className="mt-4 text-lg font-semibold">No activities yet</h3>
              <p className="mt-1 text-sm text-muted-foreground">
                {isOwn
                  ? "Record your first activity to see it here."
                  : `${p.username} hasn't shared any activities yet.`}
              </p>
              {isOwn && (
                <Button asChild className="mt-4">
                  <Link to="/activities/new">
                    <MapPin className="mr-2 h-4 w-4" aria-hidden="true" />
                    Record Activity
                  </Link>
                </Button>
              )}
            </CardContent>
          </Card>
        ) : (
          <div className="space-y-4">
            {activities.map((a) => (
              <ActivityCard
                key={a.id}
                activity={a as FeedItem}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
