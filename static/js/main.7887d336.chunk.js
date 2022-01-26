(this["webpackJsonpthrill-digger-react"]=this["webpackJsonpthrill-digger-react"]||[]).push([[0],{11:function(e,t,r){"use strict";(function(e){var n=r(13),i=r(3),o=r(4),s=r(5),a=r(7),c=r(6),l=r(1),u=r.n(l),d=(r(22),r(2)),b=r(0),p=function(t){Object(a.a)(u,t);var l=Object(c.a)(u);function u(e){var t;return Object(i.a)(this,u),(t=l.call(this,e)).state={nativeModule:null,currentMessage:"waiting to initialize...",solver:null,cellStates:[{bombPercentage:0,rupoorPercentage:0,selectedType:d.b.Unspecified,ranking:100}],boardHeight:0,boardWidth:0,possibleLoops:[],matchingSeedCount:-1},t.selectedChanged=t.selectedChanged.bind(Object(s.a)(t)),t}return Object(o.a)(u,[{key:"componentDidMount",value:function(){var t=this;r.e(3).then(r.bind(null,32)).then((function(r){t.setState({nativeModule:r,currentMessage:"computing values...",solver:r.create_solver()}),e((function(){var e;console.log("setting state"),null===(e=t.state.solver)||void 0===e||e.cache_boards();var r=t.state.solver.get_height(),n=t.state.solver.get_width(),i=Array(r*n).fill(0).map((function(){return{bombPercentage:0,rupoorPercentage:0,selectedType:d.b.Unspecified,ranking:100}}));t.setState({currentMessage:"done!",boardWidth:n,boardHeight:r,cellStates:i}),t.updateBoardAndRecalculateProbs(i)}))}))}},{key:"componentWillUnmount",value:function(){var e;null===(e=this.state.solver)||void 0===e||e.free()}},{key:"getSolverOrError",value:function(){if(null===this.state.solver)throw Error("solver is null!");return this.state.solver}},{key:"selectedChanged",value:function(e,t){var r=this.state.cellStates;r[e].selectedType=t,this.getSolverOrError().set_hole(e,t),this.updateBoardAndRecalculateProbs(r)}},{key:"updateBoardAndRecalculateProbs",value:function(e){var t=this.getSolverOrError();t.calculate_probabilities_with_pregenerated(),e.forEach((function(e,r){e.bombPercentage=t.get_probability(r),e.rupoorPercentage=t.get_rupoor_probability(r)}));var r=e.filter((function(e){return e.selectedType===d.b.Unspecified})).map((function(e,t){return[t,e]}));r.sort((function(e,t){return e[1].bombPercentage-t[1].bombPercentage||e[1].rupoorPercentage-t[1].rupoorPercentage})),r.forEach((function(e,t){var r=Object(n.a)(e,2);r[0];return r[1].ranking=t})),e.forEach((function(e){e.selectedType!==d.b.Unspecified&&(e.ranking=100)})),this.setState({cellStates:e,matchingSeedCount:t.get_possible_rng_values_count(),possibleLoops:this.getPossibleLoopArray(t)})}},{key:"getPossibleLoopArray",value:function(e){var t=new Uint8Array(9);e.get_possible_loops(t);var r=[];return t.forEach((function(e,t){r.push(1===e)})),r}},{key:"resetPossibleLoopArray",value:function(){this.getSolverOrError().reset_possible_loops(),this.updateBoardAndRecalculateProbs(this.state.cellStates)}},{key:"setPossibleLoop",value:function(e,t){this.getSolverOrError().set_possible_loop(e,t),this.updateBoardAndRecalculateProbs(this.state.cellStates)}},{key:"resetBoard",value:function(){for(var e,t,r=(null===(e=this.state.solver)||void 0===e?void 0:e.get_height())||0,n=(null===(t=this.state.solver)||void 0===t?void 0:t.get_width())||0,i=Array(r*n).fill(0).map((function(){return{bombPercentage:0,rupoorPercentage:0,selectedType:d.b.Unspecified,ranking:100}})),o=this.getSolverOrError(),s=0;s<r*n;s++)o.set_hole(s,d.b.Unspecified);this.updateBoardAndRecalculateProbs(i)}},{key:"render",value:function(){var e=this,t=this.state,r=t.boardHeight,n=t.boardWidth,i=t.cellStates,o=t.currentMessage,s=t.matchingSeedCount,a=t.possibleLoops;return Object(b.jsxs)("div",{className:"App",children:[Object(b.jsx)("h1",{children:"Thrill Digger Expert solver"}),Object(b.jsx)("div",{children:o}),Object(b.jsxs)("div",{children:["matching seeds: ",s]}),Object(b.jsxs)("div",{children:["Possible Loops: ",a.map((function(t,r){return Object(b.jsxs)("span",{children:[Object(b.jsx)("input",{type:"checkbox",checked:t,onChange:function(t){return e.setPossibleLoop(r,t.target.checked)}}),r]})}))]}),Object(b.jsx)("div",{children:Object(b.jsx)("button",{onClick:this.resetPossibleLoopArray.bind(this),children:"Reset Loops"})}),Object(b.jsx)("table",{children:Object(b.jsx)("tbody",{children:Array(r).fill(0).map((function(t,r){return Object(b.jsx)("tr",{children:Array(n).fill(0).map((function(t,o){var s=r*n+o,a=i[s];return Object(b.jsx)("td",{children:Object(b.jsx)(d.a,{bombProbability:a.bombPercentage,rupoorProbability:a.rupoorPercentage,selectedState:a.selectedType,index:s,selectionChangedCallback:e.selectedChanged,ranking:a.ranking},s)})}))})}))})}),Object(b.jsx)("button",{onClick:this.resetBoard.bind(this),children:"Reset"}),Object(b.jsxs)("div",{children:["Source code: ",Object(b.jsx)("a",{href:"https://github.com/lepelog/thrill-digger",children:"GitHub"})]})]})}}]),u}(u.a.Component);t.a=function(){return Object(b.jsx)(p,{})}}).call(this,r(19).setImmediate)},18:function(e,t,r){},2:function(e,t,r){"use strict";r.d(t,"b",(function(){return n})),r.d(t,"a",(function(){return P}));var n,i=r(3),o=r(4),s=r(7),a=r(6),c=r(1),l=r.n(c),u=r(12),d=r.n(u),b=(r(29),r.p+"static/media/Unspecified.2b9ce60f.png"),p=r.p+"static/media/GreenRupee.7b5504b7.png",h=r.p+"static/media/BlueRupee.5b70e0f7.png",g=r.p+"static/media/RedRupee.863b0080.png",v=r.p+"static/media/SilverRupee.a1abe29d.png",f=r.p+"static/media/GoldRupee.8fb25791.png",j=r.p+"static/media/Rupoor.2f12451b.png",m=r.p+"static/media/Bomb.3dc15ccf.png",O=r(0);!function(e){e[e.Unspecified=0]="Unspecified",e[e.GreenRupee=1]="GreenRupee",e[e.BlueRupee=2]="BlueRupee",e[e.RedRupee=3]="RedRupee",e[e.SilverRupee=4]="SilverRupee",e[e.GoldRupee=5]="GoldRupee",e[e.Rupoor=6]="Rupoor",e[e.Bomb=7]="Bomb"}(n||(n={}));var y=[n.Unspecified,n.GreenRupee,n.BlueRupee,n.RedRupee,n.SilverRupee,n.GoldRupee,n.Rupoor,n.Bomb],R=d()(["#2de500","#e5d200","#e50b00"]),P=function(e){Object(s.a)(r,e);var t=Object(a.a)(r);function r(e){var n;return Object(i.a)(this,r),(n=t.call(this,e)).contentImages=void 0,n.contentImages=[b,p,h,g,v,f,j,m],n}return Object(o.a)(r,[{key:"onSelectChange",value:function(e){this.props.selectionChangedCallback(this.props.index,e)}},{key:"render",value:function(){var e=this,t=this.props,r=t.bombProbability,i=t.rupoorProbability,o=t.selectedState,s=t.ranking,a=o===n.Unspecified?R(isFinite(r)?r:0):"unset";return Object(O.jsxs)("div",{className:"grid-field",style:{backgroundColor:a,borderColor:s<3?"#0011d3":"black"},children:[Object(O.jsxs)("div",{children:["bomb probability: ",(100*r).toFixed(2),"%"]}),Object(O.jsxs)("div",{children:["rupoor probability: ",(100*i).toFixed(2),"%"]}),Object(O.jsx)("div",{children:y.map((function(t,r){return Object(O.jsx)("img",{className:(r===o?"content-image-highlighted ":"")+"content-image",src:e.contentImages[r],alt:(i=t,n[i]),onClick:e.onSelectChange.bind(e,r),width:20},r);var i}))})]})}}]),r}(l.a.Component)},22:function(e,t,r){},29:function(e,t,r){},31:function(e,t,r){"use strict";r.r(t);var n=r(1),i=r.n(n),o=r(10),s=r.n(o),a=(r(18),r(11)),c=function(e){e&&e instanceof Function&&r.e(4).then(r.bind(null,36)).then((function(t){var r=t.getCLS,n=t.getFID,i=t.getFCP,o=t.getLCP,s=t.getTTFB;r(e),n(e),i(e),o(e),s(e)}))},l=r(0);s.a.render(Object(l.jsx)(i.a.StrictMode,{children:Object(l.jsx)(a.a,{})}),document.getElementById("root")),c()}},[[31,1,2]]]);
//# sourceMappingURL=main.7887d336.chunk.js.map