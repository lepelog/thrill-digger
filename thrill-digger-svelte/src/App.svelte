<script lang="ts">
  import ThrillDiggerSolver from "./lib/ThrillDiggerSolver.svelte";
  import init, { SolverWrapper, create_solver } from "./lib/thrill-digger-wasm";

  let solver: SolverWrapper | undefined;
  let solverInitialized = false;
  let wasmMemory: WebAssembly.Memory;

  init().then((value) => {
    solver = create_solver();
    solver.cache_boards();
    wasmMemory = value.memory;
    solverInitialized = true;
  });
</script>

<main>
  <h1>Thrill Digger Expert Solver</h1>
  {#if solverInitialized}
    <ThrillDiggerSolver {solver} {wasmMemory} />
  {:else}
    <div>Initializing...</div>
  {/if}
</main>

<style>
  main {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto",
      "Oxygen", "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans",
      "Helvetica Neue", sans-serif;
    text-align: center;
  }
</style>
