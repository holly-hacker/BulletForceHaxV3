'use strict';

const path = require('path');

const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const PATHS = {
    src: path.resolve(__dirname, 'src'),
    build: path.resolve(__dirname, 'build'),
};

// used in the module rules and in the stats exlude list
const IMAGE_TYPES = /\.(png|jpe?g|gif|svg)$/i;

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
            // Check for images imported in .js files and
            {
                test: IMAGE_TYPES,
                use: [
                    {
                        loader: 'file-loader',
                        options: {
                            outputPath: 'images',
                            name: '[name].[ext]',
                        },
                    },
                ],
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
    ],
};

module.exports = config;
