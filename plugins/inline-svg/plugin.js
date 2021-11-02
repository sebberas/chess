const { readFileSync } = require("fs");
const esbuild = require("esbuild");
const path = require("path");
const _ = require("lodash");

/** @type {import("snowpack").SnowpackPluginFactory } */
const plugin = (snowpackConfig, options) => {
  return {
    name: "inline-svg",
    resolve: {
      input: [".svg"],
      output: [".js"],
    },
    async load({ filePath }) {
      const fileContents = readFileSync(filePath).toString();
      const svgElementWithProps = fileContents.replace(">", " {...props}>");

      const fileName = path.basename(filePath, "svg");
      const iconName =
        fileName[0].toUpperCase() + _.camelCase(fileName.substring(1));

      const componentCode = `import {Fragment, h} from "preact";
      export const Component = ({...props}) => {return (${svgElementWithProps})};
      Component.displayName = "${iconName}Icon"; `;

      const compiledCode = esbuild.transformSync(componentCode, {
        loader: "tsx",
        jsxFactory: snowpackConfig.buildOptions.jsxFactory,
        jsxFragment: snowpackConfig.buildOptions.jsxFragment,
      });

      return { ".js": compiledCode };
    },
  };
};

module.exports = plugin;
