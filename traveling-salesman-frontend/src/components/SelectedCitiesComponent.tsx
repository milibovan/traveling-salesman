import React from "react";
import L from "leaflet";
import "./SelectedCitiesComponent.css"; // <-- Import the new CSS file

// Re-defining the types needed for the props (assuming these types are available/imported)
type LatLngTuple = L.LatLngTuple;
type MarkerData = {
  position: LatLngTuple;
  label: string;
};

interface MarkerListProps {
  markers: MarkerData[];
  // If you pass a submission handler, its type should be added here:
  // onSubmit: () => void;
}

const SelectedCitiesComponent: React.FC<MarkerListProps> = ({ markers }) => {
  // Assuming the submission logic is handled inside this component for now
  const handleSubmit = () => {
    console.log("Submitting route for", markers.length, "stops...");
    // Add actual submission logic here
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
                ({marker.position[0].toFixed(3)},{" "}
                {marker.position[1].toFixed(3)})
              </span>
            </li>
          ))}
        </ol>
      )}

      {markers.length > 0 && (
        <button className="submit-route-button" onClick={handleSubmit}>
          Submit Route
        </button>
      )}
    </div>
  );
};

export default SelectedCitiesComponent;
