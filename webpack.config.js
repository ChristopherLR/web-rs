const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = {
  entry: "./ui/index.ts",
  devtool: "inline-source-map",
  devServer: { contentBase: "./dist" },
  mode: "development",
  plugins: [
    new HtmlWebpackPlugin({ title: "Omni" }),
    new CleanWebpackPlugin({ cleanStaleWebpackAssests: false }),
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
  optimization: {
    splitChunks: {
      chunks: "all",
    },
  },
  output: {
    filename: "[name].js",
    sourceMapFilename: "[file].map",
    path: path.resolve(__dirname, "dist"),
    publicPath: "/",
  },
};
