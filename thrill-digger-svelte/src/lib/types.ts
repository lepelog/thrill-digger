export enum HoleContent {
  Unspecified = 0,
  GreenRupee = 1,
  BlueRupee = 2,
  RedRupee = 3,
  SilverRupee = 4,
  GoldRupee = 5,
  Rupoor = 6,
  Bomb = 7,
}

export const HOLE_STATES = [
  HoleContent.Unspecified,
  HoleContent.GreenRupee,
  HoleContent.BlueRupee,
  HoleContent.RedRupee,
  HoleContent.SilverRupee,
  HoleContent.GoldRupee,
  HoleContent.Rupoor,
  HoleContent.Bomb,
];

export function holeContentToStr(hc: HoleContent): string {
  return HoleContent[hc];
}

export interface CellState {
  // indexed by HoleContent
  probabilities: number[];
  selectedType: HoleContent;
  ranking: number;
}
