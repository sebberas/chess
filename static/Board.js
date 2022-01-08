import {h} from "../snowpack/pkg/preact.js";
import {useState, useEffect} from "../snowpack/pkg/preact/hooks.js";
import {Component as PawnDarkIcon} from "./icons/pawn_dark.js";
import {Component as PawnLightIcon} from "./icons/pawn_light.js";
import {Component as KnightDarkIcon} from "./icons/knight_dark.js";
import {Component as KnightLightIcon} from "./icons/knight_light.js";
import {Component as BishopDarkIcon} from "./icons/bishop_dark.js";
import {Component as BishopLightIcon} from "./icons/bishop_light.js";
import {Component as RookDarkIcon} from "./icons/rook_dark.js";
import {Component as RookLightIcon} from "./icons/rook_light.js";
import {Component as QueenDarkIcon} from "./icons/queen_dark.js";
import {Component as QueenLightIcon} from "./icons/queen_light.js";
import {Component as KingDarkIcon} from "./icons/king_dark.js";
import {Component as KingLightIcon} from "./icons/king_light.js";
import {
  Color,
  default_board,
  Piece as PieceType,
  new_pos,
  board_move,
  board_valid_moves,
  get_best_move
} from "./engine/crab_engine.js";
const extractMoves = (arr) => {
  if (arr.length === 0) {
    return [];
  }
  let temp = [];
  let view = new DataView(arr.buffer);
  for (let i = 0; i < arr.length; i += 2) {
    temp.push([view.getInt8(i), view.getInt8(i + 1)]);
  }
  return temp;
};
const BoardItem = ({
  color,
  piece,
  pos,
  onClick
}) => {
  const className = `flex items-center justify-center ${color == "black" ? "bg-[#F0D9B5]" : "bg-[#B58863]"}`;
  return /* @__PURE__ */ h("div", {
    className,
    onClick: () => onClick(pos)
  }, piece !== null ? /* @__PURE__ */ h(piece.icon, {
    style: {transform: "scale(1.25)"}
  }) : null);
};
const initPieces = () => {
  const pa = (color) => {
    return {
      color,
      type: PieceType.Pawn,
      icon: color === Color.Black ? PawnDarkIcon : PawnLightIcon
    };
  };
  const kn = (color) => {
    return {
      color,
      type: PieceType.Knight,
      icon: color == Color.Black ? KnightDarkIcon : KnightLightIcon
    };
  };
  const bi = (color) => {
    return {
      color,
      type: PieceType.Bishop,
      icon: color == Color.Black ? BishopDarkIcon : BishopLightIcon
    };
  };
  const ro = (color) => {
    return {
      color,
      type: PieceType.Rook,
      icon: color == Color.Black ? RookDarkIcon : RookLightIcon
    };
  };
  const qu = (color) => {
    return {
      color,
      type: PieceType.Queen,
      icon: color == Color.Black ? QueenDarkIcon : QueenLightIcon
    };
  };
  const ki = (color) => {
    return {
      color,
      type: PieceType.King,
      icon: color == Color.Black ? KingDarkIcon : KingLightIcon
    };
  };
  const w = Color.White;
  const b = Color.Black;
  let pieces = [
    [ro(w), kn(w), bi(w), ki(w), qu(w), bi(w), kn(w), ro(w)],
    new Array(8).fill(pa(w)),
    new Array(8).fill(null),
    new Array(8).fill(null),
    new Array(8).fill(null),
    new Array(8).fill(null),
    new Array(8).fill(pa(b)),
    [ro(b), kn(b), bi(b), ki(b), qu(b), bi(b), kn(b), ro(b)]
  ];
  return pieces;
};
const Board = () => {
  const [turn, SetTurn] = useState(Color.White);
  const [board, setBoard] = useState(default_board());
  const [pieces, setPieces] = useState(initPieces());
  const [clickedItem, setClickedItem] = useState(null);
  const [possibleMoves, setPossibleMoves] = useState([]);
  const [ai, SetAi] = useState(true);
  const changeTurn = () => {
    SetTurn((prev) => {
      if (prev === Color.White) {
        return Color.Black;
      }
      return Color.White;
    });
  };
  useEffect(() => {
    if (clickedItem !== null) {
      let piece = pieces[clickedItem[1]][clickedItem[0]];
      if (piece?.color === turn) {
        let pos = new_pos(clickedItem[0], clickedItem[1]);
        let moves = extractMoves(board_valid_moves(board, piece.type, pos, piece.color));
        setPossibleMoves(moves);
      }
    }
  }, [clickedItem]);
  useEffect(() => {
    console.log(turn);
    if (turn == Color.Black) {
      let bestMove = get_best_move(board.board, Color.Black, 3);
      console.log(bestMove);
      let from = new_pos(bestMove[0], bestMove[1]);
      let to = new_pos(bestMove[2], bestMove[3]);
      board_move(board, from, to);
      setPieces((pieces2) => {
        let piece = pieces2[bestMove[1]][bestMove[0]];
        console.log(piece);
        pieces2[bestMove[1]][bestMove[0]] = null;
        pieces2[bestMove[3]][bestMove[2]] = piece;
        console.log(pieces2);
        return pieces2;
      });
      changeTurn();
    }
  }, [turn]);
  return /* @__PURE__ */ h("div", {
    className: "mt-6 w-full flex justify-center relative"
  }, /* @__PURE__ */ h("div", {
    className: "grid grid-cols-[repeat(8,1fr)] grid-rows-[repeat(8,1fr)] h-[80vh] w-[80vh]"
  }, pieces.map((row, y) => {
    return row.map((piece, x) => /* @__PURE__ */ h(BoardItem, {
      color: (x + y) % 2 === 0 ? "black" : "white",
      piece,
      pos: [x, y],
      onClick: setClickedItem
    }));
  })), possibleMoves.length > 0 && clickedItem ? /* @__PURE__ */ h(Overlay, {
    changeTurn,
    board,
    setBoard,
    clickedItem,
    setClickedItem,
    possibleMoves,
    setPossibleMoves,
    pieces,
    setPieces
  }) : null);
};
const Overlay = (props) => {
  let {
    changeTurn,
    board,
    setBoard,
    clickedItem,
    setClickedItem,
    possibleMoves,
    setPossibleMoves,
    setPieces
  } = props;
  const handleMove = (move) => {
    setPieces((pieces) => {
      const from = new_pos(clickedItem[0], clickedItem[1]);
      const to = new_pos(move[0], move[1]);
      if (board_move(board, from, to)) {
        setBoard(board);
      }
      let piece = pieces[clickedItem[1]][clickedItem[0]];
      pieces[clickedItem[1]][clickedItem[0]] = null;
      pieces[move[1]][move[0]] = piece;
      return pieces;
    });
    setClickedItem(null);
    setPossibleMoves([]);
    changeTurn();
  };
  return /* @__PURE__ */ h("div", {
    className: "absolute grid grid-cols-[repeat(8,1fr)] grid-rows-[repeat(8,1fr)] w-[80vh] h-[80vh]"
  }, /* @__PURE__ */ h(OverlayItem, {
    pos: clickedItem,
    onClick: () => {
      setClickedItem(null);
      setPossibleMoves([]);
    }
  }), possibleMoves.map((move) => /* @__PURE__ */ h(OverlayItem, {
    pos: move,
    active: true,
    onClick: () => handleMove(move)
  })));
};
const OverlayItem = (props) => {
  let {active, onClick, pos} = props;
  return /* @__PURE__ */ h("div", {
    onClick: () => onClick !== void 0 ? onClick() : {},
    className: `${rowStart[pos[1]]} ${colStart[pos[0]]} ${active ? "bg-black opacity-30" : ""}`
  });
};
const rowStart = {
  0: "row-start-1",
  1: "row-start-2",
  2: "row-start-3",
  3: "row-start-4",
  4: "row-start-5",
  5: "row-start-6",
  6: "row-start-7",
  7: "row-start-[8]"
};
const colStart = {
  0: "col-start-1",
  1: "col-start-2",
  2: "col-start-3",
  3: "col-start-4",
  4: "col-start-5",
  5: "col-start-6",
  6: "col-start-7",
  7: "col-start-[8]"
};
export default Board;
