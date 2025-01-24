import { useState } from 'react';
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from '@/components/ui/command';
import { cn } from '@/lib/utils';

interface Data {
    id: string;
    name: string;
}

type Props<T extends Data> = {
    data: T[];
    className: string;
    onSelect: (data: T) => void;
};

export const SearchBar = <T extends Data>({
    data,
    className,
    onSelect,
}: Props<T>) => {
    const [isFocused, setIsFocused] = useState(false);
    const [query, setQuery] = useState('');

    return (
        <Command
            className={cn(
                'w-full h-fit max-w-md px-4 rounded-lg border shadow-md',
                className,
            )}
        >
            <CommandInput
                placeholder="Search devices"
                value={query}
                onInput={(e) => setQuery(e.target.value)}
                onFocus={() => setIsFocused(true)}
                onBlur={() => setTimeout(() => setIsFocused(false), 200)} // Hide options on blur with delay
            />
            <CommandList className={isFocused ? '' : 'hidden'}>
                <CommandEmpty>No results found.</CommandEmpty>
                <CommandGroup>
                    {data.map((data) => (
                        <CommandItem
                            key={data.id}
                            value={data.name}
                            onSelect={(currentValue) => {
                                setQuery(
                                    currentValue === query ? '' : currentValue,
                                );
                                setIsFocused(false);
                                onSelect(data);
                            }}
                        >
                            {data.name}
                        </CommandItem>
                    ))}
                </CommandGroup>
            </CommandList>
        </Command>
    );
};
