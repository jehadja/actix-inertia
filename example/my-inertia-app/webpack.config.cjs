const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { VueLoaderPlugin } = require('vue-loader');

module.exports = {
    mode: 'development',
    entry: './src/main.js',
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist'),
        publicPath: '/',
    },
    devServer: {
        contentBase: path.join(__dirname, 'dist'),
        writeToDisk: true, // Ensures files are written to disk
        compress: true,
        port: 9000,
        proxy: {
            '/': {
                target: 'http://127.0.0.1:8080',
                changeOrigin: true,
                bypass: function (req, res, proxyOptions) {
                    // Do not proxy requests to static files
                    if (req.url.startsWith('/static/') || req.url.startsWith('/bundle.js')) {
                        return req.url;
                    }
                },
            },
        },
        historyApiFallback: true, // Enable single-page app routing fallback
    },
    module: {
        rules: [
            {
                test: /\.vue$/,
                loader: 'vue-loader',
            },
            {
                test: /\.js$/,
                loader: 'babel-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.css$/,
                use: ['vue-style-loader', 'css-loader'],
            },
        ],
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            template: './index.html',
        }),
        new VueLoaderPlugin(),
    ],
    resolve: {
        alias: {
            vue$: 'vue/dist/vue.esm-bundler.js',
            '@': path.resolve(__dirname, 'src'),
        },
        extensions: ['.js', '.vue', '.json'],
    },
};
