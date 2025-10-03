import { OpenStreetMapProvider, GeoSearchControl } from "leaflet-geosearch";
import { useMap } from "react-leaflet";
import React from "react";
import "./SearchContorl.css";
import type { MarkerData } from "./utils/utils";

const SearchControl: React.FC<{
  setMarkers: React.Dispatch<React.SetStateAction<MarkerData[]>>;
}> = ({ setMarkers }) => {
  const map = useMap();
  const searchControlRef = React.useRef<L.Control | null>(null);

  React.useEffect(() => {
    if (searchControlRef.current === null) {
      const provider = new OpenStreetMapProvider();

      searchControlRef.current = new (GeoSearchControl as any)({
        provider: provider,
        style: "button",
        showMarker: false,
        showPopup: false,
        autoClose: true,
        searchLabel: "Search city or location",
        keepResult: true,
        position: "topright", // <-- Changed to topright for better visibility
      });

      map.addControl(searchControlRef.current as L.Control);
    }

    const eventHandler = (result: any) => {
      const { x, y, label } = result.location;
      const newMarker: MarkerData = {
        coordinates: [y, x],
        label: label || `Marker ${Date.now()}`,
      };
      setMarkers((prevMarkers) => [...prevMarkers, newMarker]);
    };

    map.on("geosearch/showlocation", eventHandler);

    return () => {
      map.off("geosearch/showlocation", eventHandler);
    };
  }, [map, setMarkers]);

  return null;
};

export default SearchControl;
