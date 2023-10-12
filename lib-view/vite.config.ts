import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			"/api": {
				target: "https://127.0.0.1:8000",
				secure: false
			}
		}
	},
	css: {
		preprocessorOptions: {
			scss: {
				additionalData: '@use "src/bootstrap/variables.scss" as *;'
			}
		}
	}
});
