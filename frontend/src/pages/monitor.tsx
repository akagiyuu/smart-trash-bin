import { Summary } from '@/components/summary';
import { Data, TrashLevelGraph } from '@/components/trash_level_graph';
import { useToast } from '@/hooks/use-toast';
import { useQuery } from '@tanstack/react-query';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router';
import useWebSocket from 'react-use-websocket';

type Status = {
    time: Date;
    is_open: boolean;
    trash_level: number;
};

const build_socket_url = (device_id: string) =>
    `ws://${import.meta.env.VITE_BACKEND_URL}/data/${device_id}`;

const build_get_name_url = (device_id: string) =>
    `http://${import.meta.env.VITE_BACKEND_URL}/name/${device_id}`;

export const Monitor = () => {
    const { toast } = useToast();
    const params = useParams();
    const device_id = params.device_id!;

    const { data: name, error } = useQuery<string>({
        queryKey: ['device_name', device_id],
        queryFn: async () => {
            const response = await fetch(build_get_name_url(device_id));

            return await response.text();
        },
    });

    const [is_open, set_is_open] = useState(false);
    const [trash_level, set_trash_level] = useState(0);
    const [trash_level_history, set_trash_level_history] = useState<Data[]>([]);

    const { lastJsonMessage: status } = useWebSocket<Status>(
        build_socket_url(device_id),
    );

    useEffect(() => {
        if (status === null) {
            return;
        }
        status.time = new Date(status.time);

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

    if(name === undefined) return;

    return (
        <div className="w-full max-w-4xl mx-auto space-y-6 p-6">
            <h1 className="text-3xl font-bold text-center mb-8">
                Trash Bin Monitoring System
            </h1>
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
