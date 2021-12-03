import {h} from "../snowpack/pkg/preact.js";
const Nav = () => {
  return /* @__PURE__ */ h("nav", {
    className: "h-12 shadow-md flex justify-center items-center"
  }, /* @__PURE__ */ h("p", {
    className: "text-2xl"
  }, " Omega Chess"));
};
export default Nav;
