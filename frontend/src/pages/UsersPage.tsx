import { useEffect, useState, useCallback } from "react";
import { Link } from "react-router-dom";
import { apiFetch } from "../lib/api";
import { useAuth } from "../contexts/useAuth";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { UserPlus, UserCheck, Search, Users } from "lucide-react";
import type { User } from "@/lib/types";

interface UserWithFollow extends User {
  is_following?: boolean;
}

export function UsersPage() {
  const { user: currentUser } = useAuth();
  const [users, setUsers] = useState<UserWithFollow[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [searching, setSearching] = useState(false);

  const fetchUsers = useCallback(async (query?: string) => {
    setLoading(true);
    try {
      const endpoint = query
        ? `/api/users?q=${encodeURIComponent(query)}`
        : "/api/users";
      const data = await apiFetch<UserWithFollow[]>(endpoint);
      setUsers(data);
    } catch {
      setUsers([]);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchUsers();
  }, [fetchUsers]);

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    setSearching(true);
    await fetchUsers(searchQuery || undefined);
    setSearching(false);
  };

  const handleFollow = async (userId: string, isCurrentlyFollowing: boolean) => {
    try {
      if (isCurrentlyFollowing) {
        await apiFetch(`/api/users/${userId}/follow`, { method: "DELETE" });
      } else {
        await apiFetch(`/api/users/${userId}/follow`, { method: "POST" });
      }
      setUsers((prev) =>
        prev.map((u) =>
          u.id === userId
            ? { ...u, is_following: !isCurrentlyFollowing }
            : u
        )
      );
    } catch {
      // Handle error silently
    }
  };

  if (loading) {
    return (
      <div className="mx-auto max-w-4xl space-y-6">
        <Skeleton className="h-8 w-[200px]" />
        <Skeleton className="h-10 w-full" />
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 6 }).map((_, i) => (
            <Card key={i}>
              <CardContent className="p-4">
                <div className="flex items-center space-x-4">
                  <Skeleton className="h-12 w-12 rounded-full" />
                  <div className="space-y-2">
                    <Skeleton className="h-4 w-[120px]" />
                    <Skeleton className="h-3 w-[80px]" />
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-4xl space-y-6">
      <h1 className="text-3xl font-bold tracking-tight">Find People</h1>

      {/* Search form */}
      <form onSubmit={handleSearch} className="flex space-x-2">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            type="text"
            placeholder="Search by username or search the fediverse (user@instance)..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          />
        </div>
        <Button
          type="submit"
          disabled={searching}
          className="focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          {searching ? "Searching..." : "Search"}
        </Button>
      </form>

      {/* Users list */}
      {users.length === 0 ? (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
              <Users className="h-8 w-8 text-muted-foreground" />
            </div>
            <h3 className="mt-4 text-lg font-semibold">No users found</h3>
            <p className="mt-1 text-sm text-muted-foreground">
              {searchQuery
                ? "Try a different search term"
                : "Be the first to join the community!"}
            </p>
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {users.map((u) => (
            <Card key={u.id} className="transition-shadow hover:shadow-md">
              <CardContent className="relative p-4">
                <div className="flex items-start gap-3">
                  <Avatar className="h-12 w-12 shrink-0">
                    <AvatarImage src={`/avatars/${u.id}.jpg`} alt="" />
                    <AvatarFallback>
                      {u.username?.[0]?.toUpperCase() || "U"}
                    </AvatarFallback>
                  </Avatar>
                  <div className="min-w-0 flex-1">
                    <Link
                      to={`/profile/${u.id}`}
                      className="focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-md"
                    >
                      <p className="font-medium">{u.username}</p>
                    </Link>
                    <p className="text-sm text-muted-foreground truncate">
                      {u.email}
                    </p>
                  </div>
                  {currentUser?.id !== u.id && (
                    <Button
                      variant={u.is_following ? "outline" : "default"}
                      size="sm"
                      onClick={() =>
                        handleFollow(u.id, u.is_following ?? false)
                      }
                      className="shrink-0 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                    >
                      {u.is_following ? (
                        <>
                          <UserCheck className="mr-1 h-4 w-4" aria-hidden="true" />
                          Following
                        </>
                      ) : (
                        <>
                          <UserPlus className="mr-1 h-4 w-4" aria-hidden="true" />
                          Follow
                        </>
                      )}
                    </Button>
                  )}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
