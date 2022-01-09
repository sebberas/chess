import {h} from "../snowpack/pkg/preact.js";
import {Opponent} from "./App.js";
const Nav = (props) => {
  return /* @__PURE__ */ h("nav", {
    className: "h-12 shadow-md flex justify-center items-center bg-gray-50"
  }, /* @__PURE__ */ h("div", {
    className: "flex space-x-5 items-center"
  }, /* @__PURE__ */ h("label", {
    for: "level"
  }, "Level: "), /* @__PURE__ */ h("input", {
    id: "level",
    type: "number",
    max: 10,
    min: 0,
    value: props.level,
    onChange: (e) => {
      props.setLevel(e.target?.value);
    }
  }), /* @__PURE__ */ h("button", {
    className: "h-10 bg-blue-500 rounded flex items-center w-36 justify-center px-5 text-white font-semibold",
    id: "modstander",
    onClick: () => props.setOpponent((prev) => prev === Opponent.SinglePlayer ? Opponent.MultiPlayer : Opponent.SinglePlayer)
  }, props.opponent === Opponent.MultiPlayer ? "Multiplayer" : "AI")));
};
export default Nav;
