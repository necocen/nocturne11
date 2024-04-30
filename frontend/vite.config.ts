import path from "node:path";
import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    root: path.resolve(__dirname, "src"),
    base: "./",
    publicDir: path.resolve(__dirname, "public"),
    resolve: {
        alias: {
            images: "src/images",
        },
    },
    envDir: path.resolve(__dirname),
    build: {
        assetsInlineLimit: 0,
        outDir: path.resolve(__dirname, "dist"),
        emptyOutDir: true,
        minify: "esbuild",
        sourcemap: true,
        rollupOptions: {
            input: [
                path.resolve(__dirname, "src/index.html"),
                path.resolve(__dirname, "src/scss/main.scss"),
                path.resolve(__dirname, "src/scss/sp.scss"),
            ],
            output: {
                assetFileNames: "assets/[name][extname]",
                chunkFileNames: "assets/[name].js",
                entryFileNames: "assets/[name].js",
            },
        },
    },
});
