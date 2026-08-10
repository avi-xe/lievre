import { useEffect, useState } from 'react';

interface ElevationProfileProps {
  activityId: string;
  height?: string;
}

interface ElevationData {
  distance: number;
  elevation: number;
}

export function ElevationProfile({ activityId, height = '200px' }: ElevationProfileProps) {
  const [data, setData] = useState<ElevationData[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [hoverIndex, setHoverIndex] = useState<number | null>(null);

  useEffect(() => {
    fetch(`/api/activities/${activityId}/geojson`)
      .then((res) => {
        if (!res.ok) throw new Error('Failed to load route');
        return res.json();
      })
      .then((geojson) => {
        const coords = geojson.geometry.coordinates;
        const elevationData: ElevationData[] = [];
        let totalDistance = 0;

        for (let i = 0; i < coords.length; i++) {
          if (i > 0) {
            // Calculate distance from previous point (simplified Haversine)
            const [lon1, lat1] = coords[i - 1];
            const [lon2, lat2] = coords[i];
            const R = 6371000; // Earth radius in meters
            const dLat = ((lat2 - lat1) * Math.PI) / 180;
            const dLon = ((lon2 - lon1) * Math.PI) / 180;
            const a =
              Math.sin(dLat / 2) * Math.sin(dLat / 2) +
              Math.cos((lat1 * Math.PI) / 180) *
                Math.cos((lat2 * Math.PI) / 180) *
                Math.sin(dLon / 2) *
                Math.sin(dLon / 2);
            const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
            totalDistance += R * c;
          }

          elevationData.push({
            distance: totalDistance / 1000, // Convert to km
            elevation: coords[i][2] || 0,
          });
        }

        setData(elevationData);
      })
      .catch((err) => setError(err.message));
  }, [activityId]);

  if (error) {
    return <div style={{ height, display: 'flex', alignItems: 'center', color: '#666' }}>Error: {error}</div>;
  }

  if (data.length === 0) {
    return <div style={{ height, display: 'flex', alignItems: 'center', color: '#666' }}>Loading elevation profile...</div>;
  }

  const maxElevation = Math.max(...data.map((d) => d.elevation));
  const minElevation = Math.min(...data.map((d) => d.elevation));
  const maxDistance = data[data.length - 1].distance;

  const svgWidth = 800;
  const svgHeight = 150;
  const padding = { top: 10, right: 10, bottom: 30, left: 50 };
  const chartWidth = svgWidth - padding.left - padding.right;
  const chartHeight = svgHeight - padding.top - padding.bottom;

  const xScale = (distance: number) => padding.left + (distance / maxDistance) * chartWidth;
  const yScale = (elevation: number) =>
    padding.top + chartHeight - ((elevation - minElevation) / (maxElevation - minElevation || 1)) * chartHeight;

  // Create path for elevation profile
  const pathD = data
    .map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(d.distance)} ${yScale(d.elevation)}`)
    .join(' ');

  // Create area path for gradient fill
  const areaD = `${pathD} L ${xScale(maxDistance)} ${yScale(minElevation)} L ${padding.left} ${yScale(minElevation)} Z`;

  const hoveredPoint = hoverIndex !== null ? data[hoverIndex] : null;

  return (
    <div style={{ height, position: 'relative' }}>
      <svg
        width="100%"
        height="100%"
        viewBox={`0 0 ${svgWidth} ${svgHeight}`}
        preserveAspectRatio="none"
        onMouseLeave={() => setHoverIndex(null)}
      >
        {/* Grid lines */}
        {[0, 0.25, 0.5, 0.75, 1].map((tick) => (
          <line
            key={tick}
            x1={padding.left}
            y1={padding.top + chartHeight * tick}
            x2={svgWidth - padding.right}
            y2={padding.top + chartHeight * tick}
            stroke="#eee"
            strokeWidth="1"
          />
        ))}

        {/* Elevation area with gradient */}
        <defs>
          <linearGradient id="elevationGradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stopColor="#3388ff" stopOpacity="0.6" />
            <stop offset="100%" stopColor="#3388ff" stopOpacity="0.1" />
          </linearGradient>
        </defs>
        <path d={areaD} fill="url(#elevationGradient)" />

        {/* Elevation line */}
        <path d={pathD} fill="none" stroke="#3388ff" strokeWidth="2" />

        {/* Hover indicator */}
        {hoveredPoint && (
          <>
            <line
              x1={xScale(hoveredPoint.distance)}
              y1={padding.top}
              x2={xScale(hoveredPoint.distance)}
              y2={padding.top + chartHeight}
              stroke="#ff4444"
              strokeWidth="1"
              strokeDasharray="4,4"
            />
            <circle
              cx={xScale(hoveredPoint.distance)}
              cy={yScale(hoveredPoint.elevation)}
              r="4"
              fill="#ff4444"
            />
          </>
        )}

        {/* Y-axis labels */}
        <text x={padding.left - 5} y={padding.top + 5} textAnchor="end" fontSize="10" fill="#666">
          {maxElevation.toFixed(0)}m
        </text>
        <text x={padding.left - 5} y={padding.top + chartHeight} textAnchor="end" fontSize="10" fill="#666">
          {minElevation.toFixed(0)}m
        </text>

        {/* X-axis labels */}
        <text x={padding.left} y={svgHeight - 5} fontSize="10" fill="#666">
          0 km
        </text>
        <text x={svgWidth - padding.right} y={svgHeight - 5} textAnchor="end" fontSize="10" fill="#666">
          {maxDistance.toFixed(1)} km
        </text>
      </svg>

      {/* Invisible hover area */}
      <div
        style={{
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
        }}
        onMouseMove={(e) => {
          const rect = e.currentTarget.getBoundingClientRect();
          const x = e.clientX - rect.left;
          const percentage = x / rect.width;
          const distance = percentage * maxDistance;
          const index = data.findIndex((d) => d.distance >= distance);
          setHoverIndex(index >= 0 ? index : data.length - 1);
        }}
      />

      {/* Tooltip */}
      {hoveredPoint && (
        <div
          style={{
            position: 'absolute',
            top: '10px',
            right: '10px',
            background: 'white',
            padding: '8px',
            borderRadius: '4px',
            boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
            fontSize: '12px',
          }}
        >
          <div><strong>{hoveredPoint.elevation.toFixed(0)}m</strong></div>
          <div style={{ color: '#666' }}>{hoveredPoint.distance.toFixed(2)} km</div>
        </div>
      )}
    </div>
  );
}
