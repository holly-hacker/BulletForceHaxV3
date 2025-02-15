'use strict';

const path = require('path');

const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const PATHS = {
	src: path.resolve(__dirname, 'src'),
	build: path.resolve(__dirname, 'build'),
};

// Merge webpack configuration files
/** @type {import('webpack').Configuration} */
const config = {
	entry: {
		popup: PATHS.src + '/popup.ts',
		contentScripts_gameFrame_documentStart: PATHS.src + '/contentScripts/gameFrameDocumentStart.ts',
		webAccessibleResources_gameFrame_loadGameHook: PATHS.src + '/webAccessibleResources/gameFrameLoadGameHook.ts',
		background: PATHS.src + '/background.ts',
	},
	devtool: 'source-map',
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
			// Help webpack in understanding CSS files imported in .js files
			{
				test: /\.css$/,
				use: [MiniCssExtractPlugin.loader, 'css-loader'],
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

		// Extract CSS into separate files
		new MiniCssExtractPlugin({
			filename: '[name].css',
		}),

		// Build the rust project
		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, '../bfh3-browser-implant'),
			extraArgs: '--target no-modules --no-pack', // --no-typescript is omitted because it is useful for development
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
};

module.exports = config;
