import { Summary } from '@/components/summary';
import { Data, TrashLevelGraph } from '@/components/trash_level_graph';
import { Button } from '@/components/ui/button';
import { useToast } from '@/hooks/use-toast';
import { useQuery } from '@tanstack/react-query';
import { ArrowLeft } from 'lucide-react';
import { useEffect, useState } from 'react';
import { Link, useParams } from 'react-router';
import useWebSocket from 'react-use-websocket';
import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from '@tauri-apps/plugin-notification';

type Status = {
    time: Date;
    is_open: boolean;
    trash_level: number;
};

const notification = async (status: Status) => {
    let permissionGranted = await isPermissionGranted();

    // If not we need to request it
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }

    // Once permission has been granted we can send the notification
    if (permissionGranted) {
        sendNotification({
            title: 'Trash bin is full',
            body: `Current trash level: ${status.trash_level}`,
        });
    }
};

export const Monitor = () => {
    const { toast } = useToast();
    const params = useParams();
    const device_id = params.device_id!;

    const { data: name, error } = useQuery<string>({
        queryKey: ['name', device_id],
        queryFn: async () => {
            const response = await fetch(
                `${import.meta.env.VITE_API_URL}/device/${device_id}/name`,
            );

            return await response.text();
        },
    });

    const [is_open, set_is_open] = useState(false);
    const [trash_level, set_trash_level] = useState(0);
    const [trash_level_history, set_trash_level_history] = useState<Data[]>([]);

    const { lastJsonMessage: status } = useWebSocket<Status>(
        `${import.meta.env.VITE_WS_URL}/device/${device_id}/data`,
    );

    useEffect(() => {
        if (status === null) {
            return;
        }
        status.time = new Date(status.time);

        if (status.trash_level >= import.meta.env.VITE_TRASH_LEVEL_THRESHOLD) {
            notification(status);
        }

        set_is_open(status.is_open);
        set_trash_level(status.trash_level);
        set_trash_level_history((prev) => prev.concat(status));
    }, [status]);

    if (error) {
        toast({
            title: 'Failed to get device name',
            description: error.toString(),
            variant: 'destructive',
        });
        return;
    }

    if (name === undefined) return;

    return (
        <div className="w-full max-w-4xl mx-auto space-y-6 p-6">
            <Link to="/">
                <Button variant="outline" className="flex items-center gap-2">
                    <ArrowLeft className="h-4 w-4" />
                    Back to Main Page
                </Button>
            </Link>
            <Summary
                device_id={device_id}
                is_open={is_open}
                trash_level={trash_level}
                name={name}
            />
            <TrashLevelGraph data={trash_level_history} />
        </div>
    );
};
