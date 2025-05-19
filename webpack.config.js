// webpack.config.js
const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin   = require("@wasm-tool/wasm-pack-plugin");

const distPath = path.resolve(__dirname, "dist");
const pkgPath  = path.resolve(__dirname, "pkg");

module.exports = {
    mode: process.env.NODE_ENV || "development",
    entry: "./bootstrap.js",

    output: {
        path: distPath,
        filename: "yewchat.js",
        webassemblyModuleFilename: "yewchat_bg.wasm",
    },

    resolve: {
        extensions: [".js", ".wasm"],
    },

    module: {
        parser: {
            javascript: {
                // Keep import.meta.url calls intact
                importMeta: false,
            }
        },
        rules: [
            // Emit .wasm as a static asset, not parse it
            {
                test: /\.wasm$/,
                type: "asset/resource",
                generator: {
                    filename: "[name][ext]"
                }
            },
            // JS/CSS/image loaders
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"]
            },
            {
                test: /\.(png|jpg|gif|svg)$/,
                type: "asset/resource"
            }
        ],
    },

    experiments: {
        // Disable webpack’s own Wasm parser
        asyncWebAssembly: false,
    },

    plugins: [
        // Static files
        new CopyWebpackPlugin({
            patterns: [{ from: "static", to: distPath }],
        }),

        // Build Rust → Wasm with web-targeted pkg
        new WasmPackPlugin({
            crateDirectory: __dirname,
            outDir:        pkgPath,
            outName:       "yewchat",
            // ensure wasm-pack uses the `web` target for a default init export
            extraArgs:     "--target web -- --features wee_alloc",
            watchDirectories: [path.resolve(__dirname, "src")],
        }),
    ],

    devServer: {
        static: distPath,
        port:   8000,
        hot:    true,
    },
};
