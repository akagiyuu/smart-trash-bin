import {
    ChartConfig,
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from '@/components/ui/chart';
import { Line, LineChart, XAxis, YAxis } from 'recharts';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

const formatDate = (date: Date) => date.toTimeString().slice(0, 8);

const formatTooltipDate = (date: Date) => {
    return date.toLocaleString('en-US', {
        month: 'short',
        day: 'numeric',
        year: 'numeric',
        hour: 'numeric',
        minute: 'numeric',
    });
};

export type Data = {
    time: Date;
    trash_level: number;
};

type Props = {
    data: Data[];
};

export const TrashLevelGraph = ({ data }: Props) => {
    const chartConfig = {
        time: {
            label: 'Trash Level',
        },
    } satisfies ChartConfig;

    return (
        <Card className="w-full">
            <CardHeader>
                <CardTitle className="text-2xl font-bold">
                    Trash Level Over Time
                </CardTitle>
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig} className="h-[400px]">
                    <LineChart data={data}>
                        <XAxis
                            dataKey="time"
                            tickFormatter={formatDate}
                            axisLine={false}
                            tickLine={false}
                        />
                        <YAxis axisLine={false} tickLine={false} />
                        <ChartTooltip
                            content={({ active, payload }) => {
                                if (active && payload && payload.length) {
                                    const data = payload[0].payload as Data;
                                    return (
                                        <ChartTooltipContent>
                                            <div className="text-sm font-medium text-muted-foreground">
                                                {formatTooltipDate(data.time)}
                                            </div>
                                            <div className="text-sm">
                                                <span className="font-medium text-[var(--color-trashLevel)]">
                                                    Trash Level:{' '}
                                                </span>
                                                {data.trash_level}%
                                            </div>
                                        </ChartTooltipContent>
                                    );
                                }
                                return null;
                            }}
                        />
                        <Line
                            type="monotone"
                            dataKey="trash_level"
                            strokeWidth={2}
                            dot={false}
                            activeDot={{ r: 8 }}
                        />
                    </LineChart>
                </ChartContainer>
            </CardContent>
        </Card>
    );
};
