import { render, h } from "preact";
import App from "./App";
import "preact/debug";

const root = document.getElementById("app");

if (root !== null) {
  render(<App />, root);
}
