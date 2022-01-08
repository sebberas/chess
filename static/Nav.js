import {h} from "../snowpack/pkg/preact.js";
const Nav = () => {
  return /* @__PURE__ */ h("nav", {
    className: "h-12 shadow-md flex justify-center items-center bg-gray-50"
  }, /* @__PURE__ */ h("p", {
    className: "text-2xl"
  }, " Omega Chess"));
};
export default Nav;
