import React, { useEffect, useState } from 'react';
import { MapContainer, TileLayer, GeoJSON, useMap } from 'react-leaflet';
import 'leaflet/dist/leaflet.css';
import { apiFetch } from '../lib/api';

function FitBounds({ coordinates }: { coordinates: [number, number][] }) {
  const map = useMap();
  useEffect(() => {
    if (coordinates.length > 0) {
      map.fitBounds(coordinates, { padding: [20, 20] });
    }
  }, [map, coordinates]);
  return null;
}

interface GeoJSONFeature {
  type: string;
  geometry: {
    type: string;
    coordinates: [number, number][] | [number, number, number][];
  };
}

interface GeoJSONResponse {
  type: string;
  features: GeoJSONFeature[];
}

export function ActivityMap({ activityId, height = '400px' }: { activityId: string; height?: string }) {
  const [geojson, setGeojson] = useState<GeoJSONResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    setLoading(true);
    apiFetch<GeoJSONResponse>(`/api/activities/${activityId}/geojson`)
      .then(setGeojson)
      .catch(err => setError(err.message))
      .finally(() => setLoading(false));
  }, [activityId]);

  if (loading) return <div style={{ height, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#f5f5f5' }}>Loading map...</div>;
  if (error) return <div style={{ height, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#f5f5f5', color: '#999' }}>No route data</div>;
  if (!geojson?.features?.length) return <div style={{ height, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#f5f5f5', color: '#999' }}>No route data</div>;

  const feature = geojson.features[0];
  const coords = feature.geometry.coordinates.map(c => [c[1], c[0]] as [number, number]);

  return (
    <div style={{ height, borderRadius: 4, overflow: 'hidden', border: '1px solid #ddd' }}>
      <MapContainer style={{ height: '100%', width: '100%' }} center={[0, 0]} zoom={13}>
        <TileLayer attribution='&copy; OpenStreetMap' url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
        <GeoJSON data={geojson as unknown as GeoJSONResponse} />
        <FitBounds coordinates={coords} />
      </MapContainer>
    </div>
  );
}
