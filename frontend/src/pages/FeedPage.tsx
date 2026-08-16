import { useEffect, useState } from "react";
import { apiFetch } from "../lib/api";
import { useAuth } from "../contexts/useAuth";
import { ActivityCard } from "@/components/activity";
import { Skeleton } from "@/components/ui/skeleton";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Map, Plus } from "lucide-react";
import { Link } from "react-router-dom";
import type { FeedItem } from "@/lib/types";

export function FeedPage() {
  const { isAuthenticated } = useAuth();
  const [items, setItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const endpoint = isAuthenticated ? "/api/feed" : "/api/feed/public";
    apiFetch<FeedItem[]>(endpoint)
      .then(setItems)
      .catch(() => setItems([]))
      .finally(() => setLoading(false));
  }, [isAuthenticated]);

  if (loading) {
    return (
      <div className="mx-auto max-w-2xl space-y-4">
        {Array.from({ length: 3 }).map((_, i) => (
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
    );
  }

  if (items.length === 0) {
    return (
      <div className="mx-auto max-w-2xl">
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
              <Map className="h-8 w-8 text-muted-foreground" />
            </div>
            <h3 className="mt-4 text-lg font-semibold">No activities yet</h3>
            <p className="mt-1 text-sm text-muted-foreground">
              {isAuthenticated
                ? "Be the first to share an activity!"
                : "Sign in to see activities from people you follow."}
            </p>
            {isAuthenticated && (
              <Button asChild className="mt-4">
                <Link to="/activities/new">
                  <Plus className="mr-2 h-4 w-4" aria-hidden="true" />
                  Record Activity
                </Link>
              </Button>
            )}
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-2xl space-y-4">
      {items.map((item) => (
        <ActivityCard key={item.id} activity={item} />
      ))}
    </div>
  );
}
