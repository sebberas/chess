// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    src: "/static",
    public: "/",
    "lib/pkg": "/static/engine",
    /* ... */
  },
  plugins: [
    "inline-svg",
    "@snowpack/plugin-typescript",
    "@snowpack/plugin-postcss",
  ],
  packageOptions: {
    /* ... */
  },
  devOptions: {
    tailwindConfig: "./tailwind.config.js",
  },
  buildOptions: {
    baseUrl: "chess",
    metaUrlPath: "snowpack",
    jsxFactory: "h",
    jsxFragment: "Fragment",
  },
};
