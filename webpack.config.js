const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("copy-webpack-plugin");

module.exports = {
  entry: "./ui/index.ts",
  devtool: "inline-source-map",
  devServer: { contentBase: "./dist" },
  mode: "development",
  plugins: [
    new CleanWebpackPlugin({ cleanStaleWebpackAssests: false }),
    new HtmlWebpackPlugin({ title: "development" }),
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
};
