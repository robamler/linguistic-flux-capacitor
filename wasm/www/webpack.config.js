const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
var path = require('path');


module.exports = {
  mode: 'production',
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.html$/i,
        use: [{
          loader: 'html-loader',
          options: {
            minimize: true
          }
        }],
      },
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\/assets\//,
        use: 'file-loader'
      },
      {
        test: /\.(png|svg|jpg|gif|woff2)$/,
        loader: 'file-loader',
        options: {
          esModule: false,
        },
      },
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, 'src', 'index.html'),
    }),
    new CopyPlugin({
      patterns: [
        { from: 'src/favicon.png', to: 'favicon.png' },
        { from: 'src/favicon-256.png', to: 'favicon-256.png' },
        { from: 'src/browsers/firefox.png', to: 'browsers/firefox.png' },
        { from: 'src/browsers/chrome.png', to: 'browsers/chrome.png' },
        { from: 'src/browsers/safari.png', to: 'browsers/safari.png' },
        { from: 'src/browsers/edge.png', to: 'browsers/edge.png' },
        { from: 'src/browsers/opera.png', to: 'browsers/opera.png' },
      ],
    }),
  ],
};
