import { FunctionalComponent, h, Fragment } from "preact";
import Nav from "./Nav";
import Board from "./Board";

import init from "../engine/pkg/crab_engine";
import { useState } from "preact/hooks";

// Vi initaliserer WebAseembly koden genenm init funktionen.
await init();

export enum Opponent {
  SinglePlayer,
  MultiPlayer,
}

// Vi definerer App komponentet som et funktionelt komponent. App tegner <Nav /> og <Board /> komponenterne.
const App: FunctionalComponent = () => {
  const [opponent, setOpponent] = useState(Opponent.MultiPlayer);
  const [level, setLevel] = useState(1);

  return (
    <div className="min-h-screen bg-[#F4F4F4]">
      <Nav
        opponent={opponent}
        setOpponent={setOpponent}
        level={level}
        setLevel={setLevel}
      />
      <Board level={level} opponent={opponent} />
    </div>
  );
};

export default App;
