import { h, Fragment, FunctionalComponent, JSX } from "preact";

enum PieceColor {
  White = "bg-[#B58863]",
  Black = "bg-[#F0D9B5]",
}

class Piece {
  constructor(x: number, y: number, type: string) {
    this.x = x;
    this.y = y;
    this.type = type;
  }
  x: number;
  y: number;
  type: string;
}

const BoardItem: FunctionalComponent<{ color: PieceColor }> = ({ color }) => (
  <div className={`${color}`}></div>
)

function renderBoardItems() {
  let items = [];

  for (let y = 0; y < 8; y++) {
    for (let x = 0; x < 8; x++) {
      items.push(<BoardItem color={((x + y) % 2 == 0) ? PieceColor.Black : PieceColor.White} />)
    }
  }
  return items;
}

function renderPieces() {
  let pieces: Piece[] = [];
  let postition:
    pieces = pieces.map(() => new Piece(2, 5, "Knight"));

  pieces.forEach(element => {
    console.log(element);
  });
}

const Board: FunctionalComponent = () => {
  { renderPieces() }

  return (


    <div className="mt-6 w-full flex justify-center">
      <div className="h-[80vh] w-[3vh] bg-[#8C715B]"></div>


      <div draggable className="relative grid grid-cols-8 grid-rows-8 h-[80vh] w-[80vh]">
        {renderBoardItems()}
      </div>

      <div className="h-[80vh] w-[3vh] bg-[#8C715B]"></div>
    </div>);
}

export default Board;
