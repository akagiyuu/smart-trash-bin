import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig(() => {
    const api = process.env.VITE_BACKEND_URL || 'http://localhost:3000';
    const ws = api.replace(/^http/, 'ws'); // Compute at build time

    return {
        plugins: [react()],
        define: {
            __API_URL__: JSON.stringify(api),
            __WS_URL__: JSON.stringify(ws),
        },
        resolve: { alias: { '@': path.resolve(__dirname, './src') } },
    };
});
