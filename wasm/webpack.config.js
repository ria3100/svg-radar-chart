const path = require("path");

module.exports = {
  mode: 'production',
  entry: './main.ts',
  output: {
    libraryTarget: 'commonjs',
    path: path.resolve(__dirname, `./lib/`),
  },
  resolve: {
    extensions: [".ts", ".js", ".wasm"]
  },
  module: {
    rules: [
      {
        test: /\.ts?$/,
        loader: "esbuild-loader",
        options: {
          loader: 'ts',
          target: 'es2015',
          tsconfigRaw: require('./tsconfig.json')
        }
      }
    ]
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  }
};
