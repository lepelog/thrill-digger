import { SolverWrapper } from "./thrill-digger-wasm/thrill_digger_wasm";
import { HoleContent } from "./types";

export interface ThrillDiggerInput {
  boardState: HoleContent[]; // 8 * 5 = 40
  selectedLoops: boolean[]; // 9
}

export interface ThrillDiggerOutput {
  probabilities: number[][]; // 40, 9
  possibleLoops: boolean[];
  possibleRngCount: number;
  ranks: number[]; // 40; rank 100 means it's not unspecified anymore
}

export function calculate(
  wasmMemory: WebAssembly.Memory,
  solver: SolverWrapper,
  input: ThrillDiggerInput
): ThrillDiggerOutput {
  const selectedLoops = new Uint8Array(
    wasmMemory.buffer,
    solver.get_input_selected_loops_ptr(),
    9
  );
  input.selectedLoops.forEach((selected, index) => {
    selectedLoops[index] = +selected;
  });
  const boardState = new Uint8Array(
    wasmMemory.buffer,
    solver.get_input_board_state_ptr(),
    40
  );
  input.boardState.forEach((content, index) => {
    boardState[index] = content;
  });
  solver.calculate_probabilities_with_pregenerated();
  // solver.change_possible_loops();
  const possibleLoops = Array.from(
    new Uint8Array(wasmMemory.buffer, solver.get_output_possible_loops_ptr(), 9)
  ).map((val) => val === 1);
  const probMemory = new Float32Array(
    wasmMemory.buffer,
    solver.get_output_probabilities_ptr(),
    40 * 8
  );
  const probabilities = Array(40)
    .fill(0)
    .map((_, index) =>
      Array.from(probMemory.slice(index * 8, (index + 1) * 8))
    );
  const ranks = Array.from(
    new Uint8Array(wasmMemory.buffer, solver.get_output_ranks_ptr(), 40)
  );
  const output: ThrillDiggerOutput = {
    possibleRngCount: solver.get_possible_rng_values_count(),
    possibleLoops,
    probabilities,
    ranks,
  };
  return output;
}
