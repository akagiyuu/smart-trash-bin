import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Monitor } from './pages/monitor';
import { Device } from './pages/device';

const queryClient = new QueryClient();

function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <Device/>
        </QueryClientProvider>
    );
}

export default App;
