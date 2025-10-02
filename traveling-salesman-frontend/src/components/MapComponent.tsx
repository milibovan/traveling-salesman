import React, { useState, useRef } from "react";
import {
  MapContainer,
  TileLayer,
  Marker,
  Popup,
  useMapEvents,
} from "react-leaflet";
import "leaflet/dist/leaflet.css";
import "leaflet-geosearch/dist/geosearch.css";
import L from "leaflet";
import SearchControlComponent from "./SearchControl";
import SelectedCities from "./SelectedCities";
import "./MapComponent.css"

// Leaflet Icon Fix (Kept from original code)
delete (L.Icon.Default.prototype as any)._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl:
    "https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon-2x.png",
  iconUrl: "https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon.png",
  shadowUrl: "https://unpkg.com/leaflet@1.7.1/dist/images/marker-shadow.png",
});

type LatLngTuple = L.LatLngTuple;

// New Marker Data Structure
type MarkerData = {
  coordinates: LatLngTuple;
  label: string; // To store the city name/description
};

// ----------------------------------------------------------------------
// 1. Click Handler Component (UPDATED to use MarkerData)
// ----------------------------------------------------------------------
const ClickHandler: React.FC<{
  setMarkers: React.Dispatch<React.SetStateAction<MarkerData[]>>;
}> = ({ setMarkers }) => {
  // 1. Create a persistent counter using useRef
  const clickCount = useRef(0);

  useMapEvents({
    click(e) {
      const newMarkerPosition: LatLngTuple = [e.latlng.lat, e.latlng.lng];

      clickCount.current += 1;

      const newMarker: MarkerData = {
        coordinates: newMarkerPosition,
        label: `Manual Click ${clickCount.current}`, // <-- Updated Label
      };

      setMarkers((prevMarkers) => [...prevMarkers, newMarker]);
      // console.log("New Marker Added (Click):", newMarker);
    },
  });

  return null;
};

// ----------------------------------------------------------------------
// 3. Main Map Component (Render and List)
// ----------------------------------------------------------------------
interface MapComponentProps {}

const MapComponent: React.FC<MapComponentProps> = () => {
  // Use the new MarkerData array state
  const [markers, setMarkers] = useState<MarkerData[]>([]);
  const initialPosition: LatLngTuple = [45.257, 19.842];
  const initialZoom: number = 13;

  // Logging (now logs the label too)
//   if (markers.length > 0) {
//     markers.forEach((marker, index) => {
//       console.log(
//         `Marker ${index + 1} (${
//           marker.label
//         }): Lat: ${marker.position[0].toFixed(
//           3
//         )}, Lng: ${marker.position[1].toFixed(3)}`
//       );
//     });
//   }

  return (
    // Use a flex container to place the map and list side-by-side
    <div className="map-container">
      {/* --- Map Container (Left Side) --- */}
      <MapContainer
        center={initialPosition}
        zoom={initialZoom}
        scrollWheelZoom={true}
        style={{ height: "60vh", width: "70%", minWidth: "400px" }} // Adjusted width
      >
        <SearchControlComponent setMarkers={setMarkers} />

        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />

        <ClickHandler setMarkers={setMarkers} />

        {/* Render Markers */}
        {markers.map((marker, index) => (
          <Marker key={index} position={marker.coordinates}>
            <Popup>
              **{marker.label}** <br />
              Lat: {marker.coordinates[0].toFixed(3)}, Lng:{" "}
              {marker.coordinates[1].toFixed(3)}
            </Popup>
          </Marker>
        ))}
      </MapContainer>
        <SelectedCities markers={markers}/>
    </div>
  );
};

export default MapComponent;
