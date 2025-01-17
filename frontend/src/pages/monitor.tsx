import { Summary } from '@/components/summary';
import { Data, TrashLevelGraph } from '@/components/trash_level_graph';
import { useEffect, useState } from 'react';
import useWebSocket from 'react-use-websocket';

type Status = {
    time: Date;
    is_open: boolean;
    trash_level: number;
};

type Props = {
    device_name: string;
};

const build_socket_url = (device_name: string) =>
    `ws://${import.meta.env.VITE_BACKEND_URL}/data/${device_name}`;

export const Monitor = ({ device_name }: Props) => {
    const [is_open, set_is_open] = useState(false);
    const [trash_level, set_trash_level] = useState(0);
    const [trash_level_history, set_trash_level_history] = useState<Data[]>([]);

    const { lastJsonMessage: status } = useWebSocket<Status>(
        build_socket_url(device_name),
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

    return (
        <div className="w-full max-w-4xl mx-auto space-y-6 p-6">
            <h1 className="text-3xl font-bold text-center mb-8">
                Trash Bin Monitoring System
            </h1>
            <Summary is_open={is_open} trash_level={trash_level} />
            <TrashLevelGraph data={trash_level_history} />
        </div>
    );
};
