import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Monitor } from './pages/monitor';

const queryClient = new QueryClient();

function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <Monitor device_name="test" />
        </QueryClientProvider>
    );
}

export default App;
