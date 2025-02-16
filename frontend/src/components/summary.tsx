import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Trash2, Edit2 } from 'lucide-react';

type Props = {
    device_id: string;
    name: string;
    is_open: boolean;
    trash_level: number;
};

export const Summary = ({
    device_id,
    name: initial_name,
    is_open,
    trash_level,
}: Props) => {
    const [name, setName] = useState(initial_name);
    const [isEditing, setIsEditing] = useState(false);

    const handleNameSubmit = async () => {
        setIsEditing(false);

        await fetch(
            `${__API_URL__}/device/${device_id}/name`,
            {
                method: 'POST',
                body: name,
            },
        );
    };

    return (
        <Card className="w-full">
            <CardHeader>
                <CardTitle className="text-2xl font-bold flex items-center justify-between">
                    <div className="flex items-center gap-2">
                        <Trash2 className="h-6 w-6 text-primary" />
                        {isEditing ? (
                            <Input
                                value={name}
                                onChange={(e) => setName(e.target.value)}
                                onBlur={handleNameSubmit}
                                onKeyDownCapture={(e) => {
                                    if (e.key == 'Enter') handleNameSubmit();
                                }}
                                className="max-w-[200px] text-lg font-bold bg-transparent border-b border-primary focus:ring-0 focus:border-primary-foreground px-0"
                                autoFocus
                            />
                        ) : (
                            <span
                                className="text-primary cursor-pointer"
                                onDoubleClick={() => setIsEditing(true)}
                                title="Double-click to edit"
                            >
                                {name}
                            </span>
                        )}
                    </div>
                    <button
                        onClick={() => setIsEditing(!isEditing)}
                        className="text-muted-foreground hover:text-primary transition-colors"
                        aria-label={isEditing ? 'Save name' : 'Edit name'}
                    >
                        <Edit2 className="h-5 w-5" />
                    </button>
                </CardTitle>
            </CardHeader>
            <CardContent className="grid gap-4">
                <div className="flex justify-between items-center">
                    <div className="space-y-1">
                        <p className="text-sm font-medium leading-none">
                            Status
                        </p>
                        <p className="text-sm text-muted-foreground">
                            {is_open ? 'Open and ready for disposal' : 'Closed'}
                        </p>
                    </div>
                    <Badge
                        variant={is_open ? 'default' : 'secondary'}
                        className="text-sm py-1 px-2"
                    >
                        {is_open ? 'Open' : 'Closed'}
                    </Badge>
                </div>
                <div className="flex justify-between items-center">
                    <div className="space-y-1">
                        <p className="text-sm font-medium leading-none">
                            Current Fill Level
                        </p>
                        <p className="text-sm text-muted-foreground">
                            {trash_level < 80 ? 'Normal' : 'Needs attention'}
                        </p>
                    </div>
                    <div className="text-right">
                        <span className="text-2xl font-bold">
                            {trash_level}%
                        </span>
                    </div>
                </div>
                <div className="w-full bg-secondary rounded-full h-2.5 dark:bg-secondary">
                    <div
                        className="bg-primary h-2.5 rounded-full transition-all duration-500 ease-in-out"
                        style={{ width: `${trash_level}%` }}
                        role="progressbar"
                        aria-valuenow={trash_level}
                        aria-valuemin={0}
                        aria-valuemax={100}
                    ></div>
                </div>
            </CardContent>
        </Card>
    );
};
