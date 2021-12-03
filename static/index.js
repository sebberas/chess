import {render, h} from "../_snowpack/pkg/preact.js";
import App from "./App.js";
import "../_snowpack/pkg/preact/debug.js";
const root = document.getElementById("app");
if (root !== null) {
  render(/* @__PURE__ */ h(App, null), root);
}
