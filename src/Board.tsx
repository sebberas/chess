import { h, Fragment, FunctionalComponent, ComponentType, JSX } from "preact";
import { useState, useEffect, StateUpdater } from "preact/hooks";

// Nedenstående kode importerer alle ikonerne til de forskellige brikker.
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

// Rust funktioner er skrevet med snake_case, mens Javascript funktioner er camelCase.
// Typer er altid i PascalCase.
import {
  Color,
  default_board,
  // Vi omdøber Piece til PieceType, for at undgå name-clashing.
  Piece as PieceType,
  new_pos,
  board_move,
  board_valid_moves,
  GameState,
  get_best_move,
  board_winner,
} from "../engine/pkg/crab_engine";
import { Opponent } from "./App";

// Denne funktion tager en buffer og konverterer den om til en liste af x,y koordinator.
const extractMoves = (arr: Int8Array): [number, number][] => {
  // Tidligt return, hvis listen er tom.
  if (arr.length === 0) {
    return [];
  }

  let temp: [number, number][] = [];
  // Vi konstruerer et DataView objekt, der konverterer rust-tal om til Javacsript kompatible tal.
  let view = new DataView(arr.buffer);
  // Vi ved at moves altid kommer i par af 2
  for (let i = 0; i < arr.length; i += 2) {
    // Hvis man kigger i motor koden, kan det ses at arrayet indeholder i8, så det er den datatype vi skal konvertere.
    temp.push([view.getInt8(i), view.getInt8(i + 1)]);
  }

  return temp;
};

// Vi definerer en Piece type, vi senere kan bruge.
type Piece = {
  color: Color;
  type: PieceType;
  icon: ComponentType<JSX.SVGAttributes<SVGElement>>;
};

// Vi definere hvilke props, BoardItem tager.
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
  // Hvis farven er sort, skal den vise en sort baggrund. Ellers skal den vise en hvid.
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

// Vi initialiserer skakbrættet, så brikkerne står korrekt.
const initPieces = () => {
  // Retunerer en sort eller hvid bonde.
  const pa = (color: Color) => {
    return {
      color: color,
      type: PieceType.Pawn,
      icon: color === Color.Black ? PawnDarkIcon : PawnLightIcon,
    };
  };

  // Retunerer en sort eller hvid springer.
  const kn = (color: Color) => {
    return {
      color: color,
      type: PieceType.Knight,
      icon: color == Color.Black ? KnightDarkIcon : KnightLightIcon,
    };
  };

  // Retunerer en sort eller hvid løber.
  const bi = (color: Color) => {
    return {
      color: color,
      type: PieceType.Bishop,
      icon: color == Color.Black ? BishopDarkIcon : BishopLightIcon,
    };
  };

  // Retunerer et sort eller hvidt tårn.
  const ro = (color: Color) => {
    return {
      color: color,
      type: PieceType.Rook,
      icon: color == Color.Black ? RookDarkIcon : RookLightIcon,
    };
  };

  // Retunerer en sort eller hvid dronning.
  const qu = (color: Color) => {
    return {
      color: color,
      type: PieceType.Queen,
      icon: color == Color.Black ? QueenDarkIcon : QueenLightIcon,
    };
  };

  // Retunerer en sort eller hvid konge.
  const ki = (color: Color) => {
    return {
      color: color,
      type: PieceType.King,
      icon: color == Color.Black ? KingDarkIcon : KingLightIcon,
    };
  };

  /* Ovenstående funktioner er kun lavet, for at gøre koden mere letlæselig, når vi initialiserer brættet. */
  const w = Color.White;
  const b = Color.Black;

  // Brættet er en 2-dimensionel liste.
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

type BoardProps = {
  level: number;
  opponent: Opponent;
};

const Board: FunctionalComponent<BoardProps> = ({ level, opponent }) => {
  const [turn, SetTurn] = useState(Color.White);
  const [board, setBoard] = useState(default_board());
  const [pieces, setPieces] = useState<(Piece | null)[][]>(initPieces());
  const [clickedItem, setClickedItem] = useState<[number, number] | null>(null);
  const [possibleMoves, setPossibleMoves] = useState<[number, number][]>([]);

  // Ændrer turen til den næste spiller.
  const changeTurn = () => {
    // Vi tjekker om nogen har vundet. Hvis nogen har det, viser vi en boks.
    let winner = board_winner(board);
    if (winner !== "none") {
      setTimeout(() => {
        switch (winner) {
          case "black":
            window.alert("Sort har vundet!");
            break;
          case "white":
            window.alert("Hvid har vundet!");
            break;
        }
      }, 100);
    }

    SetTurn((prev) => {
      if (prev === Color.White) {
        return Color.Black;
      }

      return Color.White;
    });
  };

  // Håndterer når en brik bliver trykket på.
  useEffect(() => {
    if (clickedItem !== null) {
      let piece = pieces[clickedItem[1]][clickedItem[0]];

      if (piece?.color === turn) {
        let pos = new_pos(clickedItem[0], clickedItem[1]);

        // let moves = valid_moves(piece.type, pos, piece.color);
        let moves = extractMoves(
          board_valid_moves(board, piece.type, pos, piece.color)
        );

        setPossibleMoves(moves);
      }
    }
  }, [clickedItem]);

  // Denne funktion kaldes hver gang turen skifter.
  useEffect(() => {
    // Vi tjekker om den nuværende tur er sort, fordi den kunstige
    // intelligens altid spiller sort og om den kunstige intelligens er aktiveret.
    if (turn === Color.Black && opponent === Opponent.SinglePlayer) {
      // Vi bruger en rust funktion til at finde de bedste træk.
      let bestMove = get_best_move(board.board, Color.Black, level);

      // Vi konstruerer 2 positioner.
      let from = new_pos(bestMove[0], bestMove[1]);
      let to = new_pos(bestMove[2], bestMove[3]);

      // Vi rykker på brikken, så den kunstige intelligens, ved den er rykket.
      board_move(board, from, to);

      // Vi opdaterer ui'ens repræsentation af brikkerne.
      setPieces((pieces) => {
        let piece = pieces[bestMove[1]][bestMove[0]];
        pieces[bestMove[1]][bestMove[0]] = null;
        pieces[bestMove[3]][bestMove[2]] = piece;
        return pieces;
      });

      // Vi skifter tur.
      changeTurn();
    }
  }, [turn]);

  return (
    <div className="mt-6 w-full flex justify-center relative">
      {/* Board */}
      <div className="grid grid-cols-[repeat(8,1fr)] grid-rows-[repeat(8,1fr)] h-[80vh] w-[80vh]">
        {/* Vi itererer over alle felterne på brættet. */}
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
      {/* Vi tegner overlay, hvis der er nogen mulige moves og der er trykket på en brik. */}
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

// Props tilhørende OverlayProps.
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
  // Vi udtager de forskellige variabler fra props.
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

  // Håndterer når en brik bliver rykket.
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

    // Vi opdaterer Boards state.
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
      {/* Vi itererer over alle possibleMoves og tegner en mørk firkant de steder. */}
      {possibleMoves.map((move) => (
        <OverlayItem pos={move} active onClick={() => handleMove(move)} />
      ))}
    </div>
  );
};

// Props tilhørende OverlayItem.
type OverlayItemProps = {
  active?: boolean;
  onClick?: () => unknown;
  pos: [number, number];
};

// OverlayItem tegner en mørk firkant.
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
