import { useEffect, useState } from 'react';
import { MapContainer, TileLayer, GeoJSON, useMap } from 'react-leaflet';
import L from 'leaflet';
import 'leaflet/dist/leaflet.css';

interface GeoJsonFeature {
  type: string;
  geometry: {
    type: string;
    coordinates: number[][];
  };
}

interface ActivityMapProps {
  activityId: string;
  height?: string;
}

// Component to fit map bounds to route
function FitBounds({ coordinates }: { coordinates: number[][] }) {
  const map = useMap();

  useEffect(() => {
    if (coordinates.length > 0) {
      const bounds = L.latLngBounds(
        coordinates.map((c) => [c[1], c[0]] as [number, number])
      );
      map.fitBounds(bounds, { padding: [20, 20] });
    }
  }, [map, coordinates]);

  return null;
}

export function ActivityMap({ activityId, height = '400px' }: ActivityMapProps) {
  const [geoJson, setGeoJson] = useState<GeoJsonFeature | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch(`/api/activities/${activityId}/geojson`)
      .then((res) => {
        if (!res.ok) throw new Error('Failed to load route');
        return res.json();
      })
      .then((data) => setGeoJson(data as GeoJsonFeature))
      .catch((err) => setError(err.message));
  }, [activityId]);

  if (error) {
    return <div style={{ height, display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#666' }}>Error: {error}</div>;
  }

  if (!geoJson) {
    return <div style={{ height, display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#666' }}>Loading map...</div>;
  }

  const coordinates = geoJson.geometry.coordinates;

  return (
    <MapContainer
      style={{ height, width: '100%' }}
      zoom={13}
      scrollWheelZoom={false}
    >
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      <FitBounds coordinates={coordinates} />
      <GeoJSON
        data={geoJson as GeoJsonFeature & { type: 'Feature' }}
        style={{
          color: '#3388ff',
          weight: 4,
          opacity: 0.8,
        }}
      />
    </MapContainer>
  );
}
