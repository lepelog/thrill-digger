var De=Object.defineProperty;var $e=(e,t,r)=>t in e?De(e,t,{enumerable:!0,configurable:!0,writable:!0,value:r}):e[t]=r;var ne=(e,t,r)=>($e(e,typeof t!="symbol"?t+"":t,r),r);(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))n(o);new MutationObserver(o=>{for(const l of o)if(l.type==="childList")for(const u of l.addedNodes)u.tagName==="LINK"&&u.rel==="modulepreload"&&n(u)}).observe(document,{childList:!0,subtree:!0});function r(o){const l={};return o.integrity&&(l.integrity=o.integrity),o.referrerPolicy&&(l.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?l.credentials="include":o.crossOrigin==="anonymous"?l.credentials="omit":l.credentials="same-origin",l}function n(o){if(o.ep)return;o.ep=!0;const l=r(o);fetch(o.href,l)}})();function q(){}function Ce(e){return e()}function me(){return Object.create(null)}function V(e){e.forEach(Ce)}function Xe(e){return typeof e=="function"}function ae(e,t){return e!=e?t==t:e!==t||e&&typeof e=="object"||typeof e=="function"}let H;function He(e,t){return e===t?!0:(H||(H=document.createElement("a")),H.href=t,e===H.href)}function Ye(e){return Object.keys(e).length===0}function ye(e){return e??""}function v(e,t){e.appendChild(t)}function O(e,t,r){e.insertBefore(t,r||null)}function k(e){e.parentNode&&e.parentNode.removeChild(e)}function ee(e,t){for(let r=0;r<e.length;r+=1)e[r]&&e[r].d(t)}function R(e){return document.createElement(e)}function T(e){return document.createTextNode(e)}function M(){return T(" ")}function xe(){return T("")}function te(e,t,r,n){return e.addEventListener(t,r,n),()=>e.removeEventListener(t,r,n)}function G(e,t,r){r==null?e.removeAttribute(t):e.getAttribute(t)!==r&&e.setAttribute(t,r)}function et(e){return Array.from(e.childNodes)}function oe(e,t){t=""+t,e.data!==t&&(e.data=t)}function we(e,t,r,n){r==null?e.style.removeProperty(t):e.style.setProperty(t,r,n?"important":"")}let ue;function D(e){ue=e}const N=[],Ae=[];let Z=[];const ve=[],tt=Promise.resolve();let ie=!1;function rt(){ie||(ie=!0,tt.then(Pe))}function se(e){Z.push(e)}const le=new Set;let P=0;function Pe(){if(P!==0)return;const e=ue;do{try{for(;P<N.length;){const t=N[P];P++,D(t),nt(t.$$)}}catch(t){throw N.length=0,P=0,t}for(D(null),N.length=0,P=0;Ae.length;)Ae.pop()();for(let t=0;t<Z.length;t+=1){const r=Z[t];le.has(r)||(le.add(r),r())}Z.length=0}while(N.length);for(;ve.length;)ve.pop()();ie=!1,le.clear(),D(e)}function nt(e){if(e.fragment!==null){e.update(),V(e.before_update);const t=e.dirty;e.dirty=[-1],e.fragment&&e.fragment.p(e.ctx,t),e.after_update.forEach(se)}}function lt(e){const t=[],r=[];Z.forEach(n=>e.indexOf(n)===-1?t.push(n):r.push(n)),r.forEach(n=>n()),Z=t}const Y=new Set;let Q;function fe(){Q={r:0,c:[],p:Q}}function ce(){Q.r||V(Q.c),Q=Q.p}function L(e,t){e&&e.i&&(Y.delete(e),e.i(t))}function j(e,t,r,n){if(e&&e.o){if(Y.has(e))return;Y.add(e),Q.c.push(()=>{Y.delete(e),n&&(r&&e.d(1),n())}),e.o(t)}else n&&n()}function W(e){return(e==null?void 0:e.length)!==void 0?e:Array.from(e)}function Ne(e){e&&e.c()}function _e(e,t,r){const{fragment:n,after_update:o}=e.$$;n&&n.m(t,r),se(()=>{const l=e.$$.on_mount.map(Ce).filter(Xe);e.$$.on_destroy?e.$$.on_destroy.push(...l):V(l),e.$$.on_mount=[]}),o.forEach(se)}function pe(e,t){const r=e.$$;r.fragment!==null&&(lt(r.after_update),V(r.on_destroy),r.fragment&&r.fragment.d(t),r.on_destroy=r.fragment=null,r.ctx=[])}function ot(e,t){e.$$.dirty[0]===-1&&(N.push(e),rt(),e.$$.dirty.fill(0)),e.$$.dirty[t/31|0]|=1<<t%31}function de(e,t,r,n,o,l,u,i=[-1]){const s=ue;D(e);const a=e.$$={fragment:null,ctx:[],props:l,update:q,not_equal:o,bound:me(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(t.context||(s?s.$$.context:[])),callbacks:me(),dirty:i,skip_bound:!1,root:t.target||s.$$.root};u&&u(a.root);let f=!1;if(a.ctx=r?r(e,t.props||{},(d,b,...m)=>{const I=m.length?m[0]:b;return a.ctx&&o(a.ctx[d],a.ctx[d]=I)&&(!a.skip_bound&&a.bound[d]&&a.bound[d](I),f&&ot(e,d)),b}):[],a.update(),f=!0,V(a.before_update),a.fragment=n?n(a.ctx):!1,t.target){if(t.hydrate){const d=et(t.target);a.fragment&&a.fragment.l(d),d.forEach(k)}else a.fragment&&a.fragment.c();t.intro&&L(e.$$.fragment),_e(e,t.target,t.anchor),Pe()}D(s)}class ge{constructor(){ne(this,"$$");ne(this,"$$set")}$destroy(){pe(this,1),this.$destroy=q}$on(t,r){if(!Xe(r))return q;const n=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return n.push(r),()=>{const o=n.indexOf(r);o!==-1&&n.splice(o,1)}}$set(t){this.$$set&&!Ye(t)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}const it="4";typeof window<"u"&&(window.__svelte||(window.__svelte={v:new Set})).v.add(it);function st(e){return e&&e.__esModule&&Object.prototype.hasOwnProperty.call(e,"default")?e.default:e}var at={aliceblue:[240,248,255],antiquewhite:[250,235,215],aqua:[0,255,255],aquamarine:[127,255,212],azure:[240,255,255],beige:[245,245,220],bisque:[255,228,196],black:[0,0,0],blanchedalmond:[255,235,205],blue:[0,0,255],blueviolet:[138,43,226],brown:[165,42,42],burlywood:[222,184,135],cadetblue:[95,158,160],chartreuse:[127,255,0],chocolate:[210,105,30],coral:[255,127,80],cornflowerblue:[100,149,237],cornsilk:[255,248,220],crimson:[220,20,60],cyan:[0,255,255],darkblue:[0,0,139],darkcyan:[0,139,139],darkgoldenrod:[184,134,11],darkgray:[169,169,169],darkgreen:[0,100,0],darkgrey:[169,169,169],darkkhaki:[189,183,107],darkmagenta:[139,0,139],darkolivegreen:[85,107,47],darkorange:[255,140,0],darkorchid:[153,50,204],darkred:[139,0,0],darksalmon:[233,150,122],darkseagreen:[143,188,143],darkslateblue:[72,61,139],darkslategray:[47,79,79],darkslategrey:[47,79,79],darkturquoise:[0,206,209],darkviolet:[148,0,211],deeppink:[255,20,147],deepskyblue:[0,191,255],dimgray:[105,105,105],dimgrey:[105,105,105],dodgerblue:[30,144,255],firebrick:[178,34,34],floralwhite:[255,250,240],forestgreen:[34,139,34],fuchsia:[255,0,255],gainsboro:[220,220,220],ghostwhite:[248,248,255],gold:[255,215,0],goldenrod:[218,165,32],gray:[128,128,128],green:[0,128,0],greenyellow:[173,255,47],grey:[128,128,128],honeydew:[240,255,240],hotpink:[255,105,180],indianred:[205,92,92],indigo:[75,0,130],ivory:[255,255,240],khaki:[240,230,140],lavender:[230,230,250],lavenderblush:[255,240,245],lawngreen:[124,252,0],lemonchiffon:[255,250,205],lightblue:[173,216,230],lightcoral:[240,128,128],lightcyan:[224,255,255],lightgoldenrodyellow:[250,250,210],lightgray:[211,211,211],lightgreen:[144,238,144],lightgrey:[211,211,211],lightpink:[255,182,193],lightsalmon:[255,160,122],lightseagreen:[32,178,170],lightskyblue:[135,206,250],lightslategray:[119,136,153],lightslategrey:[119,136,153],lightsteelblue:[176,196,222],lightyellow:[255,255,224],lime:[0,255,0],limegreen:[50,205,50],linen:[250,240,230],magenta:[255,0,255],maroon:[128,0,0],mediumaquamarine:[102,205,170],mediumblue:[0,0,205],mediumorchid:[186,85,211],mediumpurple:[147,112,219],mediumseagreen:[60,179,113],mediumslateblue:[123,104,238],mediumspringgreen:[0,250,154],mediumturquoise:[72,209,204],mediumvioletred:[199,21,133],midnightblue:[25,25,112],mintcream:[245,255,250],mistyrose:[255,228,225],moccasin:[255,228,181],navajowhite:[255,222,173],navy:[0,0,128],oldlace:[253,245,230],olive:[128,128,0],olivedrab:[107,142,35],orange:[255,165,0],orangered:[255,69,0],orchid:[218,112,214],palegoldenrod:[238,232,170],palegreen:[152,251,152],paleturquoise:[175,238,238],palevioletred:[219,112,147],papayawhip:[255,239,213],peachpuff:[255,218,185],peru:[205,133,63],pink:[255,192,203],plum:[221,160,221],powderblue:[176,224,230],purple:[128,0,128],rebeccapurple:[102,51,153],red:[255,0,0],rosybrown:[188,143,143],royalblue:[65,105,225],saddlebrown:[139,69,19],salmon:[250,128,114],sandybrown:[244,164,96],seagreen:[46,139,87],seashell:[255,245,238],sienna:[160,82,45],silver:[192,192,192],skyblue:[135,206,235],slateblue:[106,90,205],slategray:[112,128,144],slategrey:[112,128,144],snow:[255,250,250],springgreen:[0,255,127],steelblue:[70,130,180],tan:[210,180,140],teal:[0,128,128],thistle:[216,191,216],tomato:[255,99,71],turquoise:[64,224,208],violet:[238,130,238],wheat:[245,222,179],white:[255,255,255],whitesmoke:[245,245,245],yellow:[255,255,0],yellowgreen:[154,205,50]},ke=at,ut=ft,Re={red:0,orange:60,yellow:120,green:180,blue:240,purple:300};function ft(e){var t,r=[],n=1,o;if(typeof e=="string")if(ke[e])r=ke[e].slice(),o="rgb";else if(e==="transparent")n=0,o="rgb",r=[0,0,0];else if(/^#[A-Fa-f0-9]+$/.test(e)){var l=e.slice(1),u=l.length,i=u<=4;n=1,i?(r=[parseInt(l[0]+l[0],16),parseInt(l[1]+l[1],16),parseInt(l[2]+l[2],16)],u===4&&(n=parseInt(l[3]+l[3],16)/255)):(r=[parseInt(l[0]+l[1],16),parseInt(l[2]+l[3],16),parseInt(l[4]+l[5],16)],u===8&&(n=parseInt(l[6]+l[7],16)/255)),r[0]||(r[0]=0),r[1]||(r[1]=0),r[2]||(r[2]=0),o="rgb"}else if(t=/^((?:rgb|hs[lvb]|hwb|cmyk?|xy[zy]|gray|lab|lchu?v?|[ly]uv|lms)a?)\s*\(([^\)]*)\)/.exec(e)){var s=t[1],a=s==="rgb",l=s.replace(/a$/,"");o=l;var u=l==="cmyk"?4:l==="gray"?1:3;r=t[2].trim().split(/\s*[,\/]\s*|\s+/).map(function(b,m){if(/%$/.test(b))return m===u?parseFloat(b)/100:l==="rgb"?parseFloat(b)*255/100:parseFloat(b);if(l[m]==="h"){if(/deg$/.test(b))return parseFloat(b);if(Re[b]!==void 0)return Re[b]}return parseFloat(b)}),s===l&&r.push(1),n=a||r[u]===void 0?1:r[u],r=r.slice(0,u)}else e.length>10&&/[0-9](?:\s|\/)/.test(e)&&(r=e.match(/([0-9]+)/g).map(function(f){return parseFloat(f)}),o=e.match(/([a-z])/ig).join("").toLowerCase());else isNaN(e)?Array.isArray(e)||e.length?(r=[e[0],e[1],e[2]],o="rgb",n=e.length===4?e[3]:1):e instanceof Object&&(e.r!=null||e.red!=null||e.R!=null?(o="rgb",r=[e.r||e.red||e.R||0,e.g||e.green||e.G||0,e.b||e.blue||e.B||0]):(o="hsl",r=[e.h||e.hue||e.H||0,e.s||e.saturation||e.S||0,e.l||e.lightness||e.L||e.b||e.brightness]),n=e.a||e.alpha||e.opacity||1,e.opacity!=null&&(n/=100)):(o="rgb",r=[e>>>16,(e&65280)>>>8,e&255]);return{space:o,values:r,alpha:n}}var ct={name:"rgb",min:[0,0,0],max:[255,255,255],channel:["red","green","blue"],alias:["RGB"]},_t=ct,pt={name:"hsl",min:[0,0,0],max:[360,100,100],channel:["hue","saturation","lightness"],alias:["HSL"],rgb:function(e){var t=e[0]/360,r=e[1]/100,n=e[2]/100,o,l,u,i,s;if(r===0)return s=n*255,[s,s,s];n<.5?l=n*(1+r):l=n+r-n*r,o=2*n-l,i=[0,0,0];for(var a=0;a<3;a++)u=t+1/3*-(a-1),u<0?u++:u>1&&u--,6*u<1?s=o+(l-o)*6*u:2*u<1?s=l:3*u<2?s=o+(l-o)*(2/3-u)*6:s=o,i[a]=s*255;return i}};_t.hsl=function(e){var t=e[0]/255,r=e[1]/255,n=e[2]/255,o=Math.min(t,r,n),l=Math.max(t,r,n),u=l-o,i,s,a;return l===o?i=0:t===l?i=(r-n)/u:r===l?i=2+(n-t)/u:n===l&&(i=4+(t-r)/u),i=Math.min(i*60,360),i<0&&(i+=360),a=(o+l)/2,l===o?s=0:a<=.5?s=u/(l+o):s=u/(2-l-o),[i,s*100,a*100]};function dt(e,t,r){return e*(1-r)+t*r}var gt=dt,ht=bt;function bt(e,t,r){return t<r?e<t?t:e>r?r:e:e<r?r:e>t?t:e}const mt=ut,yt=pt,wt=gt,At=ht;var vt=kt;function kt(e){return e=e.map(function(t){if(t=mt(t),t.space!="rgb"){if(t.space!="hsl")throw"c.spacespace is not supported.";t.values=yt.rgb(t.values)}return t.values.push(t.alpha),t.values}),function(t,r){r=r||wt,t=At(t,0,1);var n=(e.length-1)*t,o=Math.floor(n),l=Math.ceil(n);t=n-o;var u=e[o],i=e[l],s=u.map(function(a,f){return a=r(a,i[f],t),f<3&&(a=Math.round(a)),a});return s[3]===1?"rgb("+s.slice(0,3)+")":"rgba("+s+")"}}const Se=st(vt);var X=(e=>(e[e.Unspecified=0]="Unspecified",e[e.GreenRupee=1]="GreenRupee",e[e.BlueRupee=2]="BlueRupee",e[e.RedRupee=3]="RedRupee",e[e.SilverRupee=4]="SilverRupee",e[e.GoldRupee=5]="GoldRupee",e[e.Rupoor=6]="Rupoor",e[e.Bomb=7]="Bomb",e))(X||{});const Oe=[0,1,2,3,4,5,6,7];function Rt(e){return X[e]}const St="/thrill-digger/assets/GreenRupee-3ebb2ebe.webp",Ot="/thrill-digger/assets/BlueRupee-c7f81d2e.webp",It="/thrill-digger/assets/RedRupee-fabb7539.webp",Et="/thrill-digger/assets/SilverRupee-f08d9b0a.webp",Tt="/thrill-digger/assets/GoldRupee-a6b43b25.webp",Lt="/thrill-digger/assets/Rupoor-8521d663.webp",Bt="/thrill-digger/assets/Bomb-35b9d7cb.webp",Mt="data:image/webp;base64,UklGRnAFAABXRUJQVlA4WAoAAAAQAAAAfQEAVgIAQUxQSBoAAAABBxARERASJP7fbiei/xn/+c9//vOf//y/AlZQOCAwBQAAEEwAnQEqfgFXAj5RKJBGo6KhoSJdiEBwCglpbuF2dvvAvkV2O9/0Ef49HGiVnRHc03Af+3Mz6Bf9T/5bJ32493zYB+il/W5WyMuum8rZGXXTeVsjLrpvK2Rl103lbIy66bytkZddN5WyMuum8rZGXXTeVsjLrpvK2Rl103lbIy66bytkZddN5WyMuum8rZGXXTeVsjLrpvK2Rl103lbIy66bySGRS/rcrZGXXTeVarMx3v7c33ToXeTrh68QkSRJEkSRJEkSRJEkSRJEkSRJDBct//juCZ3BM7gmdwTO4JncEzuCZ3BM7gmdwTO4JgQ1Mg3UtfIG614zWvyry3/P2il/W5WyMuum8rZGXXTeVsjLr9utytkZddN5WyMhzoSpS8VD2zYy7Da137MbhJVeXIy66bytkZddNep6Z5+0Uv63K2RltB0k8/aKX9blbIy6i5IH/W5WyMuum8rWVTCtpayH9WvsqHoKrS1kP6tfAdAVaWsh/Vr7Kh6Cq0tZD+rXvlhYR7WyMuum8rZGXXSI4vI4hI3UrsqHoKrS1kP6tfZTzRWFhzmef7B8/2D5/sHz/YPNZh51F2XII6K7sNNd+fHoQpRfzUhouHrxCRJEkSRJEkSRJEkSRJEkSRJEkSRJEkSQ1FUI9Gxgq8gtf/KKvI8MaQS/a7sFt9GjU3lbIy66bytjqHXFyMuum8rZGXUtLHaKX9blbIy66SfTeVsjLrpvK2RkZPEtkZddN5WyMupaN03lbIy66bytjld3trZGXXTeVsjIyZddN5WyMuum8q7tGpvK1hy4D9hGBTPtVBRQqWaW5WyKcAAA/v/UowAAAAAAAAIH6j2fTvgRcZ53l04gIb/b3/6ojA1er/BrPQqf/utQQ3/pURjOUm55TGDhpExWElg8m53aBh+yP/4D/0IHP//32z/+c8Z8LXc/Fi8CTQLpqymzmWtoJL8llpaFYHUlowylFq/ZsP+0v9ochLPI1a5NCVH9kJ/veEZhqqAyzOc9sCqVTnYMs9StngwpjkLoCbfo7O3WyPeFhOiO2gkT/eIKb1CnnPgoRinsToplEXWvCQB8PRqtOq06rTqtOq06rTq0fqlrqrC8FAs5zsu/NPTrfVvcLBHaBZznZd+aenW+re4WCO0CznOy7809Ot+ew8IDJVLOQqbbOMQMLOBHiQSG8SCQ3iQSG8SCQ3iQSG8SCQ3iQSG8SCPH9K2Uo7t2z00XdIc7OfpWu6Q52c/Std0hzs5+la7pDnZz9K13SHOzn6Vr/YRwBSI+8SDyEN7kilxgQIkAKJACiQAokAKJACiQAokAMJxO3fVXIOQAAAAAAAAAAAA0mtyWbroaJU7nlR65onvkGQW2zD0/RjbQ+VoPtGqcFolRXxpUV8aVFfGkk9rTNugS8ci5ypG6whh8jGT7VHNVw33dYyK63TjwM2ycFAWftIAgAAAAAAAAAAAAAIKAc6Gz90XxK/mFfcSh1aPaqk6uEpvBHs8MkalTc6Y83bWDA5N21gwOXwxCDAkjf9++J//9fN//nOtt3igAAAAAAAAh2vPOz5XA01qcuifpbFSa/rIt8VfULjIRJvPcQbx4F5YdkjaqTcPQrbPj/YM0wgh4ZHQK52teBUjphX/jEWceEZOkOWugPFaAzkFAzhDrf+uKuvG4pqjQqKTn6f9iUVEwX6utf0FoDicN3gYFv44gZR+j+LMRgzFXRPBxOrrLkthA1KcRr7anhaAcjXjqhu+mBZEvLT8ytUwoMo1GBX2/asLEASGOAAA=";function Ie(e,t,r){const n=e.slice();return n[10]=t[r],n[12]=r,n}function Ee(e){let t,r,n,o,l;function u(){return e[6](e[12])}return{c(){t=R("img"),G(t,"class",r=ye((e[12]===e[1].selectedType?"content-image-highlighted ":"")+"content-image")+" svelte-19xt5sk"),He(t.src,n=e[5][e[12]])||G(t,"src",n),G(t,"alt",Rt(e[10])),G(t,"width","20")},m(i,s){O(i,t,s),o||(l=te(t,"click",u),o=!0)},p(i,s){e=i,s&2&&r!==(r=ye((e[12]===e[1].selectedType?"content-image-highlighted ":"")+"content-image")+" svelte-19xt5sk")&&G(t,"class",r)},d(i){i&&k(t),o=!1,l()}}}function Te(e){let t,r=e[1].probabilities[e[12]]!==0&&Ee(e);return{c(){r&&r.c(),t=xe()},m(n,o){r&&r.m(n,o),O(n,t,o)},p(n,o){n[1].probabilities[n[12]]!==0?r?r.p(n,o):(r=Ee(n),r.c(),r.m(t.parentNode,t)):r&&(r.d(1),r=null)},d(n){n&&k(t),r&&r.d(n)}}}function Ft(e){let t,r,n,o=(e[3]*100).toFixed(2)+"",l,u,i,s,a,f=(e[2]*100).toFixed(2)+"",d,b,m,I,E=W(Oe),_=[];for(let h=0;h<E.length;h+=1)_[h]=Te(Ie(e,E,h));return{c(){t=R("div"),r=R("div"),n=T("bomb probability: "),l=T(o),u=T("%"),i=M(),s=R("div"),a=T("rupoor probability: "),d=T(f),b=T("%"),m=M(),I=R("div");for(let h=0;h<_.length;h+=1)_[h].c();G(t,"class","grid-field svelte-19xt5sk"),we(t,"background",e[4])},m(h,B){O(h,t,B),v(t,r),v(r,n),v(r,l),v(r,u),v(t,i),v(t,s),v(s,a),v(s,d),v(s,b),v(t,m),v(t,I);for(let w=0;w<_.length;w+=1)_[w]&&_[w].m(I,null)},p(h,[B]){if(B&8&&o!==(o=(h[3]*100).toFixed(2)+"")&&oe(l,o),B&4&&f!==(f=(h[2]*100).toFixed(2)+"")&&oe(d,f),B&35){E=W(Oe);let w;for(w=0;w<E.length;w+=1){const S=Ie(h,E,w);_[w]?_[w].p(S,B):(_[w]=Te(S),_[w].c(),_[w].m(I,null))}for(;w<_.length;w+=1)_[w].d(1);_.length=E.length}B&16&&we(t,"background",h[4])},i:q,o:q,d(h){h&&k(t),ee(_,h)}}}const Le="#53fc05",Be="#349b04";function Gt(e,t,r){let n,o,l,{selectChanged:u}=t,{cell:i}=t;const s=[Mt,St,Ot,It,Et,Tt,Lt,Bt],a=Se([Le,Be]),f=Se(["#fcf80c","#a30800"]);function d(m,I,E){if(!isFinite(I))return"unset";if(I===0)return E===0?Le:Be;const _=4;return m<_?a(m/_):f(I)}const b=m=>u(m);return e.$$set=m=>{"selectChanged"in m&&r(0,u=m.selectChanged),"cell"in m&&r(1,i=m.cell)},e.$$.update=()=>{e.$$.dirty&2&&r(3,n=i.probabilities[X.Bomb]),e.$$.dirty&2&&r(2,o=i.probabilities[X.Rupoor]),e.$$.dirty&14&&r(4,l=i.selectedType!=X.Unspecified?"unset":d(i.ranking,n,o))},[u,i,o,n,l,s,b]}class Wt extends ge{constructor(t){super(),de(this,t,Gt,Ft,ae,{selectChanged:0,cell:1})}}let g;const Ze=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&Ze.decode();let K=null;function he(){return(K===null||K.byteLength===0)&&(K=new Uint8Array(g.memory.buffer)),K}function Me(e,t){return e=e>>>0,Ze.decode(he().subarray(e,e+t))}const F=new Array(128).fill(void 0);F.push(void 0,null,!0,!1);let $=F.length;function Ve(e){$===F.length&&F.push(F.length+1);const t=$;return $=F[t],F[t]=e,t}function x(e){return F[e]}function qt(e){e<132||(F[e]=$,$=e)}function re(e){const t=x(e);return qt(e),t}function jt(e,t){return e=e>>>0,he().subarray(e/1,e/1+t)}let J=null;function C(){return(J===null||J.byteLength===0)&&(J=new Int32Array(g.memory.buffer)),J}let ze=0;function Ut(e,t){const r=t(e.length*1,1)>>>0;return he().set(e,r/1),ze=e.length,r}function Qt(){try{const n=g.__wbindgen_add_to_stack_pointer(-16);g.create_solver(n);var e=C()[n/4+0],t=C()[n/4+1],r=C()[n/4+2];if(r)throw re(t);return be.__wrap(e)}finally{g.__wbindgen_add_to_stack_pointer(16)}}class be{static __wrap(t){t=t>>>0;const r=Object.create(be.prototype);return r.__wbg_ptr=t,r}__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,t}free(){const t=this.__destroy_into_raw();g.__wbg_solverwrapper_free(t)}set_hole(t,r){try{const l=g.__wbindgen_add_to_stack_pointer(-16);g.solverwrapper_set_hole(l,this.__wbg_ptr,t,r);var n=C()[l/4+0],o=C()[l/4+1];if(o)throw re(n)}finally{g.__wbindgen_add_to_stack_pointer(16)}}calculate_probabilities_with_pregenerated(){g.solverwrapper_calculate_probabilities_with_pregenerated(this.__wbg_ptr)}get_a_probability(t,r){return g.solverwrapper_get_a_probability(this.__wbg_ptr,t,r)}get_possible_rng_values_count(){return g.solverwrapper_get_possible_rng_values_count(this.__wbg_ptr)>>>0}reset_possible_loops(){g.solverwrapper_reset_possible_loops(this.__wbg_ptr)}set_possible_loop(t,r){g.solverwrapper_set_possible_loop(this.__wbg_ptr,t,r)}get_possible_loops(t){try{const u=g.__wbindgen_add_to_stack_pointer(-16);var r=Ut(t,g.__wbindgen_malloc),n=ze;g.solverwrapper_get_possible_loops(u,this.__wbg_ptr,r,n,Ve(t));var o=C()[u/4+0],l=C()[u/4+1];if(l)throw re(o)}finally{g.__wbindgen_add_to_stack_pointer(16)}}get_total_loop_count(){return g.solverwrapper_get_total_loop_count(this.__wbg_ptr)>>>0}cache_boards(){g.solverwrapper_cache_boards(this.__wbg_ptr)}get_width(){return g.solverwrapper_get_width(this.__wbg_ptr)}get_height(){return g.solverwrapper_get_height(this.__wbg_ptr)}get_hole_count(){return g.solverwrapper_get_hole_count(this.__wbg_ptr)}get_input_board_state_ptr(){return g.solverwrapper_get_input_board_state_ptr(this.__wbg_ptr)}get_input_selected_loops_ptr(){return g.solverwrapper_get_input_selected_loops_ptr(this.__wbg_ptr)}get_output_possible_loops_ptr(){return g.solverwrapper_get_output_possible_loops_ptr(this.__wbg_ptr)}get_output_probabilities_ptr(){return g.solverwrapper_get_output_probabilities_ptr(this.__wbg_ptr)}get_output_ranks_ptr(){return g.solverwrapper_get_output_ranks_ptr(this.__wbg_ptr)}}async function Ct(e,t){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,t)}catch(n){if(e.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",n);else throw n}const r=await e.arrayBuffer();return await WebAssembly.instantiate(r,t)}else{const r=await WebAssembly.instantiate(e,t);return r instanceof WebAssembly.Instance?{instance:r,module:e}:r}}function Xt(){const e={};return e.wbg={},e.wbg.__wbindgen_string_new=function(t,r){const n=Me(t,r);return Ve(n)},e.wbg.__wbindgen_object_drop_ref=function(t){re(t)},e.wbg.__wbindgen_copy_to_typed_array=function(t,r,n){new Uint8Array(x(n).buffer,x(n).byteOffset,x(n).byteLength).set(jt(t,r))},e.wbg.__wbindgen_throw=function(t,r){throw new Error(Me(t,r))},e}function Pt(e,t){return g=e.exports,Ke.__wbindgen_wasm_module=t,J=null,K=null,g}async function Ke(e){if(g!==void 0)return g;typeof e>"u"&&(e=new URL("/thrill-digger/assets/thrill_digger_wasm_bg-6cb48f0c.wasm",self.location));const t=Xt();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:r,module:n}=await Ct(await e,t);return Pt(r,n)}function Nt(e,t,r){const n=new Uint8Array(e.buffer,t.get_input_selected_loops_ptr(),9);r.selectedLoops.forEach((f,d)=>{n[d]=+f});const o=new Uint8Array(e.buffer,t.get_input_board_state_ptr(),40);r.boardState.forEach((f,d)=>{o[d]=f}),t.calculate_probabilities_with_pregenerated();const l=Array.from(new Uint8Array(e.buffer,t.get_output_possible_loops_ptr(),9)).map(f=>f===1),u=new Float32Array(e.buffer,t.get_output_probabilities_ptr(),40*8),i=Array(40).fill(0).map((f,d)=>Array.from(u.slice(d*8,(d+1)*8))),s=Array.from(new Uint8Array(e.buffer,t.get_output_ranks_ptr(),40));return{possibleRngCount:t.get_possible_rng_values_count(),possibleLoops:l,probabilities:i,ranks:s}}function Fe(e,t,r){const n=e.slice();return n[13]=t[r],n[15]=r,n}function Ge(e,t,r){const n=e.slice();n[13]=t[r],n[18]=r;const o=n[18]+n[15]*n[4];return n[16]=o,n}function We(e,t,r){const n=e.slice();return n[19]=t[r],n[20]=t,n[21]=r,n}function qe(e){let t,r,n,o,l;function u(){e[9].call(r,e[21])}return{c(){t=R("span"),r=R("input"),n=T(e[21]),G(r,"type","checkbox")},m(i,s){O(i,t,s),v(t,r),r.checked=e[0][e[21]],v(t,n),o||(l=te(r,"change",u),o=!0)},p(i,s){e=i,s&1&&(r.checked=e[0][e[21]])},d(i){i&&k(t),o=!1,l()}}}function je(e){let t,r,n;function o(...l){return e[11](e[16],...l)}return r=new Wt({props:{cell:{ranking:e[2].ranks[e[16]],probabilities:e[2].probabilities[e[16]],selectedType:e[1][e[16]]},selectChanged:o}}),{c(){t=R("td"),Ne(r.$$.fragment)},m(l,u){O(l,t,u),_e(r,t,null),n=!0},p(l,u){e=l;const i={};u&6&&(i.cell={ranking:e[2].ranks[e[16]],probabilities:e[2].probabilities[e[16]],selectedType:e[1][e[16]]}),r.$set(i)},i(l){n||(L(r.$$.fragment,l),n=!0)},o(l){j(r.$$.fragment,l),n=!1},d(l){l&&k(t),pe(r)}}}function Ue(e){let t,r,n,o=W(Array(e[4]).fill(0)),l=[];for(let i=0;i<o.length;i+=1)l[i]=je(Ge(e,o,i));const u=i=>j(l[i],1,1,()=>{l[i]=null});return{c(){t=R("tr");for(let i=0;i<l.length;i+=1)l[i].c();r=M()},m(i,s){O(i,t,s);for(let a=0;a<l.length;a+=1)l[a]&&l[a].m(t,null);v(t,r),n=!0},p(i,s){if(s&86){o=W(Array(i[4]).fill(0));let a;for(a=0;a<o.length;a+=1){const f=Ge(i,o,a);l[a]?(l[a].p(f,s),L(l[a],1)):(l[a]=je(f),l[a].c(),L(l[a],1),l[a].m(t,r))}for(fe(),a=o.length;a<l.length;a+=1)u(a);ce()}},i(i){if(!n){for(let s=0;s<o.length;s+=1)L(l[s]);n=!0}},o(i){l=l.filter(Boolean);for(let s=0;s<l.length;s+=1)j(l[s]);n=!1},d(i){i&&k(t),ee(l,i)}}}function Zt(e){let t,r,n=e[2].possibleRngCount+"",o,l,u,i,s,a,f,d,b,m,I,E,_,h,B,w=W(e[0]),S=[];for(let c=0;c<w.length;c+=1)S[c]=qe(We(e,w,c));let U=W(Array(e[3]).fill(0)),A=[];for(let c=0;c<U.length;c+=1)A[c]=Ue(Fe(e,U,c));const Je=c=>j(A[c],1,1,()=>{A[c]=null});return{c(){t=R("div"),r=T("matching seeds: "),o=T(n),l=M();for(let c=0;c<S.length;c+=1)S[c].c();u=M(),i=R("div"),i.textContent="Possible Loops:",s=M(),a=R("div"),f=R("button"),f.textContent="Reset Loops",d=M(),b=R("table"),m=R("tbody");for(let c=0;c<A.length;c+=1)A[c].c();I=M(),E=R("button"),E.textContent="Reset Board"},m(c,y){O(c,t,y),v(t,r),v(t,o),O(c,l,y);for(let p=0;p<S.length;p+=1)S[p]&&S[p].m(c,y);O(c,u,y),O(c,i,y),O(c,s,y),O(c,a,y),v(a,f),O(c,d,y),O(c,b,y),v(b,m);for(let p=0;p<A.length;p+=1)A[p]&&A[p].m(m,null);O(c,I,y),O(c,E,y),_=!0,h||(B=[te(f,"click",e[10]),te(E,"click",e[12])],h=!0)},p(c,[y]){if((!_||y&4)&&n!==(n=c[2].possibleRngCount+"")&&oe(o,n),y&1){w=W(c[0]);let p;for(p=0;p<w.length;p+=1){const z=We(c,w,p);S[p]?S[p].p(z,y):(S[p]=qe(z),S[p].c(),S[p].m(u.parentNode,u))}for(;p<S.length;p+=1)S[p].d(1);S.length=w.length}if(y&86){U=W(Array(c[3]).fill(0));let p;for(p=0;p<U.length;p+=1){const z=Fe(c,U,p);A[p]?(A[p].p(z,y),L(A[p],1)):(A[p]=Ue(z),A[p].c(),L(A[p],1),A[p].m(m,null))}for(fe(),p=U.length;p<A.length;p+=1)Je(p);ce()}},i(c){if(!_){for(let y=0;y<U.length;y+=1)L(A[y]);_=!0}},o(c){A=A.filter(Boolean);for(let y=0;y<A.length;y+=1)j(A[y]);_=!1},d(c){c&&(k(t),k(l),k(u),k(i),k(s),k(a),k(d),k(b),k(I),k(E)),ee(S,c),ee(A,c),h=!1,V(B)}}}function Qe(){return Array(9).fill(!0)}function Vt(e,t,r){let{solver:n}=t,{wasmMemory:o}=t;const l=n.get_height(),u=n.get_width();function i(){return Array(40).fill(0).map(_=>X.Unspecified)}let s=Qe(),a=i();function f(_,h){a[_]==h&&(h=X.Unspecified),r(1,a[_]=h,a)}let d;function b(_){s[_]=this.checked,r(0,s),r(8,o),r(7,n),r(1,a),r(2,d)}const m=()=>r(0,s=Qe()),I=(_,h)=>f(_,h),E=()=>r(1,a=i());return e.$$set=_=>{"solver"in _&&r(7,n=_.solver),"wasmMemory"in _&&r(8,o=_.wasmMemory)},e.$$.update=()=>{e.$$.dirty&391&&(r(2,d=Nt(o,n,{boardState:a,selectedLoops:s})),console.log(d.possibleLoops),d.possibleLoops.forEach((_,h)=>{_||r(0,s[h]=!1,s)}))},[s,a,d,l,u,i,f,n,o,b,m,I,E]}class zt extends ge{constructor(t){super(),de(this,t,Vt,Zt,ae,{solver:7,wasmMemory:8})}}function Kt(e){let t;return{c(){t=R("div"),t.textContent="Initializing..."},m(r,n){O(r,t,n)},p:q,i:q,o:q,d(r){r&&k(t)}}}function Jt(e){let t,r;return t=new zt({props:{solver:e[0],wasmMemory:e[2]}}),{c(){Ne(t.$$.fragment)},m(n,o){_e(t,n,o),r=!0},p(n,o){const l={};o&1&&(l.solver=n[0]),o&4&&(l.wasmMemory=n[2]),t.$set(l)},i(n){r||(L(t.$$.fragment,n),r=!0)},o(n){j(t.$$.fragment,n),r=!1},d(n){pe(t,n)}}}function Dt(e){let t,r,n,o,l,u;const i=[Jt,Kt],s=[];function a(f,d){return f[1]?0:1}return o=a(e),l=s[o]=i[o](e),{c(){t=R("main"),r=R("h1"),r.textContent="Thrill Digger Expert Solver",n=M(),l.c(),G(t,"class","svelte-1qepwnq")},m(f,d){O(f,t,d),v(t,r),v(t,n),s[o].m(t,null),u=!0},p(f,[d]){let b=o;o=a(f),o===b?s[o].p(f,d):(fe(),j(s[b],1,1,()=>{s[b]=null}),ce(),l=s[o],l?l.p(f,d):(l=s[o]=i[o](f),l.c()),L(l,1),l.m(t,null))},i(f){u||(L(l),u=!0)},o(f){j(l),u=!1},d(f){f&&k(t),s[o].d()}}}function $t(e,t,r){let n,o=!1,l;return Ke().then(u=>{r(0,n=Qt()),n.cache_boards(),r(2,l=u.memory),r(1,o=!0)}),[n,o,l]}class Ht extends ge{constructor(t){super(),de(this,t,$t,Dt,ae,{})}}new Ht({target:document.getElementById("app")});
