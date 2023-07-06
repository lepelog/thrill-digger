<script lang="ts">
  import interpolate from "color-interpolate";
  import {
    HoleContent,
    type CellState,
    holeContentToStr,
    HOLE_STATES,
  } from "./types";
  import GreenRupee from "../assets/GreenRupee.webp";
  import BlueRupee from "../assets/BlueRupee.webp";
  import RedRupee from "../assets/RedRupee.webp";
  import SilverRupee from "../assets/SilverRupee.webp";
  import GoldRupee from "../assets/GoldRupee.webp";
  import Rupoor from "../assets/Rupoor.webp";
  import Bomb from "../assets/Bomb.webp";
  import Unspecified from "../assets/Unspecified.webp";

  export let selectChanged: (content: HoleContent) => void;
  export let cell: CellState;

  const IMAGES = [
    Unspecified,
    GreenRupee,
    BlueRupee,
    RedRupee,
    SilverRupee,
    GoldRupee,
    Rupoor,
    Bomb,
  ];

  const SAFEST_COLOR = "#53fc05";
  const ALMOST_SAFEST_COLOR = "#349b04";

  const firstTierInterpolation = interpolate([
    SAFEST_COLOR,
    ALMOST_SAFEST_COLOR,
  ]);
  const secondTierInterpolation = interpolate(["#fcf80c", "#a30800"]);

  function getBgColor(
    rank: number,
    bombProbability: number,
    rupoorProbability: number
  ): string {
    if (!isFinite(bombProbability)) {
      return "unset";
    }
    if (bombProbability === 0) {
      if (rupoorProbability === 0) {
        return SAFEST_COLOR;
      } else {
        return ALMOST_SAFEST_COLOR;
      }
    }
    const GOOD_RANK_COUNT = 4;
    if (rank < GOOD_RANK_COUNT) {
      return firstTierInterpolation(rank / GOOD_RANK_COUNT);
    } else {
      return secondTierInterpolation(bombProbability);
    }
  }

  $: bombProbability = cell.probabilities[HoleContent.Bomb];
  $: rupoorProbability = cell.probabilities[HoleContent.Rupoor];
  $: bgColor =
    cell.selectedType != HoleContent.Unspecified
      ? "unset"
      : getBgColor(cell.ranking, bombProbability, rupoorProbability); // TODO
</script>

<div class="grid-field" style:background={bgColor}>
  <div>bomb probability: {(bombProbability * 100).toFixed(2)}%</div>
  <div>rupoor probability: {(rupoorProbability * 100).toFixed(2)}%</div>
  <div>
    {#each HOLE_STATES as holeState, index}
      {#if cell.probabilities[index] !== 0}
        <img
          on:click={() => selectChanged(index)}
          class={(index === cell.selectedType
            ? "content-image-highlighted "
            : "") + "content-image"}
          src={IMAGES[index]}
          alt={holeContentToStr(holeState)}
          width="20"
        />
      {/if}
    {/each}
  </div>
</div>

<style>
  .grid-field {
    border: 4px solid black;
  }

  .content-image {
    border: 1px solid black;
    margin: 2px;
  }

  .content-image-highlighted {
    background: gray;
  }
</style>
