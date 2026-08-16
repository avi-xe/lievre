import { useEffect, useState } from "react";
import { useParams, useNavigate, Link } from "react-router-dom";
import { apiFetch } from "../lib/api";
import { useAuth } from "../contexts/useAuth";
import { ActivityMap } from "../components/ActivityMap";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import {
  Heart,
  MessageCircle,
  Pencil,
  Trash2,
  Clock,
  Mountain,
  MapPin,
  Send,
} from "lucide-react";
import { activityIcons, activityColors } from "@/lib/activity-helpers";
import type { Activity, Comment } from "@/lib/types";

function formatDuration(seconds: number | null): string | null {
  if (!seconds) return null;
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  }
  return `${minutes}m`;
}

function formatDistance(meters: number | null): string | null {
  if (!meters) return null;
  const km = meters / 1000;
  return `${km.toFixed(1)} km`;
}

export function ActivityDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { user } = useAuth();
  const navigate = useNavigate();
  const [activity, setActivity] = useState<Activity | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [commentText, setCommentText] = useState("");
  const [liked, setLiked] = useState(false);
  const [likeCount, setLikeCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

  const loadLikes = async (activityId: string) => {
    try {
      const data = await apiFetch<{ count: number; liked: boolean }>(
        `/api/activities/${activityId}/likes`
      );
      setLikeCount(data.count);
      setLiked(data.liked);
    } catch {
      // likes endpoint may not exist yet
    }
  };

  useEffect(() => {
    if (!id) return;
    Promise.all([
      apiFetch<Activity>(`/api/activities/${id}`),
      apiFetch<Comment[]>(`/api/activities/${id}/comments`).catch(() => []),
    ])
      .then(([act, cmts]) => {
        setActivity(act);
        setComments(cmts);
        loadLikes(id);
      })
      .catch((err) => setError(err.message))
      .finally(() => setLoading(false));
  }, [id]);

  const handleDelete = async () => {
    if (!id) return;
    await apiFetch(`/api/activities/${id}`, { method: "DELETE" });
    navigate("/");
  };

  const handleLike = async () => {
    if (!id) return;
    if (liked) {
      await apiFetch(`/api/activities/${id}/like`, { method: "DELETE" });
    } else {
      await apiFetch(`/api/activities/${id}/like`, { method: "POST" });
    }
    loadLikes(id);
  };

  const handleComment = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!id || !commentText.trim()) return;
    const comment = await apiFetch<Comment>(`/api/activities/${id}/comments`, {
      method: "POST",
      body: JSON.stringify({ content: commentText }),
    });
    setComments([...comments, comment]);
    setCommentText("");
  };

  const handleDeleteComment = async (commentId: string) => {
    await apiFetch(`/api/comments/${commentId}`, { method: "DELETE" });
    setComments((prev) => prev.filter((c) => c.id !== commentId));
  };

  if (loading) {
    return (
      <div className="mx-auto max-w-4xl space-y-6">
        <Skeleton className="h-8 w-[300px]" />
        <Skeleton className="h-4 w-[200px]" />
        <div className="grid grid-cols-3 gap-4">
          <Skeleton className="h-24" />
          <Skeleton className="h-24" />
          <Skeleton className="h-24" />
        </div>
        <Skeleton className="h-[400px] w-full" />
        <Skeleton className="h-32" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="mx-auto max-w-4xl">
        <Card>
          <CardContent className="py-12 text-center">
            <p className="text-destructive">{error}</p>
            <Button asChild variant="outline" className="mt-4">
              <Link to="/">Back to Feed</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (!activity) {
    return (
      <div className="mx-auto max-w-4xl">
        <Card>
          <CardContent className="py-12 text-center">
            <p className="text-muted-foreground">Activity not found</p>
            <Button asChild variant="outline" className="mt-4">
              <Link to="/">Back to Feed</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  const isOwner = user?.id === activity.user_id;
  const Icon = activityIcons[activity.activity_type] || Bike;
  const colorClass =
    activityColors[activity.activity_type] || "bg-muted text-muted-foreground";

  return (
    <div className="mx-auto max-w-4xl space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div className="space-y-2">
          <div className="flex items-center space-x-3">
            <Link
              to={`/profile/${activity.user_id}`}
              className="focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-full"
            >
              <Avatar className="h-12 w-12">
                <AvatarImage src={`/avatars/${activity.user_id}.jpg`} alt="" />
                <AvatarFallback>
                  {activity.username?.[0]?.toUpperCase() || activity.user_id?.[0]?.toUpperCase() || "U"}
                </AvatarFallback>
              </Avatar>
            </Link>
            <div>
              <Link
                to={`/profile/${activity.user_id}`}
                className="text-lg font-semibold hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-sm"
              >
                {activity.username || activity.user_id}
              </Link>
              <p className="text-sm text-muted-foreground">
                {new Date(activity.started_at).toLocaleString()}
              </p>
            </div>
          </div>
          <h1 className="text-3xl font-bold tracking-tight">
            {activity.title}
          </h1>
        </div>
        <Badge variant="secondary" className={colorClass}>
          <Icon className="mr-1 h-4 w-4" aria-hidden="true" />
          {activity.activity_type}
        </Badge>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-3 gap-4">
        <Card>
          <CardContent className="py-4 text-center">
            <MapPin className="mx-auto h-5 w-5 text-muted-foreground" />
            <p className="mt-2 text-2xl font-bold">
              {formatDistance(activity.distance_meters) || "-"}
            </p>
            <p className="text-sm text-muted-foreground">Distance</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="py-4 text-center">
            <Clock className="mx-auto h-5 w-5 text-muted-foreground" />
            <p className="mt-2 text-2xl font-bold">
              {formatDuration(activity.duration_seconds) || "-"}
            </p>
            <p className="text-sm text-muted-foreground">Duration</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="py-4 text-center">
            <Mountain className="mx-auto h-5 w-5 text-muted-foreground" />
            <p className="mt-2 text-2xl font-bold">
              {activity.elevation_gain_meters != null
                ? `${Math.round(activity.elevation_gain_meters)} m`
                : "-"}
            </p>
            <p className="text-sm text-muted-foreground">Elevation</p>
          </CardContent>
        </Card>
      </div>

      {/* Map */}
      {id && (
        <Card className="overflow-hidden">
          <ActivityMap activityId={id} />
        </Card>
      )}

      {/* Actions */}
      <div className="flex items-center space-x-2">
        <Button
          variant={liked ? "default" : "outline"}
          onClick={handleLike}
          className="h-11 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          <Heart
            className={`mr-2 h-4 w-4 ${liked ? "fill-current" : ""}`}
            aria-hidden="true"
          />
          {liked ? "Liked" : "Like"}
          {likeCount > 0 && ` (${likeCount})`}
        </Button>
        {isOwner && (
          <>
            <Link
              to={`/activities/${id}/edit`}
              className="inline-flex h-11 items-center justify-center rounded-md border border-input bg-background px-4 text-sm font-medium shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            >
              <Pencil className="mr-2 h-4 w-4" aria-hidden="true" />
              Edit
            </Link>
            <Button
              variant="destructive"
              onClick={() => setDeleteDialogOpen(true)}
              className="h-11 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            >
              <Trash2 className="mr-2 h-4 w-4" aria-hidden="true" />
              Delete
            </Button>
          </>
        )}
      </div>

      <Separator />

      {/* Comments */}
      <div className="space-y-4">
        <h2 className="text-xl font-semibold">
          Comments ({comments.length})
        </h2>

        {comments.length === 0 ? (
          <Card>
            <CardContent className="py-8 text-center">
              <MessageCircle className="mx-auto h-8 w-8 text-muted-foreground" />
              <p className="mt-2 text-sm text-muted-foreground">
                No comments yet. Be the first to comment!
              </p>
            </CardContent>
          </Card>
        ) : (
          <div className="space-y-4">
            {comments.map((comment) => (
              <Card key={comment.id}>
                <CardHeader className="pb-2">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-2">
                      <Avatar className="h-8 w-8">
                        <AvatarImage src={`/avatars/${comment.user_id}.jpg`} alt="" />
                        <AvatarFallback>
                          {comment.username?.[0]?.toUpperCase() || "U"}
                        </AvatarFallback>
                      </Avatar>
                      <div>
                        <p className="text-sm font-medium">
                          {comment.username}
                        </p>
                        <p className="text-xs text-muted-foreground">
                          {new Date(comment.created_at).toLocaleString()}
                        </p>
                      </div>
                    </div>
                    {user?.id === comment.user_id && (
                      <Button
                        variant="ghost"
                        size="icon"
                        className="h-8 w-8 text-muted-foreground hover:text-destructive focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                        onClick={() => handleDeleteComment(comment.id)}
                        aria-label="Delete comment"
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    )}
                  </div>
                </CardHeader>
                <CardContent>
                  <p className="text-sm">{comment.content}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        )}

        {/* Comment form */}
        <form onSubmit={handleComment} className="flex space-x-2">
          <Input
            type="text"
            placeholder="Add a comment..."
            value={commentText}
            onChange={(e) => setCommentText(e.target.value)}
            className="h-11 flex-1 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          />
          <Button
            type="submit"
            disabled={!commentText.trim()}
            className="h-11 w-11 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          >
            <Send className="h-4 w-4" aria-hidden="true" />
            <span className="sr-only">Post comment</span>
          </Button>
        </form>
      </div>

      {/* Delete Dialog */}
      <AlertDialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Activity</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this activity? This action cannot
              be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDelete}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
