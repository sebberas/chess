import { h, Fragment, FunctionalComponent, ComponentType, JSX } from "preact";
import { useState, useEffect, StateUpdater } from "preact/hooks";
import { Component as PawnDarkIcon } from "./icons/pawn_dark.svg";
import { Component as PawnLightIcon } from "./icons/pawn_light.svg";
import { Component as KnightDarkIcon } from "./icons/knight_dark.svg";
import { Component as KnightLightIcon } from "./icons/knight_light.svg";
import { Component as BishopDarkIcon } from "./icons/bishop_dark.svg";
import { Component as BishopLightIcon } from "./icons/bishop_light.svg";
import { Component as RookDarkIcon } from "./icons/rook_dark.svg";
import { Component as RookLightIcon } from "./icons/rook_light.svg";
import { Component as QueenDarkIcon } from "./icons/queen_dark.svg";
import { Component as QueenLightIcon } from "./icons/queen_light.svg";
import { Component as KingDarkIcon } from "./icons/king_dark.svg";
import { Component as KingLightIcon } from "./icons/king_light.svg";

import {
  Color,
  default_board,
  Piece as PieceType,
  new_pos,
  valid_moves,
  board_is_valid_move,
  board_move,
  board_valid_moves,
  GameState,
} from "../engine/pkg/crab_engine";

type Piece = {
  color: Color;
  type: PieceType;
  icon: ComponentType<JSX.SVGAttributes<SVGElement>>;
};

type BoardItemProps = {
  color: "white" | "black";
  piece: Piece | null;
  pos: [number, number];
  onClick: StateUpdater<[number, number] | null>;
};

const BoardItem: FunctionalComponent<BoardItemProps> = ({
  color,
  piece,
  pos,
  onClick,
}) => {
  const className = `flex items-center justify-center ${
    color == "black" ? "bg-[#F0D9B5]" : "bg-[#B58863]"
  }`;

  return (
    <div className={className} onClick={() => onClick(pos)}>
      {piece !== null ? (
        <piece.icon style={{ transform: "scale(1.25)" }} />
      ) : null}
    </div>
  );
};

const initPieces = () => {
  const pa = (color: Color) => {
    return {
      color: color,
      type: PieceType.Pawn,
      icon: color === Color.Black ? PawnDarkIcon : PawnLightIcon,
    };
  };

  const kn = (color: Color) => {
    return {
      color: color,
      type: PieceType.Knight,
      icon: color == Color.Black ? KnightDarkIcon : KnightLightIcon,
    };
  };

  const bi = (color: Color) => {
    return {
      color: color,
      type: PieceType.Bishop,
      icon: color == Color.Black ? BishopDarkIcon : BishopLightIcon,
    };
  };

  const ro = (color: Color) => {
    return {
      color: color,
      type: PieceType.Rook,
      icon: color == Color.Black ? RookDarkIcon : RookLightIcon,
    };
  };

  const qu = (color: Color) => {
    return {
      color: color,
      type: PieceType.Queen,
      icon: color == Color.Black ? QueenDarkIcon : QueenLightIcon,
    };
  };

  const ki = (color: Color) => {
    return {
      color: color,
      type: PieceType.King,
      icon: color == Color.Black ? KingDarkIcon : KingLightIcon,
    };
  };

  const w = Color.White;
  const b = Color.Black;
  let pieces: (Piece | null)[][] = [
    [ro(w), kn(w), bi(w), ki(w), qu(w), bi(w), kn(w), ro(w)],
    new Array(8).fill(pa(w)),
    new Array(8).fill(null),
    new Array(8).fill(null),
    new Array(8).fill(null),
    new Array(8).fill(null),
    new Array(8).fill(pa(b)),
    [ro(b), kn(b), bi(b), ki(b), qu(b), bi(b), kn(b), ro(b)],
  ];

  return pieces;
};

const Board: FunctionalComponent = () => {
  const [turn, SetTurn] = useState(Color.White);
  const [board, setBoard] = useState(default_board());
  const [pieces, setPieces] = useState<(Piece | null)[][]>(initPieces());
  const [clickedItem, setClickedItem] = useState<[number, number] | null>(null);
  const [possibleMoves, setPossibleMoves] = useState<[number, number][]>([]);

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

        // let moves = valid_moves(piece.type, pos, piece.color);
        let moves = board_valid_moves(board, piece.type, pos, piece.color);
        if (moves.length > 0) {
          let view = new DataView(moves.buffer);
          let temp: [number, number][] = [];
          for (let i = 0; i < moves.length; i += 2) {
            temp.push([view.getInt8(i), view.getInt8(i + 1)]);
          }

          return setPossibleMoves(temp);
        }

        setPossibleMoves([]);
      }

      if (piece !== null) {
      }
    }
  }, [clickedItem]);

  return (
    <div className="mt-6 w-full flex justify-center relative">
      {/* Board */}
      <div className="grid grid-cols-[repeat(8,1fr)] grid-rows-[repeat(8,1fr)] h-[80vh] w-[80vh]">
        {pieces.map((row, y) => {
          return row.map((piece, x) => (
            <BoardItem
              color={(x + y) % 2 === 0 ? "black" : "white"}
              piece={piece}
              pos={[x, y]}
              onClick={setClickedItem}
            />
          ));
        })}
      </div>
      {/* Overlay */}
      {possibleMoves.length > 0 && clickedItem ? (
        <Overlay
          changeTurn={changeTurn}
          board={board}
          setBoard={setBoard}
          clickedItem={clickedItem as [number, number]}
          setClickedItem={setClickedItem}
          possibleMoves={possibleMoves}
          setPossibleMoves={setPossibleMoves}
          pieces={pieces}
          setPieces={setPieces}
        />
      ) : null}
    </div>
  );
};

type OverlayProps = {
  changeTurn: () => void;
  board: GameState;
  setBoard: StateUpdater<GameState>;
  clickedItem: [number, number];
  setClickedItem: StateUpdater<[number, number] | null>;
  possibleMoves: [number, number][];
  setPossibleMoves: StateUpdater<[number, number][]>;
  pieces: (Piece | null)[][];
  setPieces: StateUpdater<(Piece | null)[][]>;
};

const Overlay: FunctionalComponent<OverlayProps> = (props) => {
  let {
    changeTurn,
    board,
    setBoard,
    clickedItem,
    setClickedItem,
    possibleMoves,
    setPossibleMoves,
    setPieces,
  } = props;

  const handleMove = (move: [number, number]) => {
    setPieces((pieces) => {
      const from = new_pos(clickedItem[0], clickedItem[1]);
      const to = new_pos(move[0], move[1]);

      // console.log(board);
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

  return (
    <div className="absolute grid grid-cols-[repeat(8,1fr)] grid-rows-[repeat(8,1fr)] w-[80vh] h-[80vh]">
      <OverlayItem
        pos={clickedItem}
        onClick={() => {
          setClickedItem(null);
          setPossibleMoves([]);
        }}
      />
      {possibleMoves.map((move) => (
        <OverlayItem pos={move} active onClick={() => handleMove(move)} />
      ))}
    </div>
  );
};

type OverlayItemProps = {
  active?: boolean;
  onClick?: () => unknown;
  pos: [number, number];
};

const OverlayItem: FunctionalComponent<OverlayItemProps> = (props) => {
  let { active, onClick, pos } = props;

  return (
    <div
      onClick={() => (onClick !== undefined ? onClick() : {})}
      className={`${rowStart[pos[1]]} ${colStart[pos[0]]} ${
        active ? "bg-black opacity-30" : ""
      }`}
    ></div>
  );
};

interface RowStart {
  [index: number]: string;
}

const rowStart: RowStart = {
  0: "row-start-1",
  1: "row-start-2",
  2: "row-start-3",
  3: "row-start-4",
  4: "row-start-5",
  5: "row-start-6",
  6: "row-start-7",
  7: "row-start-[8]",
};

interface ColStart {
  [index: number]: string;
}

const colStart: ColStart = {
  0: "col-start-1",
  1: "col-start-2",
  2: "col-start-3",
  3: "col-start-4",
  4: "col-start-5",
  5: "col-start-6",
  6: "col-start-7",
  7: "col-start-[8]",
};

export default Board;
