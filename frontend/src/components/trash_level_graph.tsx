import {
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from '@/components/ui/chart';
import { Line, LineChart, XAxis, YAxis } from 'recharts';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

const formatDate = (date: Date) => date.toTimeString().slice(0, 8);

export type Data = {
    time: Date;
    trash_level: number;
};

type Props = {
    data: Data[];
};

export const TrashLevelGraph = ({ data }: Props) => {
    return (
        <Card className="w-full">
            <CardHeader>
                <CardTitle className="text-2xl font-bold">
                    Trash Level Over Time
                </CardTitle>
            </CardHeader>
            <CardContent>
                <ChartContainer
                    config={{
                        trash_level: {
                            label: 'Trash Level',
                            color: 'hsl(var(--chart-1))',
                        },
                    }}
                    className="h-[400px]"
                >
                    <LineChart data={data}>
                        <XAxis
                            dataKey="time"
                            tickFormatter={formatDate}
                            axisLine={false}
                            tickLine={false}
                            additive="sum"
                        />
                        <YAxis axisLine={false} tickLine={false} />
                        <ChartTooltip
                            content={
                                <ChartTooltipContent
                                    labelFormatter={(_, payload) => {
                                        return new Date(payload[0].payload.time)
                                            .toString()
                                            .slice(0, 24);
                                    }}
                                />
                            }
                            cursor={false}
                            defaultIndex={1}
                        />
                        <Line
                            type="monotone"
                            dataKey="trash_level"
                            strokeWidth={2}
                            dot={false}
                            isAnimationActive={false}
                            additive="sum"
                        />
                    </LineChart>
                </ChartContainer>
            </CardContent>
        </Card>
    );
};
