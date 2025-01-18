import { Button } from '@/components/ui/button';
import {
    Card,
    CardFooter,
    CardDescription,
    CardContent,
    CardHeader,
    CardTitle,
} from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

export const Device = () => {
    const { isPending, error, data } = useQuery<string[]>({
        queryKey: ['device'],
        queryFn: async () => {
            const response = await fetch(
                `http://${import.meta.env.VITE_BACKEND_URL}/device`,
            );

            return await response.json();
        },
    });
    const [selected_device, set_selected_device] = useState<string | null>(
        null,
    );

    if (isPending) return 'Loading...';

    if (error) return 'An error has occurred: ' + error.message;

    if (data === undefined) return <div></div>;

    return (
        <div className="flex justify-center items-center h-screen">
            <Card className="w-full max-w-sm">
                <CardHeader>
                    <CardTitle className="text-2xl font-bold text-center">
                        Trash Bin Monitoring
                    </CardTitle>
                    <CardDescription className="text-center">
                        Monitor a chosen trash bin
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div className="flex items-center space-x-4 mb-6">
                        <Select
                            value={selected_device || ''}
                            onValueChange={(device) =>
                                set_selected_device(device)
                            }
                        >
                            <SelectTrigger className="w-full">
                                <SelectValue placeholder="Select a device" />
                            </SelectTrigger>
                            <SelectContent>
                                {data.map((item) => (
                                    <SelectItem key={item} value={item}>
                                        {item}
                                    </SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>
                    <Button
                        className="w-full"
                        disabled={!selected_device}
                        onClick={() =>
                            window.location.replace(`/${selected_device}`)
                        }
                    >
                        Connect Phone
                    </Button>
                </CardContent>
            </Card>
        </div>
    );
};
