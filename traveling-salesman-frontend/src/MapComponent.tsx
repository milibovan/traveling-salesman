import React from 'react';
import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet';
import 'leaflet/dist/leaflet.css';
import L from 'leaflet';

delete (L.Icon.Default.prototype as any)._getIconUrl;

L.Icon.Default.mergeOptions({
    iconRetinaUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon-2x.png',
    iconUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-icon.png',
    shadowUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-shadow.png',
});

interface MapComponentProps {} 

const MapComponent: React.FC<MapComponentProps> = () => {
    // Defines a type for the coordinates (Latitude and Longitude)
    const initialPosition: L.LatLngTuple = [45.257, 19.842]; // Example: Coordinates for Novi Sad
    const initialZoom: number = 13;

    return (
        <MapContainer 
            center={initialPosition} 
            zoom={initialZoom} 
            scrollWheelZoom={true} 
            // Use standard React styling syntax
            style={{ height: '60vh', width: '100%', margin: '10 auto' }} 
        >
            {/* Map Tiles (OpenStreetMap) */}
            <TileLayer
                attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            />
            
            <Marker position={initialPosition}>
                <Popup>
                    {"Starting City (Novi Sad)"}
                </Popup>
            </Marker>
            
        </MapContainer>
    );
}

export default MapComponent;