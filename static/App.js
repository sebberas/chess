import {h} from "../snowpack/pkg/preact.js";
import Nav from "./Nav.js";
import Board from "./Board.js";
import init from "./engine/crab_engine.js";
import {useState} from "../snowpack/pkg/preact/hooks.js";
await init();
export var Opponent;
(function(Opponent2) {
  Opponent2[Opponent2["SinglePlayer"] = 0] = "SinglePlayer";
  Opponent2[Opponent2["MultiPlayer"] = 1] = "MultiPlayer";
})(Opponent || (Opponent = {}));
const App = () => {
  const [opponent, setOpponent] = useState(1);
  const [level, setLevel] = useState(1);
  return /* @__PURE__ */ h("div", {
    className: "min-h-screen bg-[#F4F4F4]"
  }, /* @__PURE__ */ h(Nav, {
    opponent,
    setOpponent,
    level,
    setLevel
  }), /* @__PURE__ */ h(Board, {
    level,
    opponent
  }));
};
export default App;
