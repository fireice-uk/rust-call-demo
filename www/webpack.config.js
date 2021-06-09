const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const iframe_path = path.resolve(__dirname, "dist", "ifrm")
const root_path = path.resolve(__dirname, "dist")

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: iframe_path,
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      {from : 'sandbox.html', to: iframe_path},
      {from : path.resolve(__dirname, "extension_static"), to: root_path}
    ])
  ],
};
