// typescript acts a bit buggy here
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

'use strict';

import path from 'path';

import CopyWebpackPlugin from 'copy-webpack-plugin';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';

/** @type {{src: string, build: string}} */
const PATHS = {
	src: path.resolve(import.meta.dirname, 'src'),
	build: path.resolve(import.meta.dirname, 'build'),
};

// Merge webpack configuration files
/** @type {(env, argv: { [key:string]:string }) => import('webpack').Configuration} */
const config = (_env, argv) => ({
	entry: {
		sidepanel: PATHS.src + '/sidepanel/index.tsx',
		contentScripts_gameFrame_documentStart: PATHS.src + '/contentScripts/gameFrameDocumentStart.ts',
		webAccessibleResources_gameFrameRuntime: PATHS.src + '/webAccessibleResources/gameFrameRuntime/index.ts',
		background: PATHS.src + '/background.ts',
		devtools_page: PATHS.src + '/devtools/page.ts',
		devtools_panel: PATHS.src + '/devtools/panel.tsx',
	},
	devtool: argv.mode === 'development' ? 'inline-source-map' : false,
	output: {
		// the build folder to output bundles and assets in.
		path: PATHS.build,
		// the filename template for entry chunks
		filename: '[name].js',
	},
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				use: 'ts-loader',
				exclude: /node_modules/,
			},
		],
	},
	resolve: {
		extensions: ['.tsx', '.ts', '.js'],
	},
	plugins: [
		// Copy static assets from `public` folder to `build` folder
		new CopyWebpackPlugin({
			patterns: [
				{
					from: '**/*',
					context: 'public',
				},
			],
		}),

		// Build the rust project
		new WasmPackPlugin({
			crateDirectory: path.resolve(import.meta.dirname, '../bfh3-browser-implant'),
			extraArgs: '--target no-modules --no-pack',
			outDir: '../bfh3-extension/bfh3-browser-implant'
		}),

		// Copy the built wasm binaries so they can be used by the browser extension.
		new CopyWebpackPlugin({
			patterns: [
				{
					from: '*.{js,wasm}',
					to: 'wasm',
					context: 'bfh3-browser-implant',
				},
			],
		}),
	],
	experiments: {
		asyncWebAssembly: true,
	}
});

export default config;
