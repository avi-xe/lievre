// Shared activity type helpers — icons, colors, labels
import { Bike, Footprints, Waves, TreePine } from "lucide-react";
import type { LucideIcon } from "lucide-react";

export const activityIcons: Record<string, LucideIcon> = {
  ride: Bike,
  run: Footprints,
  swim: Waves,
  walk: Footprints,
  hike: TreePine,
};

export const activityColors: Record<string, string> = {
  ride: "bg-ride/10 text-ride",
  run: "bg-run/10 text-run",
  swim: "bg-swim/10 text-swim",
  walk: "bg-walk/10 text-walk",
  hike: "bg-hike/10 text-hike",
};

export const activityLabels: Record<string, string> = {
  ride: "Ride",
  run: "Run",
  swim: "Swim",
  walk: "Walk",
  hike: "Hike",
};
