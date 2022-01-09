import { FunctionalComponent, h } from "preact";
import { StateUpdater } from "preact/hooks";

import { Opponent } from "./App";

export type NavProps = {
  level: number;
  setLevel: StateUpdater<number>;
  opponent: Opponent;
  setOpponent: StateUpdater<Opponent>;
};

const Nav: FunctionalComponent<NavProps> = (props) => {
  return (
    <nav className="h-12 shadow-md flex justify-center items-center bg-gray-50">
      <div className="flex space-x-5 items-center">
        <label for="level">Level: </label>
        <input
          id="level"
          type="number"
          max={10}
          min={0}
          value={props.level}
          onChange={(e) => {
            /* @ts-ignore */
            props.setLevel(e.target?.value);
          }}
        />
        <button
          className="h-10 bg-blue-500 rounded flex items-center w-36 justify-center px-5 text-white font-semibold"
          id="modstander"
          onClick={() =>
            props.setOpponent((prev) =>
              prev === Opponent.SinglePlayer
                ? Opponent.MultiPlayer
                : Opponent.SinglePlayer
            )
          }
        >
          {props.opponent === Opponent.MultiPlayer ? "Multiplayer" : "AI"}
        </button>
      </div>
    </nav>
  );
};

export default Nav;
