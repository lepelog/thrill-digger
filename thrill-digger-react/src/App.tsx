import React from 'react';
import './App.css';
import { SolverWrapper } from "./native/build";
import { GridField, HoleContent } from "./GridField";

type CellState = {
  bombPercentage: number,
  rupoorPercentage: number,
  selectedType: HoleContent,
  ranking: number,
};

type AppState = {
  nativeModule: any,
  currentMessage: string,
  solver: SolverWrapper | null,
  cellStates: CellState[],
  boardWidth: number,
  boardHeight: number,
  possibleLoops: boolean[],
  matchingSeedCount: number,
};

class TestComp extends React.Component<{}, AppState> {
  constructor(props: AppState) {
    super(props);

    this.state = {
      nativeModule: null,
      currentMessage: "waiting to initialize...",
      solver: null,
      cellStates: [{bombPercentage: 0, rupoorPercentage: 0, selectedType: HoleContent.Unspecified, ranking: 100}],
      boardHeight: 0,
      boardWidth: 0,
      possibleLoops: [],
      matchingSeedCount: -1,
    };

    this.selectedChanged = this.selectedChanged.bind(this);
  }

  componentDidMount() {
    import("./native/build").then(native => {
      this.setState({
        nativeModule: native,
        currentMessage: "computing values...",
        solver: native.create_solver(2/* expert */),
      });
      setImmediate(() => {
        console.log("setting state");
        // this takes a really long time
        this.state.solver?.cache_boards();
        // this.state.solver?.lock_to_loop(0);
        const boardHeight = this.state.solver!.get_height();
        const boardWidth = this.state.solver!.get_width();
        const newStates = Array(boardHeight * boardWidth).fill(0).map(() => {
          return {
            bombPercentage: 0,
            rupoorPercentage: 0,
            selectedType: HoleContent.Unspecified,
            ranking: 100,
          }
        });
        this.setState({
          currentMessage: "done!",
          boardWidth,
          boardHeight,
          cellStates: newStates,
        });
        this.updateBoardAndRecalculateProbs(newStates);
      });
    });
  }

  componentWillUnmount() {
    this.state.solver?.free();
  }

  getSolverOrError(): SolverWrapper {
    if (this.state.solver === null) {
      throw Error("solver is null!");
    }
    return this.state.solver;
  }

  selectedChanged(index: number, selection: HoleContent) {
    const cellStates = this.state.cellStates;
    cellStates[index].selectedType = selection;
    const solver = this.getSolverOrError();
    solver.set_hole(index, selection);
    this.updateBoardAndRecalculateProbs(cellStates);
  }

  // calculate the new probabilites and sets the cellStates to the state at the end
  updateBoardAndRecalculateProbs(cellStates: CellState[]) {
    const solver = this.getSolverOrError();
    solver.calculate_probabilities_with_pregenerated();
    cellStates.forEach((cellState, index) => {
      cellState.bombPercentage = solver.get_probability(index);
      // cellState.rupoorPercentage = solver.get_rupoor_probability(index);
      cellState.rupoorPercentage = solver.get_rupoor_probability(index);
    });
    // figure out the best places for the ranking, don't include already placed
    const cellStatesWithIndex: [number, CellState][] = cellStates
      .filter(cs => cs.selectedType === HoleContent.Unspecified)
      .map((CellState, index) => [index, CellState]);
    // first sort by bomb probability, then by rupoor probability
    cellStatesWithIndex.sort((a,b) => a[1].bombPercentage - b[1].bombPercentage || a[1].rupoorPercentage - b[1].rupoorPercentage);
    cellStatesWithIndex.forEach(([_, CellState], index) => CellState.ranking = index);
    // make all cells, that are already dug up have no ranking
    cellStates.forEach(cs => {
      if (cs.selectedType !== HoleContent.Unspecified) {
        cs.ranking = 100;
      }
    });
    this.setState({
      cellStates,
      matchingSeedCount: solver.get_possible_rng_values_count(),
      possibleLoops: this.getPossibleLoopArray(solver),
    });
  }

  getPossibleLoopArray(solver: SolverWrapper): boolean[] {
    let loop_array = new Uint8Array(9);
    solver.get_possible_loops(loop_array);
    let our_arr: boolean[] = [];
    loop_array.forEach((val, idx) => {
      our_arr.push(val === 1);
    });
    return our_arr;
  }

  resetPossibleLoopArray() {
    this.getSolverOrError().reset_possible_loops();
    this.updateBoardAndRecalculateProbs(this.state.cellStates);
  }

  setPossibleLoop(idx: number, state: boolean) {
    this.getSolverOrError().set_possible_loop(idx, state);
    this.updateBoardAndRecalculateProbs(this.state.cellStates);
  }

  resetBoard() {
    const boardHeight = this.state.solver?.get_height() || 0;
    const boardWidth = this.state.solver?.get_width() || 0;
    const cellStates = Array(boardHeight * boardWidth).fill(0).map(() => {
      return {
        bombPercentage: 0,
        rupoorPercentage: 0,
        selectedType: HoleContent.Unspecified,
        ranking: 100,
      }
    });
    const solver = this.getSolverOrError();
    for (let i = 0;i < boardHeight * boardWidth;i++) {
      solver.set_hole(i, HoleContent.Unspecified);
    }
    this.updateBoardAndRecalculateProbs(cellStates);
  }

  render() {
    const {boardHeight, boardWidth, cellStates, currentMessage, matchingSeedCount, possibleLoops} = this.state;
    return (
      <div className="App">
      <h1>Thrill Digger Expert solver</h1>
        <div>{currentMessage}</div>
        <div>matching seeds: {matchingSeedCount}</div>
        <div>Possible Loops: {possibleLoops.map((val, idx) => {
          return (<span><input type="checkbox" checked={val} onChange={e => this.setPossibleLoop(idx, e.target.checked)}/>{idx}</span>);
        })}</div>
        <div><button onClick={this.resetPossibleLoopArray.bind(this)}>Reset Loops</button></div>
        <table>
          <tbody>
            {
              Array(boardHeight).fill(0).map((_, y) => {
                return (<tr>
                  {
                    Array(boardWidth).fill(0).map((_, x) => {
                      const index = y * boardWidth + x;
                      const cellState = cellStates[index];
                      return (<td><GridField
                        key={index}
                        bombProbability={cellState.bombPercentage}
                        rupoorProbability={cellState.rupoorPercentage}
                        selectedState={cellState.selectedType}
                        index={index}
                        selectionChangedCallback={this.selectedChanged}
                        ranking={cellState.ranking}></GridField></td>)
                    })
                  }
                </tr>)
              })
            }
          </tbody>
        </table>
        <button onClick={this.resetBoard.bind(this)}>Reset</button>
        <div>Source code: <a href="https://github.com/lepelog/thrill-digger">GitHub</a></div>
      </div>
    );
  }
}

function App() {
  return (
    <TestComp/>
  );
}

export default App;
