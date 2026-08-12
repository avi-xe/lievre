import { useEffect, useState, useRef } from 'react';
import { apiFetch } from '../lib/api';

interface GeoJSONResponse {
  type: string;
  features: Array<{
    geometry: {
      type: string;
      coordinates: [number, number, number][];
    };
  }>;
}

function haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const R = 6371000;
  const dLat = ((lat2 - lat1) * Math.PI) / 180;
  const dLon = ((lon2 - lon1) * Math.PI) / 180;
  const a = Math.sin(dLat / 2) ** 2 + Math.cos((lat1 * Math.PI) / 180) * Math.cos((lat2 * Math.PI) / 180) * Math.sin(dLon / 2) ** 2;
  return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

export function ElevationProfile({ activityId, height = '200px' }: { activityId: string; height?: string }) {
  const [points, setPoints] = useState<{ distance: number; elevation: number }[]>([]);
  const [loading, setLoading] = useState(true);
  const [hovered, setHovered] = useState<{ x: number; y: number; distance: number; elevation: number } | null>(null);
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    apiFetch<GeoJSONResponse>(`/api/activities/${activityId}/geojson`)
      .then(geojson => {
        const coords = geojson.features?.[0]?.geometry?.coordinates;
        if (!coords?.length) return;
        const hasElev = coords.some(c => c.length >= 3 && c[2] != null);
        if (!hasElev) return;

        let cumDist = 0;
        const pts = coords.map((c, i) => {
          if (i > 0) cumDist += haversine(coords[i - 1][1], coords[i - 1][0], c[1], c[0]);
          return { distance: cumDist, elevation: c[2] || 0 };
        });
        setPoints(pts);
      })
      .catch(() => {})
      .finally(() => setLoading(false));
  }, [activityId]);

  if (loading) return <div style={{ height, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#f5f5f5' }}>Loading elevation...</div>;
  if (points.length === 0) return <div style={{ height, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#f5f5f5', color: '#999' }}>No elevation data</div>;

  const maxDist = points[points.length - 1].distance;
  const maxElev = Math.max(...points.map(p => p.elevation));
  const minElev = Math.min(...points.map(p => p.elevation));
  const elevRange = maxElev - minElev || 1;
  const w = 800;
  const h = 150;
  const padL = 40;
  const padB = 20;

  const pathData = points.map((p, i) => {
    const x = padL + (p.distance / maxDist) * (w - padL);
    const y = h - padB - ((p.elevation - minElev) / elevRange) * (h - padB);
    return `${i === 0 ? 'M' : 'L'}${x},${y}`;
  }).join(' ');

  return (
    <div style={{ background: '#f5f5f5', borderRadius: 4, padding: '10px 0', position: 'relative' }}>
      <svg
        ref={svgRef}
        viewBox={`0 0 ${w} ${h}`}
        style={{ width: '100%', height }}
        onMouseMove={e => {
          if (!svgRef.current) return;
          const rect = svgRef.current.getBoundingClientRect();
          const mouseX = ((e.clientX - rect.left) / rect.width) * w;
          let nearest = points[0];
          let minDist = Infinity;
          points.forEach((p) => {
            const ptx = padL + (p.distance / maxDist) * (w - padL);
            const d = Math.abs(ptx - mouseX);
            if (d < minDist) { minDist = d; nearest = p; }
          });
          const py = h - padB - ((nearest.elevation - minElev) / elevRange) * (h - padB);
          setHovered({ x: e.clientX, y: rect.top + (py / h) * rect.height, distance: nearest.distance, elevation: nearest.elevation });
        }}
        onMouseLeave={() => setHovered(null)}
      >
        <path d={pathData} fill="none" stroke="#333" strokeWidth="1.5" />
        <text x={padL / 2} y={h - padB} fontSize="10" textAnchor="middle">{Math.round(minElev)}m</text>
        <text x={padL / 2} y={12} fontSize="10" textAnchor="middle">{Math.round(maxElev)}m</text>
        <text x={w / 2} y={h - 2} fontSize="10" textAnchor="middle">{(maxDist / 1000).toFixed(1)} km</text>
        {hovered && (
          <>
            <line
              x1={padL + (hovered.distance / maxDist) * (w - padL)}
              y1={0}
              x2={padL + (hovered.distance / maxDist) * (w - padL)}
              y2={h - padB}
              stroke="#999"
              strokeWidth="1"
              strokeDasharray="4"
            />
            <circle
              cx={padL + (hovered.distance / maxDist) * (w - padL)}
              cy={h - padB - ((hovered.elevation - minElev) / elevRange) * (h - padB)}
              r={4}
              fill="#333"
            />
          </>
        )}
      </svg>
      {hovered && (
        <div
          style={{
            position: 'fixed',
            left: hovered.x + 12,
            top: hovered.y - 40,
            background: '#333',
            color: 'white',
            padding: '4px 8px',
            borderRadius: 4,
            fontSize: 12,
            pointerEvents: 'none',
            whiteSpace: 'nowrap',
            zIndex: 10,
          }}
        >
          {(hovered.distance / 1000).toFixed(2)} km · {Math.round(hovered.elevation)} m
        </div>
      )}
    </div>
  );
}
