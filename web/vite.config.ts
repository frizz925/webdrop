import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		proxy: {
			'/ws': {
				target: 'ws://localhost:8000',
				ws: true,
				rewriteWsOrigin: true
			},
			'/api': 'http://localhost:8000',
			'/objects': 'http://localhost:8000'
		}
	}
});
