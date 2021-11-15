import { FunctionalComponent, h, Fragment } from "preact";
import Nav from "./Nav";
import Board from "./Board";

const App: FunctionalComponent = () => (
  <div className="min-h-screen bg-[#F4F4F4]">
    <Nav />
    <Board />
  </div>
);

export default App;
