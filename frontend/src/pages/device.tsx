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

    if (isPending) return 'Loading...';

    if (error) return 'An error has occurred: ' + error.message;

    if (data === undefined) return <div></div>;

    return (
        <Card className="w-[350px]">
            <CardHeader>
                <CardTitle>Trash Bin Monitoring</CardTitle>
                <CardDescription>Monitor a chosen trash bin</CardDescription>
            </CardHeader>
            <CardContent>
                <form
                    onSubmit={(event) => {
                        event.preventDefault();
                        console.log(event.target)
                        // window.location.replace('/');
                    }}
                >
                    <div className="grid w-full items-center gap-4">
                        <div className="flex flex-col space-y-1.5">
                            <Label htmlFor="device">Trash bin</Label>
                            <Select>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select a trash bin" />
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
                    </div>
                </form>
            </CardContent>
            <CardFooter>
                <Button className="m-auto">Monitor</Button>
            </CardFooter>
        </Card>
    );
};
