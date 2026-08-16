import { Link } from "react-router-dom";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Heart, MessageCircle, Repeat2, Clock, Mountain, Bike, Footprints, Waves, TreePine } from "lucide-react";
import type { FeedItem } from "@/lib/types";

interface ActivityCardProps {
  activity: FeedItem;
}

const activityIcons: Record<string, typeof Bike> = {
  ride: Bike,
  run: Footprints,
  swim: Waves,
  walk: Footprints,
  hike: TreePine,
};

const activityColors: Record<string, string> = {
  ride: "bg-ride/10 text-ride",
  run: "bg-run/10 text-run",
  swim: "bg-swim/10 text-swim",
  walk: "bg-walk/10 text-walk",
  hike: "bg-hike/10 text-hike",
};

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

export function ActivityCard({ activity }: ActivityCardProps) {
  const Icon = activityIcons[activity.activity_type] || Bike;
  const colorClass = activityColors[activity.activity_type] || "bg-muted text-muted-foreground";

  return (
    <Card className="transition-shadow hover:shadow-md">
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <Link
              to={`/profile/${activity.user_id}`}
              className="focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-full"
            >
              <Avatar className="h-10 w-10">
                <AvatarImage src={`/avatars/${activity.user_id}.jpg`} alt="" />
                <AvatarFallback>{activity.username?.[0]?.toUpperCase() || "U"}</AvatarFallback>
              </Avatar>
            </Link>
            <div>
              <Link
                to={`/profile/${activity.user_id}`}
                className="font-medium hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-sm"
              >
                {activity.username || "Unknown"}
              </Link>
              <p className="text-sm text-muted-foreground">
                {new Date(activity.started_at).toLocaleDateString()}
              </p>
            </div>
          </div>
          <Badge variant="secondary" className={colorClass}>
            <Icon className="mr-1 h-3 w-3" aria-hidden="true" />
            {activity.activity_type}
          </Badge>
        </div>
      </CardHeader>

      <CardContent className="pb-3">
        <Link
          to={`/activities/${activity.id}`}
          className="text-lg font-semibold hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-sm"
        >
          {activity.title}
        </Link>

        <div className="mt-3 flex flex-wrap gap-4 text-sm text-muted-foreground">
          {activity.distance_meters != null && (
            <div className="flex items-center">
              <Mountain className="mr-1 h-4 w-4" aria-hidden="true" />
              {formatDistance(activity.distance_meters)}
            </div>
          )}
          {activity.duration_seconds != null && (
            <div className="flex items-center">
              <Clock className="mr-1 h-4 w-4" aria-hidden="true" />
              {formatDuration(activity.duration_seconds)}
            </div>
          )}
        </div>
      </CardContent>

      <Separator />

      <CardFooter className="pt-3">
        <div className="flex w-full items-center justify-between">
          <div className="flex items-center space-x-1">
            <Button
              variant="ghost"
              size="sm"
              className="h-8 px-2 text-muted-foreground hover:text-primary focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              aria-label={`Like ${activity.title}`}
            >
              <Heart className="mr-1 h-4 w-4" aria-hidden="true" />
              {activity.like_count ?? 0}
            </Button>
            <Button
              variant="ghost"
              size="sm"
              className="h-8 px-2 text-muted-foreground hover:text-primary focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              aria-label={`Comment on ${activity.title}`}
            >
              <MessageCircle className="mr-1 h-4 w-4" aria-hidden="true" />
              0
            </Button>
            <Button
              variant="ghost"
              size="sm"
              className="h-8 px-2 text-muted-foreground hover:text-primary focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              aria-label={`Repost ${activity.title}`}
            >
              <Repeat2 className="h-4 w-4" aria-hidden="true" />
            </Button>
          </div>
        </div>
      </CardFooter>
    </Card>
  );
}
