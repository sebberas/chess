import { h, Fragment, FunctionalComponent, ComponentType, JSX } from "preact";
import { useState, useEffect, StateUpdater } from "preact/hooks";
import { Component as PawnLightIcon } from "./icons/pawn_light.svg";
import { Component as PawnDarkIcon } from "./icons/pawn_dark.svg";
import {
  Color,
  default_board,
  Piece as PieceType,
  Pos,
  valid_moves,
} from "../engine/pkg/crab_engine";

type Piece = {
  color: Color;
  type: PieceType;
  icon: ComponentType<JSX.SVGAttributes<SVGElement>>;
};

type BoardItemProps = {
  color: "white" | "black";
  piece: Piece | null;
  x: number;
  y: number;
  onClick: StateUpdater<[number, number] | null>;
};

const BoardItem: FunctionalComponent<BoardItemProps> = ({
  color,
  piece,
  x,
  y,
  onClick,
}) => {
  const className = `flex items-center justify-center ${
    color == "black" ? "bg-[#F0D9B5]" : "bg-[#B58863]"
  }`;

  return (
    <div
      className={className}
      onClick={() => {
        if (piece != null) onClick([x, y]);
      }}
    >
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

  useEffect(() => {
    console.log(clickedItem);

    if (clickedItem !== null) {
      let piece = pieces[clickedItem[0]][clickedItem[1]] as Piece;
      let pos = new Pos();
      pos.x = clickedItem[0];
      pos.y = clickedItem[1];

      console.log(valid_moves(piece.type, pos, piece.color));
    }
  }, [clickedItem]);

  return (
    <div className="mt-6 w-full flex justify-center">
      <div className="grid grid-cols-[repeat(8,1fr)] grid-rows-[repeat(8,1fr)] h-[80vh] w-[80vh]">
        {pieces.map((row, x) => {
          return row.map((piece, y) => (
            <BoardItem
              color={(x + y) % 2 === 0 ? "black" : "white"}
              piece={piece}
              x={x}
              y={y}
              onClick={setClickedItem}
            />
          ));
        })}
      </div>
    </div>
  );
};

export default Board;
