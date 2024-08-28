(()=>{"use strict";var n,e,t,o,i,r,a={365:(n,e,t)=>{t.d(e,{A:()=>p});var o=t(601),i=t.n(o),r=t(314),a=t.n(r),l=t(417),s=t.n(l),c=new URL(t(465),t.b),d=a()(i()),u=s()(c);d.push([n.id,`@font-face {\n    font-family: 'DM Serif Display';\n    font-style: normal;\n    font-weight: 400;\n    font-display: swap;\n    src: url(${u}) format('woff2');\n    unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;\n}\n  \nbody {\n    font-family: "Roboto", "Helvetica Neue", Arial, sans-serif;\n    font-size: 20px;\n}\n\n.pageContainer {\n    width: 50em;\n    position: absolute;\n    left: 50%;\n    transform: translateX(-50%);\n}\n\nh1 {\n    text-align: center;\n    font-weight: normal;\n    font-size: 280%;\n    font-family: 'DM Serif Display', serif;\n    margin: 0.3em 0 0.7em 0;\n    padding: 0;\n    line-height: 1.1em;\n}\n\n.app>h2 {\n    font-family: "Roboto", "Helvetica Neue", Arial, sans-serif;\n    font-weight: normal;\n    text-align: center;\n    font-size: 160%;\n    margin: 0.2em 0;\n}\n\n.legendItems>h3 {\n    font-family: "Roboto", "Helvetica Neue", Arial, sans-serif;\n    font-size: 110%;\n    font-weight: bold;\n    text-align: center;\n    margin: 0 0 0.3em 0;\n    display: block;\n    white-space: nowrap;\n}\n\nh2 {\n    font-size: 230%;\n    font-weight: normal;\n    font-family: 'DM Serif Display', serif;\n    margin: 1.4em 0 0.6em 0;\n    line-height: 100%;\n}\n\nh3 {\n    font-family: 'DM Serif Display', serif;\n    font-size: 125%;\n    font-weight: 700;\n    margin: 1em 0.4em -0.3em 0;\n    padding: 0;\n    display: inline;\n}\n\n.prose {\n    color: #333;\n    line-height: 150%;\n}\n\n.prose li {\n    margin-top: 0.3em;\n}\n\nfooter {\n    opacity: 0.7;\n    font-size: 80%;\n    margin: 2em 0 1em 0;\n}\n\n.subtitle {\n    font-size: 60%;\n    display: block;\n}\n\n.splashScreen {\n    text-align: center;\n    font-size: 120%;\n    margin: 4em 2em 7em 2em;\n}\n\n#wasmErrorMessage {\n    text-align: center;\n}\n\n#wasmErrorMessage>table {\n    border: 0;\n    width: 100%;\n}\n\n#wasmErrorMessage td {\n    width: 20%;\n    text-align: center;\n    vertical-align: top;\n}\n\n.appContainer {\n    width: 40em;\n}\n\n.app {\n    display: none;\n}\n\n#downloadProgressPane {\n    color: #0d441d;\n}\n\n.progressBarContainer {\n    margin: 0.7em;\n    border: #36864e solid 0.1em;\n    border-radius: 0.3em;\n    position: relative;\n    height: 1.5em;\n}\n\n.progressBar {\n    background: #bfdfc9;\n    position: absolute;\n    left: 0;\n    top: 0;\n    bottom: 0;\n    width: 0;\n    z-index: 0;\n    border-radius: 0.15em;\n}\n\n.progressText {\n    position: absolute;\n    margin: auto;\n    left: 0;\n    right: 0;\n    top: 0.1em;\n    text-shadow: 0 0 0.2em #fff;\n}\n\n.plotAndLegend {\n    position: relative;\n    margin: 1.3em -2% 0 -2%;\n}\n\n.plotContainer {\n    position: relative;\n    left: -2em;\n    width: 41em;\n}\n\n.centered {\n    text-align: center;\n}\n\n.wordInput {\n    margin: 0 0 0.1em 0;\n    font-size: 160%;\n    width: 40%;\n    text-align: center;\n}\n\ninput {\n    border: solid 0.05em #ccc;\n    border-radius: 0.2em;\n    padding: 0.2em;\n}\n\ninput:focus {\n    border-color: #86bdf0;\n    background: #fafbfd;\n    outline: none;\n}\n\ninput.invalid {\n    background-color: #fdebeb;\n}\n\ninput.invalid:focus {\n    border-color: #9c5555;\n}\n\n.tooltipContent .explanation {\n    font-size: 90%;\n}\n\n.tooltipContent th {\n    font-size: 120%;\n}\n\n.tooltipContent th, .tooltipContent td {\n    vertical-align: baseline;\n}\n\n.tooltipContent th.left, .tooltipContent td.left {\n    padding-right: 0.3em;\n}\n\n.tooltipContent th.right, .tooltipContent td.right {\n    padding-left: 0.3em;\n}\n\n.tooltipContent .year {\n    font-size: 140%;\n    font-weight: bold;\n}\n\n.tooltipContent .interleave {\n    padding: 0.2em 0 0.5em 0;\n}\n\n.hint {\n    opacity: 0.7;\n    font-size: 80%;\n    margin-top: 0.1em;\n}\n\n.shareContainer {\n    margin: -0.3em 0.3em 0 0;\n    white-space: nowrap;\n    color: #555;\n    font-size: 80%;\n}\n\n.shareContainer::before {\n    display: inline-block;\n    vertical-align: top;\n    padding-top: 0.1em;\n    content: 'Share current plot on:';\n}\n\n.legend.empty>.shareContainer::before {\n    content: 'Share this app on:';\n    display: block;\n}\n\n.shareContainer>a {\n    display: inline-block;\n    vertical-align: top;\n    margin-left: 0.2em;\n}\n\n.legend.empty>.shareContainer>a {\n    margin: 0.4em 0.4em 0.2em 0.4em;\n}\n\n.shareContainer>a>img {\n    width: 1.5em;\n    height: 1.5em;\n    opacity: 0.6;\n    transition: opacity 0.1s;\n}\n\n.shareContainer>a:hover>img {\n    opacity: 1;\n}\n\n.toast {\n    position: relative;\n    left: -0.9em;\n    display: none;\n    opacity: 0;\n    transition: opacity 0.8s;\n}\n\n.toast>span {\n    display: block;\n    position: absolute;\n    top: 1.1em;\n    transform: translateX(-50%);\n    white-space: nowrap;\n    background: #cddbc5;\n    border: 0.2em solid #fff;\n    border-radius: 0.4em;\n    padding: 0.3em 0.5em;\n    font-size: 110%;\n    color: #1b3e14;\n    text-align: center;\n}\n\n.wordInputError {\n    position: absolute;\n    margin: 0.2em 0 0 0.6em;\n    color: #821313;\n    display: none;\n    line-height: 1.05em;\n    text-align: left;\n}\n\n.tooltipContent table {\n    width: 100%;\n}\n\n.tooltipContent .wait {\n    display: none;\n    position: absolute;\n    text-align: center;\n    width: 10em;\n    white-space: normal;\n    left: 50%;\n    top: 50%;\n    transform: translate(-50%, -50%);\n    opacity: 0.8;\n    font-size: 80%;\n}\n\n.tooltipContent.waiting .wait {\n    display: block;\n}\n\n.tooltipContent .suggestionsTable {\n    visibility: visible;\n}\n\n.tooltipContent.waiting .suggestionsTable {\n    visibility: hidden;\n}\n\n.tooltipContent .suggestion {\n    padding: 0.05em 0;\n    font-size: 90%;\n}\n\n.tooltipContent .left {\n    text-align: left;\n}\n\n.tooltipContent .right {\n    text-align: right;\n}\n\n.tooltipContent .removeWordButton {\n    background-color: #BCB8AE;\n    /*Mindful Gray*/\n    color: white;\n    font-size: 80%;\n}\n\na {\n    text-decoration: none;\n    color: #1135a0;\n}\n\n.legend {\n    background-color: #e8e8e8;\n    border-radius: 0.5em;\n    padding: 0.6em 0.7em 0.2em 0.6em;\n    width: auto;\n    position: absolute;\n    left: 40em;\n    top: 0;\n}\n\n.legend.empty {\n    text-align: center;\n}\n\n.legend.empty>.legendItems {\n    display: none;\n}\n\n.legendItems>ul {\n    list-style-type: none;\n    margin: 0 0 0.5em 0;\n    padding: 0;\n}\n\n.legendItems>ul>li {\n    margin: -0.2em -0.4em 0.0em -0.4em;\n    padding: 0.2em 0.4em 0.3em 0.4em;\n    border-radius: 0.2em;\n    white-space: nowrap;\n    cursor: default;\n}\n\n.legendItems>ul>li:hover, .legendItems>ul>li.hovering {\n    background-color: #f8f8f8;\n}\n\n.legendItems>ul>li>a {\n    text-decoration: none;\n    color: #152d74;\n}\n\n.legendItems>ul>li::before {\n    content: "———";\n    font-family: monospace;\n    margin-right: 0.1em;\n    font-weight: bold;\n}\n\n.legendItems>ul>li.inactive::before {\n    visibility: hidden;\n}\n\n#manualComparisons>li::before {\n    content: "— —";\n}\n\n.legendItems>ul>li.color0::before {\n    color: #f94a01;\n}\n\n.legendItems>ul>li.color1::before {\n    color: #6b42b6;\n}\n\n.legendItems>ul>li.color2::before {\n    color: #11a854;\n}\n\n.legendItems>ul>li.color3::before {\n    color: #128db2;\n}\n\n.legendItems>ul>li.color4::before {\n    color: #e12fbc;\n}\n\n.legendItems>ul>li.color5::before {\n    color: #e6ab02;\n}\n\n.legendItems>ul>li.color6::before {\n    color: #b40e0e;\n}\n\n.legendItems>ul>li.color7::before {\n    color: #4257b6;\n}\n\n.legendItems>ul>li.color8::before {\n    color: #18a380;\n}\n\n.legendItems>ul>li.color9::before {\n    color: #ad27c5;\n}\n\n.legendItems>ul>li.color10::before {\n    color: #2e2e2e;\n}\n\n.manualComparisonInput {\n    font-size: 80%;\n    min-width: 5.2em;\n}\n\n.legendItems input[type="button"] {\n    font-family: "Roboto", "Helvetica Neue", Arial, sans-serif;\n    font-size: 90%;\n    width: 1.4em;\n    height: 1.4em;\n    line-height: 0;\n}\n\n.inputWidthMeasure {\n    font-family: "Roboto", "Helvetica Neue", Arial, sans-serif;\n    font-size: 90%;\n    white-space: nowrap;\n    position: absolute;\n    opacity: 0;\n}\n\n.pinWordButton {\n    background-color: #BCB8AE;\n    /*Mindful Gray*/\n    border: none;\n    color: white;\n    padding: 10px 32px;\n    text-align: center;\n    text-decoration: none;\n    display: inline-block;\n    font-size: 80%;\n}\n\n.getUrlButton {\n    background-color: #BCB8AE;\n    /*Mindful Gray*/\n    border: none;\n    color: white;\n    padding: 5px 5px;\n    text-align: center;\n    text-decoration: none;\n    display: inline-block;\n    font-size: 80%;\n}\n\n.github-corner>svg {\n    fill: #666;\n    color: #fff;\n    position: absolute;\n    top: 0;\n    border: 0;\n    right: 0;\n    width: 3.5em;\n    height: 3.5em;\n}\n\n.github-corner:hover .octo-arm {\n    animation: octocat-wave 560ms ease-in-out;\n}\n\n@keyframes octocat-wave {\n    0%, 100% {\n        transform: rotate(0);\n    }\n    20%, 60% {\n        transform: rotate(-25deg);\n    }\n    40%, 80% {\n        transform: rotate(10deg);\n    }\n}\n\n@media (max-width: 70em) {\n    body {\n        font-size: 16px;\n    }\n    .pageContainer {\n        width: 95%;\n    }\n    .appContainer {\n        width: 100%;\n    }\n    .plotContainer {\n        left: 0;\n        width: 68%;\n    }\n    .legend {\n        left: 70%;\n        font-size: 75%;\n    }\n    .github-corner>svg {\n        width: 2.5em;\n        height: 2.5em;\n    }\n}\n`,""]);const p=d},314:n=>{n.exports=function(n){var e=[];return e.toString=function(){return this.map((function(e){var t="",o=void 0!==e[5];return e[4]&&(t+="@supports (".concat(e[4],") {")),e[2]&&(t+="@media ".concat(e[2]," {")),o&&(t+="@layer".concat(e[5].length>0?" ".concat(e[5]):""," {")),t+=n(e),o&&(t+="}"),e[2]&&(t+="}"),e[4]&&(t+="}"),t})).join("")},e.i=function(n,t,o,i,r){"string"==typeof n&&(n=[[null,n,void 0]]);var a={};if(o)for(var l=0;l<this.length;l++){var s=this[l][0];null!=s&&(a[s]=!0)}for(var c=0;c<n.length;c++){var d=[].concat(n[c]);o&&a[d[0]]||(void 0!==r&&(void 0===d[5]||(d[1]="@layer".concat(d[5].length>0?" ".concat(d[5]):""," {").concat(d[1],"}")),d[5]=r),t&&(d[2]?(d[1]="@media ".concat(d[2]," {").concat(d[1],"}"),d[2]=t):d[2]=t),i&&(d[4]?(d[1]="@supports (".concat(d[4],") {").concat(d[1],"}"),d[4]=i):d[4]="".concat(i)),e.push(d))}},e}},417:n=>{n.exports=function(n,e){return e||(e={}),n?(n=String(n.__esModule?n.default:n),/^['"].*['"]$/.test(n)&&(n=n.slice(1,-1)),e.hash&&(n+=e.hash),/["'() \t\n]|(%20)/.test(n)||e.needQuotes?'"'.concat(n.replace(/"/g,'\\"').replace(/\n/g,"\\n"),'"'):n):n}},601:n=>{n.exports=function(n){return n[1]}},72:n=>{var e=[];function t(n){for(var t=-1,o=0;o<e.length;o++)if(e[o].identifier===n){t=o;break}return t}function o(n,o){for(var r={},a=[],l=0;l<n.length;l++){var s=n[l],c=o.base?s[0]+o.base:s[0],d=r[c]||0,u="".concat(c," ").concat(d);r[c]=d+1;var p=t(u),m={css:s[1],media:s[2],sourceMap:s[3],supports:s[4],layer:s[5]};if(-1!==p)e[p].references++,e[p].updater(m);else{var f=i(m,o);o.byIndex=l,e.splice(l,0,{identifier:u,updater:f,references:1})}a.push(u)}return a}function i(n,e){var t=e.domAPI(e);return t.update(n),function(e){if(e){if(e.css===n.css&&e.media===n.media&&e.sourceMap===n.sourceMap&&e.supports===n.supports&&e.layer===n.layer)return;t.update(n=e)}else t.remove()}}n.exports=function(n,i){var r=o(n=n||[],i=i||{});return function(n){n=n||[];for(var a=0;a<r.length;a++){var l=t(r[a]);e[l].references--}for(var s=o(n,i),c=0;c<r.length;c++){var d=t(r[c]);0===e[d].references&&(e[d].updater(),e.splice(d,1))}r=s}}},659:n=>{var e={};n.exports=function(n,t){var o=function(n){if(void 0===e[n]){var t=document.querySelector(n);if(window.HTMLIFrameElement&&t instanceof window.HTMLIFrameElement)try{t=t.contentDocument.head}catch(n){t=null}e[n]=t}return e[n]}(n);if(!o)throw new Error("Couldn't find a style target. This probably means that the value for the 'insert' parameter is invalid.");o.appendChild(t)}},540:n=>{n.exports=function(n){var e=document.createElement("style");return n.setAttributes(e,n.attributes),n.insert(e,n.options),e}},56:(n,e,t)=>{n.exports=function(n){var e=t.nc;e&&n.setAttribute("nonce",e)}},825:n=>{n.exports=function(n){if("undefined"==typeof document)return{update:function(){},remove:function(){}};var e=n.insertStyleElement(n);return{update:function(t){!function(n,e,t){var o="";t.supports&&(o+="@supports (".concat(t.supports,") {")),t.media&&(o+="@media ".concat(t.media," {"));var i=void 0!==t.layer;i&&(o+="@layer".concat(t.layer.length>0?" ".concat(t.layer):""," {")),o+=t.css,i&&(o+="}"),t.media&&(o+="}"),t.supports&&(o+="}");var r=t.sourceMap;r&&"undefined"!=typeof btoa&&(o+="\n/*# sourceMappingURL=data:application/json;base64,".concat(btoa(unescape(encodeURIComponent(JSON.stringify(r))))," */")),e.styleTagTransform(o,n,e.options)}(e,n,t)},remove:function(){!function(n){if(null===n.parentNode)return!1;n.parentNode.removeChild(n)}(e)}}}},113:n=>{n.exports=function(n,e){if(e.styleSheet)e.styleSheet.cssText=n;else{for(;e.firstChild;)e.removeChild(e.firstChild);e.appendChild(document.createTextNode(n))}}},465:(n,e,t)=>{n.exports=t.p+"48631b1f1c0f8649c924.woff2"}},l={};function s(n){var e=l[n];if(void 0!==e)return e.exports;var t=l[n]={id:n,exports:{}};return a[n](t,t.exports,s),t.exports}s.m=a,n="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",e="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",o=n=>{n&&n.d<1&&(n.d=1,n.forEach((n=>n.r--)),n.forEach((n=>n.r--?n.r++:n())))},s.a=(i,r,a)=>{var l;a&&((l=[]).d=-1);var s,c,d,u=new Set,p=i.exports,m=new Promise(((n,e)=>{d=e,c=n}));m[e]=p,m[n]=n=>(l&&n(l),u.forEach(n),m.catch((n=>{}))),i.exports=m,r((i=>{var r;s=(i=>i.map((i=>{if(null!==i&&"object"==typeof i){if(i[n])return i;if(i.then){var r=[];r.d=0,i.then((n=>{a[e]=n,o(r)}),(n=>{a[t]=n,o(r)}));var a={};return a[n]=n=>n(r),a}}var l={};return l[n]=n=>{},l[e]=i,l})))(i);var a=()=>s.map((n=>{if(n[t])throw n[t];return n[e]})),c=new Promise((e=>{(r=()=>e(a)).r=0;var t=n=>n!==l&&!u.has(n)&&(u.add(n),n&&!n.d&&(r.r++,n.push(r)));s.map((e=>e[n](t)))}));return r.r?c:a()}),(n=>(n?d(m[t]=n):c(p),o(l)))),l&&l.d<0&&(l.d=0)},s.n=n=>{var e=n&&n.__esModule?()=>n.default:()=>n;return s.d(e,{a:e}),e},s.d=(n,e)=>{for(var t in e)s.o(e,t)&&!s.o(n,t)&&Object.defineProperty(n,t,{enumerable:!0,get:e[t]})},s.f={},s.e=n=>Promise.all(Object.keys(s.f).reduce(((e,t)=>(s.f[t](n,e),e)),[])),s.u=n=>n+".js",s.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(n){if("object"==typeof window)return window}}(),s.o=(n,e)=>Object.prototype.hasOwnProperty.call(n,e),i={},r="linguistic-flux-capacitor:",s.l=(n,e,t,o)=>{if(i[n])i[n].push(e);else{var a,l;if(void 0!==t)for(var c=document.getElementsByTagName("script"),d=0;d<c.length;d++){var u=c[d];if(u.getAttribute("src")==n||u.getAttribute("data-webpack")==r+t){a=u;break}}a||(l=!0,(a=document.createElement("script")).charset="utf-8",a.timeout=120,s.nc&&a.setAttribute("nonce",s.nc),a.setAttribute("data-webpack",r+t),a.src=n),i[n]=[e];var p=(e,t)=>{a.onerror=a.onload=null,clearTimeout(m);var o=i[n];if(delete i[n],a.parentNode&&a.parentNode.removeChild(a),o&&o.forEach((n=>n(t))),e)return e(t)},m=setTimeout(p.bind(null,void 0,{type:"timeout",target:a}),12e4);a.onerror=p.bind(null,a.onerror),a.onload=p.bind(null,a.onload),l&&document.head.appendChild(a)}},s.r=n=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(n,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(n,"__esModule",{value:!0})},s.v=(n,e,t,o)=>{var i=fetch(s.p+""+t+".module.wasm"),r=()=>i.then((n=>n.arrayBuffer())).then((n=>WebAssembly.instantiate(n,o))).then((e=>Object.assign(n,e.instance.exports)));return i.then((e=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(e,o).then((e=>Object.assign(n,e.instance.exports)),(n=>{if("application/wasm"!==e.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",n),r();throw n})):r()))},(()=>{var n;s.g.importScripts&&(n=s.g.location+"");var e=s.g.document;if(!n&&e&&(e.currentScript&&"SCRIPT"===e.currentScript.tagName.toUpperCase()&&(n=e.currentScript.src),!n)){var t=e.getElementsByTagName("script");if(t.length)for(var o=t.length-1;o>-1&&(!n||!/^http(s?):/.test(n));)n=t[o--].src}if(!n)throw new Error("Automatic publicPath is not supported in this browser");n=n.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),s.p=n})(),(()=>{s.b=document.baseURI||self.location.href;var n={792:0};s.f.j=(e,t)=>{var o=s.o(n,e)?n[e]:void 0;if(0!==o)if(o)t.push(o[2]);else{var i=new Promise(((t,i)=>o=n[e]=[t,i]));t.push(o[2]=i);var r=s.p+s.u(e),a=new Error;s.l(r,(t=>{if(s.o(n,e)&&(0!==(o=n[e])&&(n[e]=void 0),o)){var i=t&&("load"===t.type?"missing":t.type),r=t&&t.target&&t.target.src;a.message="Loading chunk "+e+" failed.\n("+i+": "+r+")",a.name="ChunkLoadError",a.type=i,a.request=r,o[1](a)}}),"chunk-"+e,e)}};var e=(e,t)=>{var o,i,[r,a,l]=t,c=0;if(r.some((e=>0!==n[e]))){for(o in a)s.o(a,o)&&(s.m[o]=a[o]);l&&l(s)}for(e&&e(t);c<r.length;c++)i=r[c],s.o(n,i)&&n[i]&&n[i][0](),n[i]=0},t=self.webpackChunklinguistic_flux_capacitor=self.webpackChunklinguistic_flux_capacitor||[];t.forEach(e.bind(null,0)),t.push=e.bind(null,t.push.bind(t))})(),s.nc=void 0;var c=s(72),d=s.n(c),u=s(825),p=s.n(u),m=s(659),f=s.n(m),h=s(56),g=s.n(h),b=s(540),y=s.n(b),v=s(113),w=s.n(v),x=s(365),C={};C.styleTagTransform=w(),C.setAttributes=g(),C.insert=f().bind(null,"head"),C.domAPI=p(),C.insertStyleElement=y(),d()(x.A,C),x.A&&x.A.locals&&x.A.locals;const E=s.p+"1eb7d266c109b7ee50d5.bin";let I=s.e(104).then(s.bind(s,104));!async function(){if("object"!=typeof WebAssembly||"function"!=typeof WebAssembly.instantiate)return;const n=await s.e(395).then(s.bind(s,395));"loading"===document.readyState&&await new Promise((function(n,e){window.addEventListener("DOMContentLoaded",n)}));let e=[],t=[];for(let n=1800;n<=2008;n+=1)e.push(n),n%20==0&&t.push(n);let o="",i=[],r=[],a=document.getElementById("mainLegend"),l=document.getElementById("suggestedComparisons").querySelectorAll("li"),c=document.getElementById("manualComparisons").querySelectorAll("li"),d=null,u=[],p=[],m=[...l,...c],f=document.querySelector(".inputWidthMeasure"),h=function(){let n=document.getElementById("tooltipTemplate"),t=n.querySelector(".tooltipContent"),o=n.querySelector(".year"),i=n.querySelector(".word1"),r=n.querySelector(".word2>a"),a=[],l=null,s=[{},{}],c=[0,0],d=0;return n.querySelectorAll(".suggestion.left>a").forEach((n=>{a.push(n),n.addEventListener("click",(e=>{e.preventDefault(),n.blur(),A(n.innerText,null)}))})),n.querySelectorAll(".suggestion.right>a").forEach((n=>{a.push(n),n.addEventListener("click",(e=>{e.preventDefault(),n.blur(),A(n.innerText,null)}))})),r.addEventListener("click",(n=>{n.preventDefault(),r.blur(),A(r.innerText,null)})),function(n,u,p){clearTimeout(l);let m=u.payload;o.innerText=e[p],i.innerText=m.word1,r.innerText=m.word2;let f=m.word1Id+"-"+m.word2Id+"-"+p,h=s[d][f],g=h||s[1-d][f];void 0!==g?(g.forEach(((n,e)=>{a[e].innerText=y.vocab[n]})),t.classList.remove("waiting"),void 0===h&&(1024===c[d]&&(d=1-d,s[d]={},c[d]=0),s[d][f]=g,c[d]+=1)):(t.classList.add("waiting"),l=setTimeout((()=>{let n=b.most_related_to_at_t([m.word1Id,m.word2Id],p,7);n.forEach(((n,e)=>{a[e].innerText=y.vocab[n]})),t.classList.remove("waiting"),1024==c[d]&&(d=1-d,s[d]={},c[d]=0),s[d][f]=n,c[d]+=1}),0))}}();const g=n.createPlot(document.getElementById("mainPlot"),e,t,h,document.getElementById("tooltipTemplate"),(function(n){m[n].classList.add("hovering")}),(function(n){m[n].classList.remove("hovering")}));document.getElementById("mainLegend").querySelectorAll("ul").forEach((n=>n.addEventListener("mouseout",(()=>g.lineToFront())))),m.forEach(((n,e)=>{n.addEventListener("mouseover",(()=>{g.lineToFront(e),g.hoverLine(e)})),n.addEventListener("mouseout",(()=>g.unhoverLine(e))),n.addEventListener("click",(()=>g.setMainLine(e)));const t=n.querySelector("a");t&&t.addEventListener("click",(n=>{n.preventDefault(),t.blur(),A(t.innerText,null)}));const o=n.querySelectorAll("input");if(0!==o.length){const[e,t]=o;let r=u.length;u.push(e),p.push(t);let a=n=>function(n,e,t){let o=()=>{let o=e.value.trim(),r=[...i];t>=r.length-1&&""===o?r.splice(t,1):t<r.length?r[t]=o:r.push(o),A(null,r),"blur"!==n.type&&"change"!==n.type&&g.setMainLine(l.length+t)};setTimeout(o,0),setTimeout(o,300)}(n,e,r);e.onkeydown=a,e.onchange=a,e.onclick=a,e.onblur=a,t.onclick=()=>function(n){let e=[...i];n<e.length&&(e.splice(n,1),A(null,e))}(r),0===r?(e.style.width="0",t.style.display="none"):n.style.display="none"}}));let[b,y]=await Promise.all([I.then((n=>n.loadFile())),fetch(E).then((n=>n.json()))]);document.getElementById("downloadProgressPane").style.display="none",document.querySelector(".app").style.display="block";let v={};y.vocab.forEach(((n,e)=>v[n]=e));let w=document.querySelector(".wordInput"),x=document.querySelector(".wordInputError");function C(){let n="https://robamler.github.io/linguistic-flux-capacitor";return""!==o&&(n+=location.hash),[n,"Explore how the meaning of "+(""===o?"words":'the word "'+o+'"')+" has changed over the past two centuries"]}function k(n){n.preventDefault();let[e,t]=C();window.open("https://twitter.com/intent/tweet?text="+encodeURIComponent(t+": "+e),"share-dialog","width=626,height=436")}function S(){let n="",e=[];for(let t of window.location.hash.substr(1).split("&")){let[o,i]=t.split("=");"w"===o?n=decodeURIComponent(i):"o"===o&&""!==i&&(e=i.split("+").map(decodeURIComponent))}A(n,e,!0)}function T(){let n=()=>A(w.value.trim(),null);setTimeout(n,0),setTimeout(n,300)}function A(n,e,t=!1){let s=!1,h=!1;if(null!==n){w.value.trim()!==n&&(w.value=n);let e=v[n];""===n||void 0!==e?(w.classList.remove("invalid"),x.style.display="none",n!==o&&(s=!0,o=n,d=b.largest_changes_wrt(e,l.length,2,2))):(w.classList.add("invalid"),x.style.display="inline-block")}if(null!==e){let n=[];e.length>c.length&&e.splice(c.length);for(let t=0;t<e.length;t+=1){let o=e[t],r=v[o];n.push(r),(t>=i.length||i[t]!==o)&&(h=!0,void 0===r?(u[t].classList.add("invalid"),u[t].setAttribute("title","word not found"),u[t].parentElement.removeAttribute("title"),u[t].parentElement.classList.add("inactive")):(u[t].classList.remove("invalid"),u[t].removeAttribute("title"),u[t].parentElement.setAttribute("title","Click and move mouse across diagram to explore further."),u[t].parentElement.classList.remove("inactive")),c[t].style.display="list-item",p[t].style.display="inline",u[t].value.trim()!==o&&(u[t].value=o),f.textContent=o,u[t].style.width=f.offsetWidth+"px")}if(e.length!==i.length&&(h=!0,e.length<c.length)){c[e.length].style.display="list-item",u[e.length].value="",u[e.length].style.width="0",u[e.length].classList.remove("invalid"),u[e.length].setAttribute("title","Enter a secondary word here."),u[e.length].parentElement.classList.add("inactive"),u[e.length].parentElement.removeAttribute("title"),p[e.length].style.display="none";for(let n=e.length+1;n<c.length;n+=1)c[n].style.display="none"}i=e,r=n}if(s||h){if(g.clear(),""===o)return document.title="The Linguistic Flux Capacitor",g.showInputPrompt(),a.classList.add("empty"),void(t||history.pushState(null,"The Linguistic Flux Capacitor","#"));if(document.title="The Linguistic Flux Capacitor: "+o,!t){let n="#v=0&c=en&w="+encodeURIComponent(o);0!=i.length&&(n=n+"&o="+i.map(encodeURIComponent).join("+")),history.pushState(null,"The Linguistic Flux Capacitor: "+o,n)}a.classList.remove("empty"),m.forEach((n=>{n.classList.remove("hovering"),n.firstElementChild.textContent=o}));let n=[...d],e=[];for(let t=0;t<n.length;t+=1)e.push(t);r.forEach(((t,o)=>{void 0!==t&&(n.push(t),e.push(d.length+o))}));let s=v[o],c=Array(n.length).fill(s),u=b.pairwise_trajectories(c,n),p=u.length/n.length;n.forEach(((n,t)=>{let i=y.vocab[n];g.plotLine(u.subarray(t*p,(t+1)*p),e[t],0,{word1:o,word2:i,word1Id:s,word2Id:n},!1,'"'+o+'" ↔ "'+i+'"\n(click on line to explore relationship)'),t<l.length&&(m[t].firstElementChild.nextElementSibling.textContent=i)}))}}w.onkeydown=T,w.onchange=T,w.onclick=T,w.onblur=T,document.getElementById("shareFacebookButton").onclick=function(n){n.preventDefault();let[e,t]=C(),o="https://www.facebook.com/share.php?u="+encodeURIComponent(e)+"&quote="+encodeURIComponent(t+" using this web app.");window.open(o,"share-dialog","width=626,height=436")},document.getElementById("shareTwitterButton").onclick=k,document.getElementById("copyLinkButton").onclick=async function(n){n.preventDefault();let[e,t]=C();await navigator.clipboard.writeText(t+": "+e);let o=document.querySelector(".toast");o.style.display="inline-block",o.style.opacity=1,setTimeout((()=>o.style.opacity=0),3e3),setTimeout((()=>o.style.display="none"),3900)},document.getElementById("shareTwitterButton").onclick=k,window.addEventListener("popstate",S),setTimeout((()=>{S(),""===o&&g.showInputPrompt(),w.selectionStart=w.selectionEnd=w.value.length,w.focus()}),0)}()})();