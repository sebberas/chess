import { render, h } from "preact";
import App from "./App";

const root = document.getElementById("app");

if (root !== null) {
  render(<App />, root);
}
