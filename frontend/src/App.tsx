import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Monitor } from './pages/monitor';
import { Device } from './pages/device';
import { BrowserRouter, Route } from 'react-router';
import { Routes } from 'react-router';
import { Toaster } from '@/components/ui/toaster';

const queryClient = new QueryClient();

function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <BrowserRouter>
                <Routes>
                    <Route path="/">
                        <Route index element={<Device />} />
                        <Route path=":device_id" element={<Monitor />} />
                    </Route>
                </Routes>
            </BrowserRouter>
            <Toaster />
        </QueryClientProvider>
    );
}

export default App;
