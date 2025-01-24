import { useToast } from '@/hooks/use-toast';
import { useQuery } from '@tanstack/react-query';
import { MapContainer, Marker, Popup, TileLayer, useMap } from 'react-leaflet';
import { useEffect, useState } from 'react';
import { SearchBar } from '@/components/search_bar';
import { latLng, LatLngExpression } from 'leaflet';
import { Button } from '@/components/ui/button';
import { useNavigate } from 'react-router';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

type Device = {
    id: string;
    name: string;
    latitude: number;
    longitude: number;
};

const intoLatLng = (data: { latitude: number; longitude: number }) =>
    latLng({
        lat: data.latitude,
        lng: data.longitude,
    });

// Default to Ho Chi Minh City
const DEFAULT_LOCATION = latLng({
    lat: 10.747538648019852,
    lng: 106.6762389246497,
});

function MapController({ location }: { location: LatLngExpression }) {
    const map = useMap();

    useEffect(() => {
        map.setView(location);
    }, [location, map]);

    return null;
}

export const Device = () => {
    const [location, setLocation] =
        useState<LatLngExpression>(DEFAULT_LOCATION);
    const [error, setError] = useState<GeolocationPositionError | null>(null);
    const navigate = useNavigate();

    const { toast } = useToast();

    const { error: fetchError, data } = useQuery<Device[]>({
        queryKey: ['device'],
        queryFn: async () => {
            const response = await fetch(
                `http://${import.meta.env.VITE_BACKEND_URL}/device`,
            );

            return await response.json();
        },
    });

    useEffect(() => {
        navigator.geolocation.getCurrentPosition(
            (position) => intoLatLng(position.coords),
            (error) => setError(error),
        );
    }, []);

    useEffect(() => {
        if (error) {
            toast({
                title: 'Fallback to default location',
                description: error.toString(),
                variant: 'destructive',
            });
        }
    }, [error, toast]);

    useEffect(() => {
        if (fetchError) {
            toast({
                title: 'Failed to get device list',
                description: fetchError.toString(),
                variant: 'destructive',
            });
        }
    }, [fetchError, toast]);

    if (data === undefined) return;

    return (
        <div>
            <SearchBar
                className="z-[999] fixed top-5 left-1/2 transform -translate-x-1/2"
                data={data}
                onSelect={(device) => {
                    setLocation(intoLatLng(device));
                }}
            />
            <MapContainer
                center={location}
                zoom={13}
                style={{ height: '100vh' }}
            >
                <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />

                {data.map((item) => {
                    return (
                        <Marker
                            key={item.id}
                            position={intoLatLng(item)}
                            title={item.name}
                        >
                            <Popup>
                                <Card className="w-full max-w-sm border-none">
                                    <CardHeader>
                                        <CardTitle className="text-center text-lg font-bold">
                                            {item.name}
                                        </CardTitle>
                                    </CardHeader>
                                    <CardContent className="space-y-2">
                                        <p className="text-sm">
                                            Latitude: {item.latitude}
                                        </p>
                                        <p className="text-sm">
                                            Longitude: {item.longitude}
                                        </p>
                                        <Button
                                            variant="default"
                                            className="w-full"
                                            onClick={() =>
                                                navigate(`/${item.id}`)
                                            }
                                        >
                                            Monitor
                                        </Button>
                                    </CardContent>
                                </Card>
                            </Popup>
                        </Marker>
                    );
                })}
                <MapController location={location} />
            </MapContainer>
        </div>
    );
};
