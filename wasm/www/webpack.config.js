const HtmlWebpackPlugin = require('html-webpack-plugin');
var path = require('path');

module.exports = {
  mode: 'development', // TODO: Eventually change to 'production'.
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
         test: /\.(png|svg|jpg|gif)$/,
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
    })
  ],
};
