import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { apiFetch } from "../lib/api";
import type { Activity } from "../lib/types";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectPopup,
  SelectItem,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Skeleton } from "@/components/ui/skeleton";
import { Save, X, AlertCircle } from "lucide-react";

const ACTIVITY_TYPES = [
  { value: "ride", label: "Ride" },
  { value: "run", label: "Run" },
  { value: "swim", label: "Swim" },
  { value: "walk", label: "Walk" },
  { value: "hike", label: "Hike" },
  { value: "virtual-ride", label: "Virtual Ride" },
] as const;

const VISIBILITY_OPTIONS = [
  { value: "public", label: "Public" },
  { value: "followers", label: "Followers" },
  { value: "private", label: "Private" },
] as const;

export function ActivityEditPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [form, setForm] = useState({
    activity_type: "ride",
    title: "",
    description: "",
    started_at: "",
    visibility: "public",
  });
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (!id) return;
    apiFetch<Activity>(`/api/activities/${id}`)
      .then((act) => {
        setForm({
          activity_type: act.activity_type,
          title: act.title,
          description: act.description ?? "",
          started_at: act.started_at ? act.started_at.slice(0, 16) : "",
          visibility: act.visibility,
        });
      })
      .catch((err) => setError(err.message))
      .finally(() => setLoading(false));
  }, [id]);

  const handleSelectChange = (name: string, value: string) => {
    setForm({ ...form, [name]: value });
  };

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    setForm({ ...form, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!id) return;
    setError("");
    setSaving(true);
    try {
      await apiFetch(`/api/activities/${id}`, {
        method: "PUT",
        body: JSON.stringify({
          ...form,
          started_at: form.started_at
            ? `${form.started_at}:00Z`
            : form.started_at,
        }),
      });
      navigate(`/activities/${id}`);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to update activity"
      );
    } finally {
      setSaving(false);
    }
  };

  // Loading skeleton
  if (loading) {
    return (
      <div className="mx-auto max-w-2xl space-y-4">
        <Card>
          <CardHeader>
            <Skeleton className="h-6 w-[180px]" />
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Skeleton className="h-4 w-[80px]" />
              <Skeleton className="h-8 w-full" />
            </div>
            <div className="space-y-2">
              <Skeleton className="h-4 w-[60px]" />
              <Skeleton className="h-8 w-full" />
            </div>
            <div className="space-y-2">
              <Skeleton className="h-4 w-[100px]" />
              <Skeleton className="h-20 w-full" />
            </div>
            <div className="space-y-2">
              <Skeleton className="h-4 w-[80px]" />
              <Skeleton className="h-8 w-full" />
            </div>
            <div className="space-y-2">
              <Skeleton className="h-4 w-[90px]" />
              <Skeleton className="h-8 w-full" />
            </div>
          </CardContent>
          <CardFooter className="gap-2">
            <Skeleton className="h-8 w-full" />
            <Skeleton className="h-8 w-full" />
          </CardFooter>
        </Card>
      </div>
    );
  }

  // Error state (before form loads)
  if (error && !saving && loading === false) {
    return (
      <div className="mx-auto max-w-2xl">
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <Alert variant="destructive" className="max-w-md">
              <AlertCircle className="size-4" />
              <AlertDescription>{error}</AlertDescription>
            </Alert>
            <Button
              variant="outline"
              className="mt-4"
              onClick={() => navigate("/")}
            >
              Back to Feed
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-2xl space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>Edit Activity</CardTitle>
        </CardHeader>
        <form onSubmit={handleSubmit}>
          <CardContent className="space-y-4">
            {/* Inline error */}
            {error && (
              <Alert variant="destructive">
                <AlertCircle className="size-4" />
                <AlertDescription>{error}</AlertDescription>
              </Alert>
            )}

            {/* Activity Type */}
            <div className="space-y-2">
              <Label htmlFor="activity_type" className="flex items-center gap-1.5">
                Activity Type
              </Label>
              <Select
                value={form.activity_type}
                onValueChange={(v) => handleSelectChange("activity_type", v)}
              >
                <SelectTrigger id="activity_type" className="w-full">
                  <SelectValue placeholder="Select activity type" />
                </SelectTrigger>
                <SelectPopup>
                  {ACTIVITY_TYPES.map((t) => (
                    <SelectItem key={t.value} value={t.value}>
                      {t.label}
                    </SelectItem>
                  ))}
                </SelectPopup>
              </Select>
            </div>

            {/* Title */}
            <div className="space-y-2">
              <Label htmlFor="title">Title</Label>
              <Input
                id="title"
                name="title"
                value={form.title}
                onChange={handleInputChange}
                required
                placeholder="Activity title"
              />
            </div>

            {/* Description */}
            <div className="space-y-2">
              <Label htmlFor="description">Description</Label>
              <Textarea
                id="description"
                name="description"
                value={form.description}
                onChange={handleInputChange}
                rows={3}
                placeholder="Describe your activity..."
              />
            </div>

            {/* Date Picker */}
            <div className="space-y-2">
              <Label htmlFor="started_at">Start Time</Label>
              <Input
                id="started_at"
                name="started_at"
                type="datetime-local"
                value={form.started_at}
                onChange={handleInputChange}
                required
              />
            </div>

            {/* Visibility */}
            <div className="space-y-2">
              <Label htmlFor="visibility" className="flex items-center gap-1.5">
                Visibility
              </Label>
              <Select
                value={form.visibility}
                onValueChange={(v) => handleSelectChange("visibility", v)}
              >
                <SelectTrigger id="visibility" className="w-full">
                  <SelectValue placeholder="Select visibility" />
                </SelectTrigger>
                <SelectPopup>
                  {VISIBILITY_OPTIONS.map((v) => (
                    <SelectItem key={v.value} value={v.value}>
                      {v.label}
                    </SelectItem>
                  ))}
                </SelectPopup>
              </Select>
            </div>
          </CardContent>

          <CardFooter className="gap-2">
            <Button
              type="submit"
              disabled={saving}
              className="flex-1"
              aria-label={saving ? "Saving changes..." : "Save changes"}
            >
              {saving ? (
                <>
                  <Skeleton className="mr-2 size-4" />
                  Saving...
                </>
              ) : (
                <>
                  <Save className="mr-2 size-4" aria-hidden="true" />
                  Save Changes
                </>
              )}
            </Button>
            <Button
              type="button"
              variant="outline"
              className="flex-1"
              onClick={() => navigate(`/activities/${id}`)}
              aria-label="Cancel and return to activity"
            >
              <X className="mr-2 size-4" aria-hidden="true" />
              Cancel
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  );
}
