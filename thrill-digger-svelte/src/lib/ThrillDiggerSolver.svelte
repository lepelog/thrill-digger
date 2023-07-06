<script lang="ts">
  import GridField from "./GridField.svelte";
  import { type SolverWrapper } from "./thrill-digger-wasm";
  import { HoleContent } from "./types";
  import { calculate, type ThrillDiggerOutput } from "./thrillDiggerWrapper";

  export let solver: SolverWrapper;
  export let wasmMemory: WebAssembly.Memory;

  type CellState = {
    // indexed by HoleContent
    probabilities: number[];
    selectedType: HoleContent;
    ranking: number;
  };

  const HEIGHT = solver.get_height();
  const WIDTH = solver.get_width();

  function initialLoops() {
    return Array(9).fill(true);
  }

  function initialBoardState() {
    return Array(40)
      .fill(0)
      .map((_) => HoleContent.Unspecified);
  }

  let selectedLoops = initialLoops();
  let boardState = initialBoardState();

  function selectChanged(index: number, selection: HoleContent) {
    if (boardState[index] == selection) {
      selection = HoleContent.Unspecified;
    }
    boardState[index] = selection;
  }

  let output: ThrillDiggerOutput;
  $: {
    output = calculate(wasmMemory, solver, { boardState, selectedLoops });
    console.log(output.possibleLoops);
    output.possibleLoops.forEach((val, index) => {
      if (!val) {
        selectedLoops[index] = false;
      }
    });
  }
</script>

<!-- <button on:click={test}>TEST</button> -->
<div>matching seeds: {output.possibleRngCount}</div>
{#each selectedLoops as val, idx}
  <span><input type="checkbox" bind:checked={selectedLoops[idx]} />{idx}</span>
{/each}
<div>Possible Loops:</div>
<div>
  <button on:click={() => (selectedLoops = initialLoops())}>Reset Loops</button>
</div>

<table>
  <tbody>
    {#each Array(HEIGHT).fill(0) as _, y}
      <tr>
        {#each Array(WIDTH).fill(0) as _, x}
          {@const index = x + y * WIDTH}
          <td>
            <GridField
              cell={{
                ranking: output.ranks[index],
                probabilities: output.probabilities[index],
                selectedType: boardState[index],
              }}
              selectChanged={(c) => selectChanged(index, c)}
            />
          </td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>

<button on:click={() => (boardState = initialBoardState())}>Reset Board</button>
