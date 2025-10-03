export type LatLngTuple = L.LatLngTuple;

export type MarkerData = {
  coordinates: LatLngTuple;
  label: string;
};
export type RouteResponse = {
  cities: string[];
  total_distance: number;
};