import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiFetch } from "@/lib/api";
import { Card, CardContent, CardHeader, CardTitle, CardAction } from "@/components/ui/card";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { Map, Plus, AlertCircle } from "lucide-react";
import type { Activity } from "@/lib/types";

interface ActivityWithLikes extends Activity {
  like_count?: number;
}

function formatDuration(seconds: number | null): string {
  if (seconds == null) return "-";
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m ${s}s`;
}

function formatDistance(meters: number | null): string {
  if (meters == null) return "-";
  return `${(meters / 1000).toFixed(1)} km`;
}

const activityTypeBadgeVariant: Record<string, "default" | "secondary" | "outline"> = {
  ride: "default",
  run: "secondary",
  walk: "outline",
  swim: "default",
  hike: "secondary",
};

function ActivityListSkeleton() {
  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <Skeleton className="h-6 w-[140px]" />
          <Skeleton className="h-8 w-[130px]" />
        </div>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Title</TableHead>
              <TableHead>Type</TableHead>
              <TableHead>Date</TableHead>
              <TableHead className="text-right">Distance</TableHead>
              <TableHead className="text-right">Duration</TableHead>
              <TableHead className="text-right">Likes</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {Array.from({ length: 5 }).map((_, i) => (
              <TableRow key={i}>
                <TableCell>
                  <Skeleton className="h-4 w-[120px]" />
                </TableCell>
                <TableCell>
                  <Skeleton className="h-5 w-[60px] rounded-full" />
                </TableCell>
                <TableCell>
                  <Skeleton className="h-4 w-[80px]" />
                </TableCell>
                <TableCell className="text-right">
                  <Skeleton className="ml-auto h-4 w-[50px]" />
                </TableCell>
                <TableCell className="text-right">
                  <Skeleton className="ml-auto h-4 w-[50px]" />
                </TableCell>
                <TableCell className="text-right">
                  <Skeleton className="ml-auto h-4 w-[30px]" />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}

function ActivityListError({ message, onRetry }: { message: string; onRetry: () => void }) {
  return (
    <Card>
      <CardContent className="flex flex-col items-center justify-center py-12">
        <div className="flex h-12 w-12 items-center justify-center rounded-full bg-destructive/10">
          <AlertCircle className="h-6 w-6 text-destructive" aria-hidden="true" />
        </div>
        <h3 className="mt-4 text-lg font-semibold">Failed to load activities</h3>
        <p className="mt-1 text-sm text-muted-foreground">{message}</p>
        <Button variant="outline" className="mt-4" onClick={onRetry}>
          Try again
        </Button>
      </CardContent>
    </Card>
  );
}

function ActivityListEmpty() {
  return (
    <Card>
      <CardContent className="flex flex-col items-center justify-center py-12">
        <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
          <Map className="h-8 w-8 text-muted-foreground" aria-hidden="true" />
        </div>
        <h3 className="mt-4 text-lg font-semibold">No activities yet</h3>
        <p className="mt-1 text-sm text-muted-foreground">
          Start tracking your workouts and they will appear here.
        </p>
        <Button asChild className="mt-4">
          <Link to="/activities/new" className="inline-flex items-center">
            <Plus className="mr-2 h-4 w-4" aria-hidden="true" />
            New Activity
          </Link>
        </Button>
      </CardContent>
    </Card>
  );
}

export function ActivityListPage() {
  const [activities, setActivities] = useState<ActivityWithLikes[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const fetchActivities = () => {
    setLoading(true);
    setError("");
    apiFetch<ActivityWithLikes[]>("/api/activities")
      .then(setActivities)
      .catch((err) => setError(err.message))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    fetchActivities();
  }, []);

  if (loading) {
    return (
      <div className="mx-auto max-w-3xl space-y-4 p-4">
        <ActivityListSkeleton />
      </div>
    );
  }

  if (error) {
    return (
      <div className="mx-auto max-w-3xl space-y-4 p-4">
        <ActivityListError message={error} onRetry={fetchActivities} />
      </div>
    );
  }

  if (activities.length === 0) {
    return (
      <div className="mx-auto max-w-3xl space-y-4 p-4">
        <ActivityListEmpty />
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-3xl space-y-4 p-4">
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>My Activities</CardTitle>
            <CardAction>
              <Button asChild>
                <Link to="/activities/new" className="inline-flex items-center">
                  <Plus className="mr-2 h-4 w-4" aria-hidden="true" />
                  New Activity
                </Link>
              </Button>
            </CardAction>
          </div>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Title</TableHead>
                <TableHead>Type</TableHead>
                <TableHead>Date</TableHead>
                <TableHead className="text-right">Distance</TableHead>
                <TableHead className="text-right">Duration</TableHead>
                <TableHead className="text-right">Likes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {activities.map((activity) => (
                <TableRow key={activity.id}>
                  <TableCell>
                    <Link
                      to={`/activities/${activity.id}`}
                      className="font-medium text-foreground underline-offset-4 hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-sm"
                    >
                      {activity.title}
                    </Link>
                  </TableCell>
                  <TableCell>
                    <Badge variant={activityTypeBadgeVariant[activity.activity_type] ?? "secondary"}>
                      {activity.activity_type}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-muted-foreground">
                    {new Date(activity.started_at).toLocaleDateString()}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">
                    {formatDistance(activity.distance_meters)}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">
                    {formatDuration(activity.duration_seconds)}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">
                    {activity.like_count ?? "-"}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
