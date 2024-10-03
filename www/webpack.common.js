const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, ".")
    })
  ],
  experiments: {
    asyncWebAssembly: true
  },
  module: {
    rules: [
      // https://getbootstrap.com/docs/4.0/getting-started/webpack/#importing-compiled-css
      { test: /\.css$/, use: [{ loader: 'style-loader' }, { loader: 'css-loader' }] },
      { test: /\.(ttf|eot|svg|otf|woff|woff2)(\?v=[0-9]\.[0-9]\.[0-9])?$/i, loader: 'file-loader' }
    ]
  }
}
