import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Trash2 } from 'lucide-react';

type Props = {
    is_open: boolean;
    trash_level: number;
};

export const Summary = ({ is_open, trash_level }: Props) => {
    return (
        <Card className="w-full">
            <CardHeader>
                <CardTitle className="text-2xl font-bold flex items-center gap-2">
                    <Trash2 className="h-6 w-6" />
                    Trash Can Summary
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
