import { OpenStreetMapProvider, GeoSearchControl } from "leaflet-geosearch";
import { useMap } from "react-leaflet";
import React from "react";
import './SearchContorlComponent.css'

type LatLngTuple = L.LatLngTuple;

const SearchControlComponent: React.FC<{
  setMarkers: React.Dispatch<React.SetStateAction<LatLngTuple[]>>;
}> = ({ setMarkers }) => {
  const map = useMap();
  const searchControlRef = React.useRef<L.Control | null>(null);

  // useEffect for initializing the control and listeners
  React.useEffect(() => {
    // --- 1. Control Initialization (Only happens once) ---
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

    // --- 2. Event Listener Setup ---
    const eventHandler = (result: any) => {
      const { x, y, label } = result.location;
      const newMarkerPosition: LatLngTuple = [y, x];
      setMarkers((prevMarkers) => [...prevMarkers, newMarkerPosition]);
      console.log(`New Marker Added (Search): ${label}`, newMarkerPosition);
    };

    map.on("geosearch/showlocation", eventHandler);

    // --- 4. Cleanup Function ---
    return () => {
      map.off("geosearch/showlocation", eventHandler);
    };
  }, [map, setMarkers]);

  return null;
};

export default SearchControlComponent;
