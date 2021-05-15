// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    ts: "/src",
    static: "/",
    scss: "/src",
    images: "/src",
  },
  plugins: [
    "@snowpack/plugin-typescript",
    "@snowpack/plugin-react-refresh",
    "@snowpack/plugin-sass",
    "@snowpack/plugin-dotenv"
  ],
  packageOptions: {
    /* ... */
  },
  devOptions: {
    /* ... */
  },
  buildOptions: {
    /* ... */
  },
  optimize: {
    bundle: true,
    minify: true
  },
  routes: [
    {match: "routes", src: ".*", dest: "/index.html"}
  ]
};
