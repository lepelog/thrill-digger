import React, { ChangeEvent } from 'react';
import interpolate from "color-interpolate";
import "./GridField.css";

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
    constructor(props: GridFieldProps) {
        super(props);

        this.onSelectChange = this.onSelectChange.bind(this);
    }

    onSelectChange(event: ChangeEvent<HTMLSelectElement>) {
        this.props.selectionChangedCallback(this.props.index, parseInt(event.target.value));
    }

    render() {
      const {bombProbability, rupoorProbability, selectedState, ranking} = this.props;
      const bgColor = selectedState === HoleContent.Unspecified ? goodnessInterpolation(isFinite(bombProbability) ? bombProbability : 0) : "unset";
      return (
        <div className="grid-field" style={{backgroundColor: bgColor, borderColor: ranking < 3 ? "#0011d3": "black"}}>
          <div>bomb probability: {(bombProbability * 100).toFixed(2)}%</div>
          <div>rupoor probability: {(rupoorProbability * 100).toFixed(2)}%</div>
          <div>
            <select onChange={this.onSelectChange} value={selectedState}>
                {holeStates.map(h => {
                    return (<option key={h} value={h}>{holeContentToStr(h)}</option>);
                })}
            </select>
          </div>
        </div>
      );
    }
}