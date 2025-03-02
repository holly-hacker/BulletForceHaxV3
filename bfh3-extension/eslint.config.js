import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginReact from "eslint-plugin-react";
import pluginReactHooks from 'eslint-plugin-react-hooks';


/** @type {import('eslint').Linter.Config[]} */
export default [
	{ files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"] },
	{ ignores: [ "build/*", "bfh3-browser-implant/*", ] },
	{ languageOptions: { globals: globals.browser } },
	pluginJs.configs.recommended,
	...tseslint.configs.recommended,
	pluginReact.configs.flat.recommended,
	pluginReactHooks.configs["recommended-latest"],
	{
		linterOptions: {
			reportUnusedInlineConfigs: 'error',
		},
		settings: {
			react: {
				version: "detect",
			}
		},
		rules: {
			"no-unused-vars": "off",
			"@typescript-eslint/no-unused-vars": ["error", {
				// ignore anything starting with `_`
				"argsIgnorePattern": "^_",
				"varsIgnorePattern": "^_",
			}],
		},
	}
];
