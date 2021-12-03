import {h} from "../_snowpack/pkg/preact.js";
import {Component as PawnLightIcon} from "./icons/pawn_light.js";
import {Component as PawnDarkIcon} from "./icons/pawn_dark.js";
var PieceColor;
(function(PieceColor2) {
  PieceColor2[PieceColor2["White"] = 0] = "White";
  PieceColor2[PieceColor2["Black"] = 1] = "Black";
})(PieceColor || (PieceColor = {}));
class Pawn {
  constructor(color) {
    this.moved = false;
    this.color = color;
    this.icon = color == 0 ? PawnLightIcon : PawnDarkIcon;
  }
  moves(x, y) {
    let moves = [];
    if (!this.moved) {
      moves.push([x, y + 2]);
    }
    moves.push([x, y + 1]);
    return moves;
  }
}
const BoardItem = ({color, piece}) => {
  const classNames = `relative ${color == "black" ? "bg-[#F0D9B5]" : "bg-[#B58863]"}`;
  return /* @__PURE__ */ h("div", {
    className: classNames
  }, piece !== void 0 ? /* @__PURE__ */ h(piece.icon, {
    className: "absolute inset-l-1/2 transform inset-t-1/2"
  }) : null);
};
function renderBoardItems(pieces) {
  console.log(pieces);
  let items = [];
  for (let y = 0; y < 8; y++) {
    for (let x = 0; x < 8; x++) {
      items.push(/* @__PURE__ */ h(BoardItem, {
        piece: pieces[x][y],
        color: (x + y) % 2 === 0 ? "black" : "white"
      }));
    }
  }
  return items;
}
const Board = () => {
  let pieces = Array(8).fill(void 0).map(() => Array(8).fill(void 0));
  pieces[0][0] = new Pawn(1);
  pieces[0][7] = new Pawn(0);
  pieces[1][7] = new Pawn(0);
  pieces[2][7] = new Pawn(0);
  pieces[3][7] = new Pawn(0);
  pieces[4][7] = new Pawn(0);
  pieces[5][7] = new Pawn(1);
  return /* @__PURE__ */ h("div", {
    className: "mt-6 w-full flex justify-center"
  }, /* @__PURE__ */ h("div", {
    className: "h-[80vh] w-[3vh] bg-[#8C715B]"
  }), /* @__PURE__ */ h("div", {
    className: "grid grid-cols-8 grid-rows-8 h-[80vh] w-[80vh]"
  }, renderBoardItems(pieces)), /* @__PURE__ */ h("div", {
    className: "h-[80vh] w-[3vh] bg-[#8C715B]"
  }));
};
export default Board;
