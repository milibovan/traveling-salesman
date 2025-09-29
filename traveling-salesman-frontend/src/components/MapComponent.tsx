import React, { useState } from 'react';
import { MapContainer, TileLayer, Marker, Popup, useMapEvents } from 'react-leaflet';
import 'leaflet/dist/leaflet.css';
import 'leaflet-geosearch/dist/geosearch.css';
import L from 'leaflet';
import SearchControlComponent from './SearchControlComponent';

// Leaflet Icon Fix (Kept from original code)
delete (L.Icon.Default.prototype as any)._getIconUrl;

L.Icon.Default.mergeOptions({
    iconRetinaUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon-2x.png',
    iconUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon.png',
    shadowUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-shadow.png',
});

type LatLngTuple = L.LatLngTuple;

// ----------------------------------------------------------------------
// 1. Click Handler Component (Unchanged)
// ----------------------------------------------------------------------
const ClickHandler: React.FC<{ setMarkers: React.Dispatch<React.SetStateAction<LatLngTuple[]>> }> = ({ setMarkers }) => {
    useMapEvents({
        click(e) {
            const newMarkerPosition: LatLngTuple = [e.latlng.lat, e.latlng.lng];
            setMarkers(prevMarkers => [...prevMarkers, newMarkerPosition]);
            console.log('New Marker Added (Click):', newMarkerPosition);
        },
    });

    return null; 
};

// ----------------------------------------------------------------------
// 2. Search Control Component (Final Fix for Visibility)
// ----------------------------------------------------------------------

// ----------------------------------------------------------------------
// 3. Main Map Component (UPDATED to pass setMarkers to SearchControlComponent)
// ----------------------------------------------------------------------
interface MapComponentProps {} 

const MapComponent: React.FC<MapComponentProps> = () => {
    const [markers, setMarkers] = useState<LatLngTuple[]>([]);
    const initialPosition: LatLngTuple = [45.257, 19.842];
    const initialZoom: number = 13;

    if (markers.length > 0) {
        markers.forEach((pos, index) => {
            console.log(`Marker ${index + 1}: Lat: ${pos[0].toFixed(3)}, Lng: ${pos[1].toFixed(3)}`);
        });
    }

    return (
        <MapContainer 
            center={initialPosition} 
            zoom={initialZoom} 
            scrollWheelZoom={true} 
            style={{ height: '60vh', width: '100%', margin: '10px auto' }} 
        >
            {/* Pass setMarkers down to the SearchControlComponent */}
            <SearchControlComponent setMarkers={setMarkers} /> 
            
            <TileLayer
                attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            />
            
            <ClickHandler setMarkers={setMarkers} />
            
            {/* Markers list now includes both click and search markers */}
            {markers.map((position, index) => (
                <Marker key={index} position={position}>
                    <Popup>
                        Marker **{index + 1}** <br /> 
                        Lat: {position[0].toFixed(3)}, Lng: {position[1].toFixed(3)}
                    </Popup>
                </Marker>
            ))}
            
        </MapContainer>
    );
}

export default MapComponent;