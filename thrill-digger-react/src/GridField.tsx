import React from 'react';
import interpolate from "color-interpolate";
import "./GridField.css";
import unspecified from './contents/Unspecified.webp'
import greenRupee from './contents/GreenRupee.webp'
import blueRupee from './contents/BlueRupee.webp'
import redRupee from './contents/RedRupee.webp'
import silverRupee from './contents/SilverRupee.webp'
import goldRupee from './contents/GoldRupee.webp'
import rupoor from './contents/Rupoor.webp'
import bomb from './contents/Bomb.webp'

export enum HoleContent {
  Unspecified = 0,
  GreenRupee = 1,
  BlueRupee = 2,
  RedRupee = 3,
  SilverRupee = 4,
  GoldRupee = 5,
  Rupoor = 6,
  Bomb = 7,
};

function holeContentToStr(hc: HoleContent): string {
  return HoleContent[hc];
}

type GridFieldProps = {
    bombProbability: number,
    rupoorProbability: number,
    selectedState: HoleContent,
    index: number,
    selectionChangedCallback: (index: number, newSelected: HoleContent) => void,
    ranking: number,
}

const holeStates = [
  HoleContent.Unspecified,
  HoleContent.GreenRupee,
  HoleContent.BlueRupee,
  HoleContent.RedRupee,
  HoleContent.SilverRupee,
  HoleContent.GoldRupee,
  HoleContent.Rupoor,
  HoleContent.Bomb,
];

const SAFEST_COLOR = '#53fc05';
const ALMOST_SAFEST_COLOR = '#349b04';

const firstTierInterpolation = interpolate([SAFEST_COLOR, ALMOST_SAFEST_COLOR]);
const secondTierInterpolation = interpolate(['#fcf80c', '#a30800']);

export class GridField extends React.Component<GridFieldProps, {}> {
    contentImages: string[];

    constructor(props: GridFieldProps) {
        super(props);
        this.contentImages = [
          unspecified,
          greenRupee,
          blueRupee,
          redRupee,
          silverRupee,
          goldRupee,
          rupoor,
          bomb
        ];
    }

    onSelectChange(newSelection: number) {
        this.props.selectionChangedCallback(this.props.index, newSelection);
    }

    getBgColor(rank: number, bombProbability: number, rupoorProbability: number): string {
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

    render() {
      const {bombProbability, rupoorProbability, selectedState, ranking} = this.props;
      const bgColor = selectedState === HoleContent.Unspecified ? this.getBgColor(ranking, bombProbability, rupoorProbability) : "unset";
      return (
        <div className="grid-field" style={{backgroundColor: bgColor}}>
          <div>bomb probability: {(bombProbability * 100).toFixed(2)}%</div>
          <div>rupoor probability: {(rupoorProbability * 100).toFixed(2)}%</div>
          <div>
              {holeStates.map((h, index) => {
                  return (
                    <img key={index} className={(index === selectedState ? "content-image-highlighted " : "") + "content-image"} src={this.contentImages[index]} alt={holeContentToStr(h)} onClick={this.onSelectChange.bind(this, index)} width={20} />
                  );
              })}
          </div>
        </div>
      );
    }
}
