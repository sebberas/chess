import {h} from "../snowpack/pkg/preact.js";
import Nav from "./Nav.js";
import Board from "./Board.js";
import init, {hello_world} from "./engine/crab_engine.js";
await init();
hello_world();
const App = () => /* @__PURE__ */ h("div", {
  className: "min-h-screen bg-[#F4F4F4]"
}, /* @__PURE__ */ h(Nav, null), /* @__PURE__ */ h(Board, null));
export default App;
