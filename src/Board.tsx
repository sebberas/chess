import { h, Fragment, FunctionalComponent, ComponentType, JSX } from "preact";
import { useState, useEffect, StateUpdater } from "preact/hooks";
import { Component as PawnLightIcon } from "./icons/pawn_light.svg";
import { Component as PawnDarkIcon } from "./icons/pawn_dark.svg";
import {
  Color,
  default_board,
  Piece as PieceType,
  new_pos,
  valid_moves,
} from "../engine/pkg/crab_engine";

type Position = {
  x: number;
  y: number;
};

type Piece = {
  color: Color;
  type: PieceType;
  icon: ComponentType<JSX.SVGAttributes<SVGElement>>;
};

type BoardItemProps = {
  color: "white" | "black";
  selected: boolean;
  piece: Piece | null;
  x: number;
  y: number;
  onClick: StateUpdater<[number, number] | null>;
};

const BoardItem: FunctionalComponent<BoardItemProps> = ({
  color,
  selected,
  piece,
  x,
  y,
  onClick,
}) => {
  const className = `flex items-center justify-center ${
    color == "black" ? "bg-[#F0D9B5]" : "bg-[#B58863]"
  } ${selected && "bg-red-400"}`;

  return (
    <div className={className} onClick={() => onClick([x, y])}>
      {piece !== null ? (
        <piece.icon style={{ transform: "scale(1.25)" }} />
      ) : null}
    </div>
  );
};

const initPieces = () => {
  let pieces: (Piece | null)[][] = new Array(8)
    .fill(undefined)
    .map(() => new Array(8).fill(null));

  for (let i = 0; i < 8; i++) {
    pieces[1][i] = {
      color: Color.White,
      type: PieceType.Pawn,
      icon: PawnLightIcon,
    };
  }

  return pieces;
};

const Board: FunctionalComponent = () => {
  const board = default_board();
  const [pieces, setPieces] = useState<(Piece | null)[][]>(initPieces());
  const [clickedItem, setClickedItem] = useState<[number, number] | null>(null);
  const [possibleMoves, setPossibleMoves] = useState<[number, number][]>([]);

  useEffect(() => {
    if (clickedItem !== null) {
      let piece = pieces[clickedItem[1]][clickedItem[0]];

      if (piece !== null) {
        let pos = new_pos(clickedItem[0], clickedItem[1]);
        let moves = valid_moves(PieceType.King, pos, piece.color);

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
              selected={possibleMoves.includes([x, y])}
              x={x}
              y={y}
              onClick={setClickedItem}
            />
          ));
        })}
      </div>
      {/* Overlay */}
      {possibleMoves.length === 0 ? null : (
        <div className="absolute grid grid-cols-[repeat(8,1fr)] grid-rows-[repeat(8,1fr)] w-[80vh] h-[80vh]">
          {possibleMoves.map((move) => {
            console.log(move);
            return (
              <div
                className={`${rowStart[move[1]]} ${
                  colStart[move[0]]
                } bg-black opacity-25`}
              ></div>
            );
          })}
        </div>
      )}
    </div>
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
  7: "row-start-8",
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
  7: "col-start-8",
};

export default Board;
