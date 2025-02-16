import { Button } from '@/components/ui/button';
import {
    Card,
    CardDescription,
    CardContent,
    CardHeader,
    CardTitle,
} from '@/components/ui/card';
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from '@/components/ui/command';
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from '@/components/ui/popover';
import { useToast } from '@/hooks/use-toast';
import { cn } from '@/lib/utils';
import { useQuery } from '@tanstack/react-query';
import { Check, ChevronsUpDown } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router';

type Device = {
    id: string;
    name: string;
};

export const Device = () => {
    const { toast } = useToast();
    const navigate = useNavigate();

    const { error, data } = useQuery<Device[]>({
        queryKey: ['list'],
        queryFn: async () => {
            const response = await fetch(
                `${__API_URL__}/device/list`,
            );

            return await response.json();
        },
    });
    const [selected_device, set_selected_device] = useState<Device | null>(
        null,
    );
    const [open, setOpen] = useState(false);

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
                        <Popover open={open} onOpenChange={setOpen}>
                            <PopoverTrigger asChild>
                                <Button
                                    variant="outline"
                                    role="combobox"
                                    aria-expanded={open}
                                    className="w-full justify-between"
                                >
                                    {selected_device
                                        ? selected_device.name
                                        : 'Select phone...'}
                                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                                </Button>
                            </PopoverTrigger>
                            <PopoverContent className="w-full p-0">
                                <Command>
                                    <CommandInput placeholder="Search phone..." />
                                    <CommandList>
                                        <CommandEmpty>
                                            No phone found.
                                        </CommandEmpty>
                                        <CommandGroup>
                                            {data.map((device) => (
                                                <CommandItem
                                                    key={device.id}
                                                    onSelect={() => {
                                                        set_selected_device(
                                                            device,
                                                        );
                                                        setOpen(false);
                                                    }}
                                                >
                                                    <Check
                                                        className={cn(
                                                            'mr-2 h-4 w-4',
                                                            selected_device?.id ===
                                                                device.id
                                                                ? 'opacity-100'
                                                                : 'opacity-0',
                                                        )}
                                                    />
                                                    {device.name}
                                                </CommandItem>
                                            ))}
                                        </CommandGroup>
                                    </CommandList>
                                </Command>
                            </PopoverContent>
                        </Popover>
                    </div>
                    <Button
                        className="w-full"
                        disabled={!selected_device}
                        onClick={() => navigate(`/${selected_device?.id}`)}
                    >
                        Monitor
                    </Button>
                </CardContent>
            </Card>
        </div>
    );
};
