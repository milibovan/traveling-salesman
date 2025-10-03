import React, { useState, useRef } from "react";
import {
  MapContainer,
  TileLayer,
  Marker,
  Popup,
  useMapEvents,
  Polyline,
} from "react-leaflet";
import "leaflet/dist/leaflet.css";
import "leaflet-geosearch/dist/geosearch.css";
import L from "leaflet";
import SearchControlComponent from "./SearchControl";
import SelectedCities from "./SelectedCities";
import "./MapComponent.css";
import "leaflet-polylinedecorator";
import DrawArrows from "./DrawArrows";
import type { LatLngTuple, MarkerData } from "./utils/utils";

// Leaflet Icon Fix (Kept from original code)
delete (L.Icon.Default.prototype as any)._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl:
    "https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon-2x.png",
  iconUrl: "https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon.png",
  shadowUrl: "https://unpkg.com/leaflet@1.7.1/dist/images/marker-shadow.png",
});

// ----------------------------------------------------------------------
// 1. Click Handler Component (UPDATED to use MarkerData)
// ----------------------------------------------------------------------
const ClickHandler: React.FC<{
  setMarkers: React.Dispatch<React.SetStateAction<MarkerData[]>>;
  counter: number;
  setCounter: React.Dispatch<React.SetStateAction<number>>;
}> = ({ setMarkers, counter, setCounter }) => {

  useMapEvents({
    click(e) {
      const newMarkerPosition: LatLngTuple = [e.latlng.lat, e.latlng.lng];

      const currentCounter = counter + 1;
      setCounter(currentCounter)

      const newMarker: MarkerData = {
        coordinates: newMarkerPosition,
        label: `Manual Click ${counter}`,
      };

      setMarkers((prevMarkers) => [...prevMarkers, newMarker]);
      // console.log("New Marker Added (Click):", newMarker);
    },
  });

  return null;
};

interface MapComponentProps {}

const MapComponent: React.FC<MapComponentProps> = () => {
  // Use the new MarkerData array state
  const [markers, setMarkers] = useState<MarkerData[]>([]);
  const [orderedMarkers, setOrderedMarkers] = useState<MarkerData[]>([]);
  const [counter, setCounter] = useState<number>(1);
  const initialPosition: LatLngTuple = [45.257, 19.842];
  const initialZoom: number = 13;

  const onRouteCalculated = (routeData: {
    cities: string[];
    total_distance: number;
  }) => {
    const cityMap = new Map(markers.map((m) => [m.label, m]));
    const newOrder = routeData.cities
      .map((city) => cityMap.get(city))
      .filter((m): m is MarkerData => !!m);

    setOrderedMarkers(newOrder);
    window.alert("Shortest route is: " + routeData.total_distance + "km")
  };

  const polylineCoordinates = orderedMarkers.length > 0
    ? orderedMarkers.map((m) => m.coordinates)
    : [];

  // If there are markers, add the first one again to close the loop
  if (polylineCoordinates.length > 0 && orderedMarkers.length > 1) {
    polylineCoordinates.push(polylineCoordinates[0]);
  }

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

        <ClickHandler setMarkers={setMarkers} counter={counter} setCounter={setCounter}/>

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

        {orderedMarkers.length > 1 && (
          <Polyline
            // Use the closed loop coordinates
            positions={polylineCoordinates}
            pathOptions={{ color: "blue", weight: 3 }}
          />
        )}

        {/* Arrowheads (decorators) */}
        {orderedMarkers.length > 1 && (
          <DrawArrows 
            // Use the closed loop coordinates
            coordinates={polylineCoordinates} 
          />
        )}
      </MapContainer>
      <SelectedCities markers={markers} onRouteCalculated={onRouteCalculated} setMarkers={setMarkers} setCounter={setCounter}/>
    </div>
  );
};

export default MapComponent;