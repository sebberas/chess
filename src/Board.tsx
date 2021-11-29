import { h, Fragment, FunctionalComponent, JSX } from "preact";
import { Component as PawnLightIcon } from "./icons/pawn_light.svg";
import { Component as PawnDarkIcon } from "./icons/pawn_dark.svg";

enum PieceColor {
  White,
  Black,
}

interface Piece {
  color: PieceColor;
  icon: FunctionalComponent;

  moves(x: number, y: number): [number, number][];
}

class Pawn implements Piece {
  constructor(color: PieceColor) {
    this.color = color;
    this.icon = color == PieceColor.White ? PawnLightIcon : PawnDarkIcon;
  }

  color: PieceColor;
  icon: FunctionalComponent;

  moves(x: number, y: number): [number, number][] {
    let moves: [number, number][] = [];

    if (!this.moved) {
      moves.push([x, y + 2]);
    }

    moves.push([x, y + 1]);
    return moves;
  }

  private moved: boolean = false;
}

const BoardItem: FunctionalComponent<{
  color: "black" | "white";
  piece?: Piece;
}> = ({ color, piece }) => {
  const classNames = `relative ${
    color == "black" ? "bg-[#F0D9B5]" : "bg-[#B58863]"
  }`;

  return (
    <div className={classNames}>
      {piece !== undefined ? (
        <piece.icon className="absolute inset-l-1/2 transform inset-t-1/2" />
      ) : null}
    </div>
  );
};

function renderBoardItems(pieces: (undefined | Piece)[][]) {
  console.log(pieces);
  let items: JSX.Element[] = [];

  for (let y = 0; y < 8; y++) {
    for (let x = 0; x < 8; x++) {
      items.push(
        <BoardItem
          piece={pieces[x][y]}
          color={(x + y) % 2 === 0 ? "black" : "white"}
        />
      );
    }
  }

  return items;
}

const Board: FunctionalComponent = () => {
  let pieces: (undefined | Piece)[][] = Array(8)
    .fill(undefined)
    .map(() => Array(8).fill(undefined));

  pieces[0][0] = new Pawn(PieceColor.Black);
  pieces[0][7] = new Pawn(PieceColor.White);
  pieces[1][7] = new Pawn(PieceColor.White);
  pieces[2][7] = new Pawn(PieceColor.White);
  pieces[3][7] = new Pawn(PieceColor.White);
  pieces[4][7] = new Pawn(PieceColor.White);
  pieces[5][7] = new Pawn(PieceColor.Black);

  return (
    <div className="mt-6 w-full flex justify-center">
      <div className="h-[80vh] w-[3vh] bg-[#8C715B]"></div>

      <div className="grid grid-cols-8 grid-rows-8 h-[80vh] w-[80vh]">
        {renderBoardItems(pieces)}
      </div>

      <div className="h-[80vh] w-[3vh] bg-[#8C715B]"></div>
    </div>
  );
};

export default Board;
