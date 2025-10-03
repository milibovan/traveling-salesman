import React from "react";
import "./SelectedCities.css";
import type { MarkerData, RouteResponse } from "./utils/utils";

interface MarkerListProps {
  markers: MarkerData[];
  onRouteCalculated: (response: RouteResponse) => void;
  setMarkers: React.Dispatch<React.SetStateAction<MarkerData[]>>;
  setCounter: React.Dispatch<React.SetStateAction<number>>;
}

const SelectedCities: React.FC<MarkerListProps> = ({
  markers,
  onRouteCalculated,
  setMarkers,
  setCounter,
}) => {
  const handleSubmit = () => {
    const request = new XMLHttpRequest();

    request.addEventListener("load", () => {
      const routeData = JSON.parse(request.responseText);
      console.log(routeData);

      onRouteCalculated(routeData);
    });

    request.open("POST", "http://localhost:8080/calculate-tour");
    request.setRequestHeader("Content-Type", "application/json");
    request.send(JSON.stringify(markers));
  };

  const handleDelete = () => {
    if (window.confirm("Are you sure you want to clear all selected stops?")) {
      setMarkers([]);
      setCounter(1);
    }
  };

  return (
    <div className="marker-list-sidebar">
      <h2>Selected Stops ({markers.length})</h2>

      {markers.length === 0 ? (
        <p className="marker-list-empty-text">
          Click on the map or search for a city to add a stop.
        </p>
      ) : (
        <ol className="marker-list-ordered">
          {markers.map((marker, index) => (
            <li key={index} className="marker-list-item">
              <strong>Stop {index + 1}:</strong> {marker.label}
              <br />
              <span className="marker-list-coords">
                ({marker.coordinates[0].toFixed(3)},{" "}
                {marker.coordinates[1].toFixed(3)})
              </span>
            </li>
          ))}
        </ol>
      )}

      {markers.length > 0 && (
        <>
          <button className="submit-route-button" onClick={handleSubmit}>
            Submit Route
          </button>
          <button className="delete-route-button" onClick={handleDelete}>
            Delete stops
          </button>
        </>
      )}
    </div>
  );
};

export default SelectedCities;
