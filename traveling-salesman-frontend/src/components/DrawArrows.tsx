import { useEffect } from "react";
import { useMap } from "react-leaflet";
import L from "leaflet";
import "leaflet-polylinedecorator";
import type { LatLngTuple } from "./utils/utils";

interface DrawArrowsProps {
  coordinates: LatLngTuple[];
}

const DrawArrows: React.FC<DrawArrowsProps> = ({ coordinates }) => {
  const map = useMap();

  useEffect(() => {
    if (coordinates.length < 2) return;

    const polyline = L.polyline(coordinates);
    const decorator = L.polylineDecorator(polyline, {
      patterns: [
        {
          offset: 25,
          repeat: 50,
          symbol: L.Symbol.arrowHead({
            pixelSize: 10,
            polygon: false,
            pathOptions: { stroke: true, color: "blue" },
          }),
        },
      ],
    });

    polyline.addTo(map);
    decorator.addTo(map);

    return () => {
      map.removeLayer(polyline);
      map.removeLayer(decorator);
    };
  }, [coordinates, map]);

  return null;
};

export default DrawArrows;
