import globals from "globals";
import js from "@eslint/js";
import tseslint from "typescript-eslint";
import compat from "eslint-plugin-compat";

export default tseslint.config(
	js.configs.recommended,
	...tseslint.configs.recommended,
	{
		...compat.configs["flat/recommended"],
		settings: {
			polyfills: ["Promise"],
		},
	},
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.es2021,
			},
		},
	},
	{
		ignores: ["dist", ".vite", "src/gen"],
	},
);
