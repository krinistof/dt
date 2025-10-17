import path from "node:path";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
	return {
		resolve: {
			alias: {
				"@": path.resolve(__dirname, "."),
			},
		},
	};
});
