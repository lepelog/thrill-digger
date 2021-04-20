import React from 'react';
import interpolate from "color-interpolate";
import "./GridField.css";
import unspecified from './contents/Unspecified.png'
import greenRupee from './contents/GreenRupee.png'
import blueRupee from './contents/BlueRupee.png'
import redRupee from './contents/RedRupee.png'
import silverRupee from './contents/SilverRupee.png'
import goldRupee from './contents/GoldRupee.png'
import rupoor from './contents/Rupoor.png'
import bomb from './contents/Bomb.png'

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

const goodnessInterpolation = interpolate(['#2de500', '#e5d200', '#e50b00']);

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

    render() {
      const {bombProbability, rupoorProbability, selectedState, ranking} = this.props;
      const bgColor = selectedState === HoleContent.Unspecified ? goodnessInterpolation(isFinite(bombProbability) ? bombProbability : 0) : "unset";
      return (
        <div className="grid-field" style={{backgroundColor: bgColor, borderColor: ranking < 3 ? "#0011d3": "black"}}>
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