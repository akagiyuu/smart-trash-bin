import { Button } from '@/components/ui/button';
import {
    Card,
    CardDescription,
    CardContent,
    CardHeader,
    CardTitle,
} from '@/components/ui/card';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select';
import { useToast } from '@/hooks/use-toast';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

type Device = {
    id: string,
    name: string
}

export const Device = () => {
    const { toast } = useToast();

    const { error, data } = useQuery<Device[]>({
        queryKey: ['list'],
        queryFn: async () => {
            const response = await fetch(
                `http://${import.meta.env.VITE_BACKEND_URL}/device/list`,
            );

            return await response.json();
        },
    });
    const [selected_device, set_selected_device] = useState<string | null>(
        null,
    );

    if (error) {
        toast({
            title: 'Failed to get device list',
            description: error.toString(),
            variant: 'destructive',
        });
        return;
    }

    if (data === undefined) return;

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
                                    <SelectItem key={item.id} value={item.id}>
                                        {item.name}
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
                        Monitor
                    </Button>
                </CardContent>
            </Card>
        </div>
    );
};
