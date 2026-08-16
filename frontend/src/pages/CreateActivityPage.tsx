import { useState, useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { apiFetch, apiUpload } from "../lib/api";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import { Separator } from "@/components/ui/separator";
import { Alert, AlertDescription } from "@/components/ui/alert";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectPopup,
  SelectItem,
} from "@/components/ui/select";
import {
  Loader2,
  Upload,
  FileUp,
  X,
  Timer,
  Mountain,
  Ruler,
} from "lucide-react";

const ACTIVITY_TYPES = [
  { value: "ride", label: "Ride" },
  { value: "run", label: "Run" },
  { value: "swim", label: "Swim" },
  { value: "walk", label: "Walk" },
  { value: "hike", label: "Hike" },
] as const;

const VISIBILITY_OPTIONS = [
  { value: "public", label: "Public" },
  { value: "followers", label: "Followers" },
  { value: "private", label: "Private" },
] as const;

export function CreateActivityPage() {
  const navigate = useNavigate();

  const [form, setForm] = useState({
    activity_type: "ride",
    title: "",
    started_at: "",
    duration_seconds: "",
    distance_meters: "",
    elevation_gain_meters: "",
    visibility: "public",
  });

  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [file, setFile] = useState<File | null>(null);
  const [uploading, setUploading] = useState(false);
  const [dragOver, setDragOver] = useState(false);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setForm({ ...form, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      const payload = {
        activity_type: form.activity_type,
        title: form.title,
        started_at: form.started_at,
        duration_seconds: form.duration_seconds
          ? Number(form.duration_seconds)
          : null,
        distance_meters: form.distance_meters
          ? Number(form.distance_meters)
          : null,
        elevation_gain_meters: form.elevation_gain_meters
          ? Number(form.elevation_gain_meters)
          : null,
        visibility: form.visibility,
      };

      const res = await apiFetch<{ id: string }>("/api/activities", {
        method: "POST",
        body: JSON.stringify(payload),
      });
      navigate(`/activities/${res.id}`);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to create activity"
      );
    } finally {
      setLoading(false);
    }
  };

  const handleFileUpload = async () => {
    if (!file) return;
    setUploading(true);
    setError("");
    try {
      const res = await apiUpload<{ activity_id: string }>(
        "/api/import/gpx",
        file
      );
      navigate(`/activities/${res.activity_id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Upload failed");
    } finally {
      setUploading(false);
    }
  };

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    const dropped = e.dataTransfer.files[0];
    if (dropped?.name.endsWith(".gpx")) {
      setFile(dropped);
    }
  }, []);

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selected = e.target.files?.[0];
    if (selected) setFile(selected);
  };

  if (loading || uploading) {
    return (
      <div className="mx-auto max-w-2xl px-4 py-8">
        <Card>
          <CardHeader>
            <Skeleton className="h-8 w-48" />
          </CardHeader>
          <CardContent className="space-y-4">
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-20 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-2xl px-4 py-8">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileUp className="h-5 w-5 text-primary" />
            New Activity
          </CardTitle>
        </CardHeader>

        <CardContent className="space-y-6">
          {error && (
            <Alert variant="destructive">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          {/* GPX Upload Section */}
          <div className="space-y-2">
            <Label className="text-base font-semibold">Upload GPX File</Label>
            <div
              onDragOver={handleDragOver}
              onDragLeave={handleDragLeave}
              onDrop={handleDrop}
              onClick={() => document.getElementById("gpx-file-input")?.click()}
              className={`flex cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed p-6 text-center transition-colors ${
                dragOver
                  ? "border-primary bg-primary/5"
                  : "border-muted-foreground/25 hover:border-primary/50"
              }`}
              role="button"
              tabIndex={0}
              aria-label="Drop GPX file here or click to browse"
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  document.getElementById("gpx-file-input")?.click();
                }
              }}
            >
              <Upload className="mb-2 h-8 w-8 text-muted-foreground" />
              <p className="text-sm text-muted-foreground">
                Drag & drop a <span className="font-medium">.gpx</span> file
                here, or click to browse
              </p>
              <input
                id="gpx-file-input"
                type="file"
                accept=".gpx"
                onChange={handleFileSelect}
                className="hidden"
                aria-hidden="true"
              />
            </div>

            {file && (
              <div className="flex items-center justify-between rounded-md bg-muted px-3 py-2">
                <span className="truncate text-sm font-medium">
                  {file.name}
                </span>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  onClick={() => setFile(null)}
                  aria-label={`Remove ${file.name}`}
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            )}

            {file && (
              <Button
                onClick={handleFileUpload}
                disabled={uploading}
                className="w-full"
              >
                {uploading ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Uploading...
                  </>
                ) : (
                  <>
                    <Upload className="mr-2 h-4 w-4" />
                    Upload GPX
                  </>
                )}
              </Button>
            )}
          </div>

          <div className="relative my-4">
            <div className="absolute inset-0 flex items-center">
              <Separator className="w-full" />
            </div>
            <div className="relative flex justify-center text-xs uppercase">
              <span className="bg-card px-2 text-muted-foreground">or</span>
            </div>
          </div>

          {/* Manual Entry Form */}
          <form id="create-activity-form" onSubmit={handleSubmit} className="space-y-4">
            <div className="space-y-2">
              <Label>Activity Type</Label>
              <Select
                value={form.activity_type}
                onValueChange={(v: string) =>
                  setForm((prev) => ({ ...prev, activity_type: v }))
                }
              >
                <SelectTrigger className="w-full">
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

            <div className="space-y-2">
              <Label htmlFor="title">Title</Label>
              <Input
                id="title"
                name="title"
                placeholder="Morning ride through the park"
                value={form.title}
                onChange={handleChange}
                required
              />
            </div>

            <div className="space-y-2">
              <Label
                htmlFor="started_at"
                className="flex items-center gap-1.5"
              >
                Start Time
              </Label>
              <Input
                id="started_at"
                name="started_at"
                type="datetime-local"
                value={form.started_at}
                onChange={handleChange}
                required
              />
            </div>

            <Separator />

            <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div className="space-y-2">
                <Label
                  htmlFor="duration_seconds"
                  className="flex items-center gap-1.5"
                >
                  <Timer className="h-3.5 w-3.5 text-muted-foreground" />
                  Duration (s)
                </Label>
                <Input
                  id="duration_seconds"
                  name="duration_seconds"
                  type="number"
                  min="0"
                  placeholder="0"
                  value={form.duration_seconds}
                  onChange={handleChange}
                />
              </div>

              <div className="space-y-2">
                <Label
                  htmlFor="distance_meters"
                  className="flex items-center gap-1.5"
                >
                  <Ruler className="h-3.5 w-3.5 text-muted-foreground" />
                  Distance (m)
                </Label>
                <Input
                  id="distance_meters"
                  name="distance_meters"
                  type="number"
                  min="0"
                  step="0.1"
                  placeholder="0"
                  value={form.distance_meters}
                  onChange={handleChange}
                />
              </div>

              <div className="space-y-2">
                <Label
                  htmlFor="elevation_gain_meters"
                  className="flex items-center gap-1.5"
                >
                  <Mountain className="h-3.5 w-3.5 text-muted-foreground" />
                  Elevation (m)
                </Label>
                <Input
                  id="elevation_gain_meters"
                  name="elevation_gain_meters"
                  type="number"
                  min="0"
                  step="0.1"
                  placeholder="0"
                  value={form.elevation_gain_meters}
                  onChange={handleChange}
                />
              </div>
            </div>

            <div className="space-y-2">
              <Label>Visibility</Label>
              <Select
                value={form.visibility}
                onValueChange={(v: string) =>
                  setForm((prev) => ({ ...prev, visibility: v }))
                }
              >
                <SelectTrigger className="w-full">
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

            <CardFooter className="flex justify-end gap-2 px-0 pt-2">
              <Button
                type="button"
                variant="outline"
                onClick={() => navigate(-1)}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={loading}>
                {loading ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Creating...
                  </>
                ) : (
                  "Create Activity"
                )}
              </Button>
            </CardFooter>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
