export interface User {
  id: string;
  email: string;
  username: string;
}

export interface Activity {
  id: string;
  user_id: string;
  activity_type: string;
  title: string;
  description: string | null;
  started_at: string;
  duration_seconds: number | null;
  distance_meters: number | null;
  elevation_gain_meters: number | null;
  visibility: string;
  created_at: string;
  updated_at: string;
}

export interface Comment {
  id: string;
  activity_id: string;
  user_id: string;
  content: string;
  username: string;
  created_at: string;
}

export interface FeedItem extends Activity {
  username?: string;
}
