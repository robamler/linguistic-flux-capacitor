/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "./node_modules/css-loader/dist/cjs.js!./src/styles.css":
/*!**************************************************************!*\
  !*** ./node_modules/css-loader/dist/cjs.js!./src/styles.css ***!
  \**************************************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": () => (__WEBPACK_DEFAULT_EXPORT__)\n/* harmony export */ });\n/* harmony import */ var _node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../node_modules/css-loader/dist/runtime/api.js */ \"./node_modules/css-loader/dist/runtime/api.js\");\n/* harmony import */ var _node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_0__);\n// Imports\n\nvar ___CSS_LOADER_EXPORT___ = _node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_0___default()(function(i){return i[1]});\n// Module\n___CSS_LOADER_EXPORT___.push([module.id, \"body {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 20px;\\n}\\n\\n.pageContainer {\\n    width: 50em;\\n    position: absolute;\\n    left: 50%;\\n    transform: translateX(-50%);\\n}\\n\\nh1 {\\n    text-align: center;\\n    font-weight: normal;\\n    font-size: 280%;\\n    font-family: 'DM Serif Display', serif;\\n    margin: 0.3em 0 0.7em 0;\\n    padding: 0;\\n    line-height: 1.1em;\\n}\\n\\n.app>h2 {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-weight: normal;\\n    text-align: center;\\n    font-size: 160%;\\n    margin: 0.2em 0;\\n}\\n\\n.legendItems>h3 {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 110%;\\n    font-weight: bold;\\n    text-align: center;\\n    margin: 0 0 0.3em 0;\\n    display: block;\\n    white-space: nowrap;\\n}\\n\\nh2 {\\n    font-size: 230%;\\n    font-weight: normal;\\n    font-family: 'DM Serif Display', serif;\\n    margin: 1.4em 0 0.6em 0;\\n    line-height: 100%;\\n}\\n\\nh3 {\\n    font-family: 'DM Serif Display', serif;\\n    font-size: 125%;\\n    font-weight: 700;\\n    margin: 1em 0.4em -0.3em 0;\\n    padding: 0;\\n    display: inline;\\n}\\n\\n.prose {\\n    color: #333;\\n    line-height: 150%;\\n}\\n\\n.prose li {\\n    margin-top: 0.3em;\\n}\\n\\nfooter {\\n    opacity: 0.7;\\n    font-size: 80%;\\n    margin: 2em 0 1em 0;\\n}\\n\\n.subtitle {\\n    font-size: 60%;\\n    display: block;\\n}\\n\\n.splashScreen {\\n    text-align: center;\\n    font-size: 120%;\\n    margin: 4em 2em 7em 2em;\\n}\\n\\n#wasmErrorMessage {\\n    text-align: center;\\n}\\n\\n#wasmErrorMessage>table {\\n    border: 0;\\n    width: 100%;\\n}\\n\\n#wasmErrorMessage td {\\n    width: 20%;\\n    text-align: center;\\n    vertical-align: top;\\n}\\n\\n.appContainer {\\n    width: 40em;\\n}\\n\\n.app {\\n    display: none;\\n}\\n\\n#downloadProgressPane {\\n    color: #0d441d;\\n}\\n\\n.progressBarContainer {\\n    margin: 0.7em;\\n    border: #36864e solid 0.1em;\\n    border-radius: 0.3em;\\n    position: relative;\\n    height: 1.5em;\\n}\\n\\n.progressBar {\\n    background: #bfdfc9;\\n    position: absolute;\\n    left: 0;\\n    top: 0;\\n    bottom: 0;\\n    width: 0;\\n    z-index: 0;\\n    border-radius: 0.15em;\\n}\\n\\n.progressText {\\n    position: absolute;\\n    margin: auto;\\n    left: 0;\\n    right: 0;\\n    top: 0.1em;\\n    text-shadow: 0 0 0.2em #fff;\\n}\\n\\n.plotAndLegend {\\n    position: relative;\\n    margin: 1.3em -2% 0 -2%;\\n}\\n\\n.plotContainer {\\n    position: relative;\\n    left: -2em;\\n    width: 41em;\\n}\\n\\n.centered {\\n    text-align: center;\\n}\\n\\n.wordInput {\\n    margin: 0 0 0.1em 0;\\n    font-size: 160%;\\n    width: 40%;\\n    text-align: center;\\n}\\n\\ninput {\\n    border: solid 0.05em #ccc;\\n    border-radius: 0.2em;\\n    padding: 0.2em;\\n}\\n\\ninput:focus {\\n    border-color: #86bdf0;\\n    background: #fafbfd;\\n    outline: none;\\n}\\n\\ninput.invalid {\\n    background-color: #fdebeb;\\n}\\n\\ninput.invalid:focus {\\n    border-color: #9c5555;\\n}\\n\\n.tooltipContent .explanation {\\n    font-size: 90%;\\n}\\n\\n.tooltipContent th {\\n    font-size: 120%;\\n}\\n\\n.tooltipContent th, .tooltipContent td {\\n    vertical-align: baseline;\\n}\\n\\n.tooltipContent th.left, .tooltipContent td.left {\\n    padding-right: 0.3em;\\n}\\n\\n.tooltipContent th.right, .tooltipContent td.right {\\n    padding-left: 0.3em;\\n}\\n\\n.tooltipContent .year {\\n    font-size: 140%;\\n    font-weight: bold;\\n}\\n\\n.tooltipContent .interleave {\\n    padding: 0.2em 0 0.5em 0;\\n}\\n\\n.hint {\\n    opacity: 0.7;\\n    font-size: 80%;\\n    margin-top: 0.1em;\\n}\\n\\n.shareContainer {\\n    margin: -0.3em 0.3em 0 0;\\n    white-space: nowrap;\\n    color: #555;\\n    font-size: 80%;\\n}\\n\\n.shareContainer::before {\\n    display: inline-block;\\n    vertical-align: top;\\n    padding-top: 0.1em;\\n    content: 'Share current plot on:';\\n}\\n\\n.legend.empty>.shareContainer::before {\\n    content: 'Share this app on:';\\n    display: block;\\n}\\n\\n.shareContainer>a {\\n    display: inline-block;\\n    vertical-align: top;\\n    margin-left: 0.2em;\\n}\\n\\n.legend.empty>.shareContainer>a {\\n    margin: 0.4em 0.4em 0.2em 0.4em;\\n}\\n\\n.shareContainer>a>img {\\n    width: 1.5em;\\n    height: 1.5em;\\n    opacity: 0.6;\\n    transition: opacity 0.1s;\\n}\\n\\n.shareContainer>a:hover>img {\\n    opacity: 1;\\n}\\n\\n.toast {\\n    position: relative;\\n    left: -0.9em;\\n    display: none;\\n    opacity: 0;\\n    transition: opacity 0.8s;\\n}\\n\\n.toast>span {\\n    display: block;\\n    position: absolute;\\n    top: 1.1em;\\n    transform: translateX(-50%);\\n    white-space: nowrap;\\n    background: #cddbc5;\\n    border: 0.2em solid #fff;\\n    border-radius: 0.4em;\\n    padding: 0.3em 0.5em;\\n    font-size: 110%;\\n    color: #1b3e14;\\n    text-align: center;\\n}\\n\\n.wordInputError {\\n    position: absolute;\\n    margin: 0.2em 0 0 0.6em;\\n    color: #821313;\\n    display: none;\\n    line-height: 1.05em;\\n    text-align: left;\\n}\\n\\n.tooltipContent table {\\n    width: 100%;\\n}\\n\\n.tooltipContent .wait {\\n    display: none;\\n    position: absolute;\\n    text-align: center;\\n    width: 10em;\\n    white-space: normal;\\n    left: 50%;\\n    top: 50%;\\n    transform: translate(-50%, -50%);\\n    opacity: 0.8;\\n    font-size: 80%;\\n}\\n\\n.tooltipContent.waiting .wait {\\n    display: block;\\n}\\n\\n.tooltipContent .suggestionsTable {\\n    visibility: visible;\\n}\\n\\n.tooltipContent.waiting .suggestionsTable {\\n    visibility: hidden;\\n}\\n\\n.tooltipContent .suggestion {\\n    padding: 0.05em 0;\\n    font-size: 90%;\\n}\\n\\n.tooltipContent .left {\\n    text-align: left;\\n}\\n\\n.tooltipContent .right {\\n    text-align: right;\\n}\\n\\n.tooltipContent .removeWordButton {\\n    background-color: #BCB8AE;\\n    /*Mindful Gray*/\\n    color: white;\\n    font-size: 80%;\\n}\\n\\na {\\n    text-decoration: none;\\n    color: #1135a0;\\n}\\n\\n.legend {\\n    background-color: #e8e8e8;\\n    border-radius: 0.5em;\\n    padding: 0.6em 0.7em 0.2em 0.6em;\\n    width: auto;\\n    position: absolute;\\n    left: 40em;\\n    top: 0;\\n}\\n\\n.legend.empty {\\n    text-align: center;\\n}\\n\\n.legend.empty>.legendItems {\\n    display: none;\\n}\\n\\n.legendItems>ul {\\n    list-style-type: none;\\n    margin: 0 0 0.5em 0;\\n    padding: 0;\\n}\\n\\n.legendItems>ul>li {\\n    margin: -0.2em -0.4em 0.0em -0.4em;\\n    padding: 0.2em 0.4em 0.3em 0.4em;\\n    border-radius: 0.2em;\\n    white-space: nowrap;\\n    cursor: default;\\n}\\n\\n.legendItems>ul>li:hover, .legendItems>ul>li.hovering {\\n    background-color: #f8f8f8;\\n}\\n\\n.legendItems>ul>li>a {\\n    text-decoration: none;\\n    color: #152d74;\\n}\\n\\n.legendItems>ul>li::before {\\n    content: \\\"———\\\";\\n    font-family: monospace;\\n    margin-right: 0.1em;\\n    font-weight: bold;\\n}\\n\\n.legendItems>ul>li.inactive::before {\\n    visibility: hidden;\\n}\\n\\n#manualComparisons>li::before {\\n    content: \\\"— —\\\";\\n}\\n\\n.legendItems>ul>li.color0::before {\\n    color: #f94a01;\\n}\\n\\n.legendItems>ul>li.color1::before {\\n    color: #6b42b6;\\n}\\n\\n.legendItems>ul>li.color2::before {\\n    color: #11a854;\\n}\\n\\n.legendItems>ul>li.color3::before {\\n    color: #128db2;\\n}\\n\\n.legendItems>ul>li.color4::before {\\n    color: #e12fbc;\\n}\\n\\n.legendItems>ul>li.color5::before {\\n    color: #e6ab02;\\n}\\n\\n.legendItems>ul>li.color6::before {\\n    color: #b40e0e;\\n}\\n\\n.legendItems>ul>li.color7::before {\\n    color: #4257b6;\\n}\\n\\n.legendItems>ul>li.color8::before {\\n    color: #18a380;\\n}\\n\\n.legendItems>ul>li.color9::before {\\n    color: #ad27c5;\\n}\\n\\n.legendItems>ul>li.color10::before {\\n    color: #2e2e2e;\\n}\\n\\n.legendItems input[type=\\\"text\\\"] {\\n    font-size: 80%;\\n    min-width: 5.2em;\\n}\\n\\n.legendItems input[type=\\\"button\\\"] {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 90%;\\n    width: 1.4em;\\n    height: 1.4em;\\n    line-height: 0;\\n}\\n\\n.inputWidthMeasure {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 90%;\\n    white-space: nowrap;\\n    position: absolute;\\n    opacity: 0;\\n}\\n\\n.pinWordButton {\\n    background-color: #BCB8AE;\\n    /*Mindful Gray*/\\n    border: none;\\n    color: white;\\n    padding: 10px 32px;\\n    text-align: center;\\n    text-decoration: none;\\n    display: inline-block;\\n    font-size: 80%;\\n}\\n\\n.getUrlButton {\\n    background-color: #BCB8AE;\\n    /*Mindful Gray*/\\n    border: none;\\n    color: white;\\n    padding: 5px 5px;\\n    text-align: center;\\n    text-decoration: none;\\n    display: inline-block;\\n    font-size: 80%;\\n}\\n\\n.github-corner>svg {\\n    fill: #666;\\n    color: #fff;\\n    position: absolute;\\n    top: 0;\\n    border: 0;\\n    right: 0;\\n    width: 3.5em;\\n    height: 3.5em;\\n}\\n\\n.github-corner:hover .octo-arm {\\n    animation: octocat-wave 560ms ease-in-out;\\n}\\n\\n@keyframes octocat-wave {\\n    0%, 100% {\\n        transform: rotate(0);\\n    }\\n    20%, 60% {\\n        transform: rotate(-25deg);\\n    }\\n    40%, 80% {\\n        transform: rotate(10deg);\\n    }\\n}\\n\\n@media (max-width: 70em) {\\n    body {\\n        font-size: 16px;\\n    }\\n    .pageContainer {\\n        width: 95%;\\n    }\\n    .appContainer {\\n        width: 100%;\\n    }\\n    .plotContainer {\\n        left: 0;\\n        width: 68%;\\n    }\\n    .legend {\\n        left: 70%;\\n        font-size: 75%;\\n    }\\n    .github-corner>svg {\\n        width: 2.5em;\\n        height: 2.5em;\\n    }\\n}\\n\", \"\"]);\n// Exports\n/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (___CSS_LOADER_EXPORT___);\n\n\n//# sourceURL=webpack://linguistic-flux-capacitor/./src/styles.css?./node_modules/css-loader/dist/cjs.js");

/***/ }),

/***/ "./node_modules/css-loader/dist/runtime/api.js":
/*!*****************************************************!*\
  !*** ./node_modules/css-loader/dist/runtime/api.js ***!
  \*****************************************************/
/***/ ((module) => {

"use strict";
eval("\n\n/*\n  MIT License http://www.opensource.org/licenses/mit-license.php\n  Author Tobias Koppers @sokra\n*/\n// css base code, injected by the css-loader\n// eslint-disable-next-line func-names\nmodule.exports = function (cssWithMappingToString) {\n  var list = []; // return the list of modules as css string\n\n  list.toString = function toString() {\n    return this.map(function (item) {\n      var content = cssWithMappingToString(item);\n\n      if (item[2]) {\n        return \"@media \".concat(item[2], \" {\").concat(content, \"}\");\n      }\n\n      return content;\n    }).join(\"\");\n  }; // import a list of modules into the list\n  // eslint-disable-next-line func-names\n\n\n  list.i = function (modules, mediaQuery, dedupe) {\n    if (typeof modules === \"string\") {\n      // eslint-disable-next-line no-param-reassign\n      modules = [[null, modules, \"\"]];\n    }\n\n    var alreadyImportedModules = {};\n\n    if (dedupe) {\n      for (var i = 0; i < this.length; i++) {\n        // eslint-disable-next-line prefer-destructuring\n        var id = this[i][0];\n\n        if (id != null) {\n          alreadyImportedModules[id] = true;\n        }\n      }\n    }\n\n    for (var _i = 0; _i < modules.length; _i++) {\n      var item = [].concat(modules[_i]);\n\n      if (dedupe && alreadyImportedModules[item[0]]) {\n        // eslint-disable-next-line no-continue\n        continue;\n      }\n\n      if (mediaQuery) {\n        if (!item[2]) {\n          item[2] = mediaQuery;\n        } else {\n          item[2] = \"\".concat(mediaQuery, \" and \").concat(item[2]);\n        }\n      }\n\n      list.push(item);\n    }\n  };\n\n  return list;\n};\n\n//# sourceURL=webpack://linguistic-flux-capacitor/./node_modules/css-loader/dist/runtime/api.js?");

/***/ }),

/***/ "./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin":
/*!*******************************************************************!*\
  !*** ./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin ***!
  \*******************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": () => (__WEBPACK_DEFAULT_EXPORT__)\n/* harmony export */ });\n/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (__webpack_require__.p + \"e5d63ffe4b441ec1b9ff0f187ed189b7.bin\");\n\n//# sourceURL=webpack://linguistic-flux-capacitor/./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin?");

/***/ }),

/***/ "./src/styles.css":
/*!************************!*\
  !*** ./src/styles.css ***!
  \************************/
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

eval("var api = __webpack_require__(/*! !../node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js */ \"./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js\");\n            var content = __webpack_require__(/*! !!../node_modules/css-loader/dist/cjs.js!./styles.css */ \"./node_modules/css-loader/dist/cjs.js!./src/styles.css\");\n\n            content = content.__esModule ? content.default : content;\n\n            if (typeof content === 'string') {\n              content = [[module.id, content, '']];\n            }\n\nvar options = {};\n\noptions.insert = \"head\";\noptions.singleton = false;\n\nvar update = api(content, options);\n\n\n\nmodule.exports = content.locals || {};\n\n//# sourceURL=webpack://linguistic-flux-capacitor/./src/styles.css?");

/***/ }),

/***/ "./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js":
/*!****************************************************************************!*\
  !*** ./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js ***!
  \****************************************************************************/
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

"use strict";
eval("\n\nvar isOldIE = function isOldIE() {\n  var memo;\n  return function memorize() {\n    if (typeof memo === 'undefined') {\n      // Test for IE <= 9 as proposed by Browserhacks\n      // @see http://browserhacks.com/#hack-e71d8692f65334173fee715c222cb805\n      // Tests for existence of standard globals is to allow style-loader\n      // to operate correctly into non-standard environments\n      // @see https://github.com/webpack-contrib/style-loader/issues/177\n      memo = Boolean(window && document && document.all && !window.atob);\n    }\n\n    return memo;\n  };\n}();\n\nvar getTarget = function getTarget() {\n  var memo = {};\n  return function memorize(target) {\n    if (typeof memo[target] === 'undefined') {\n      var styleTarget = document.querySelector(target); // Special case to return head of iframe instead of iframe itself\n\n      if (window.HTMLIFrameElement && styleTarget instanceof window.HTMLIFrameElement) {\n        try {\n          // This will throw an exception if access to iframe is blocked\n          // due to cross-origin restrictions\n          styleTarget = styleTarget.contentDocument.head;\n        } catch (e) {\n          // istanbul ignore next\n          styleTarget = null;\n        }\n      }\n\n      memo[target] = styleTarget;\n    }\n\n    return memo[target];\n  };\n}();\n\nvar stylesInDom = [];\n\nfunction getIndexByIdentifier(identifier) {\n  var result = -1;\n\n  for (var i = 0; i < stylesInDom.length; i++) {\n    if (stylesInDom[i].identifier === identifier) {\n      result = i;\n      break;\n    }\n  }\n\n  return result;\n}\n\nfunction modulesToDom(list, options) {\n  var idCountMap = {};\n  var identifiers = [];\n\n  for (var i = 0; i < list.length; i++) {\n    var item = list[i];\n    var id = options.base ? item[0] + options.base : item[0];\n    var count = idCountMap[id] || 0;\n    var identifier = \"\".concat(id, \" \").concat(count);\n    idCountMap[id] = count + 1;\n    var index = getIndexByIdentifier(identifier);\n    var obj = {\n      css: item[1],\n      media: item[2],\n      sourceMap: item[3]\n    };\n\n    if (index !== -1) {\n      stylesInDom[index].references++;\n      stylesInDom[index].updater(obj);\n    } else {\n      stylesInDom.push({\n        identifier: identifier,\n        updater: addStyle(obj, options),\n        references: 1\n      });\n    }\n\n    identifiers.push(identifier);\n  }\n\n  return identifiers;\n}\n\nfunction insertStyleElement(options) {\n  var style = document.createElement('style');\n  var attributes = options.attributes || {};\n\n  if (typeof attributes.nonce === 'undefined') {\n    var nonce =  true ? __webpack_require__.nc : 0;\n\n    if (nonce) {\n      attributes.nonce = nonce;\n    }\n  }\n\n  Object.keys(attributes).forEach(function (key) {\n    style.setAttribute(key, attributes[key]);\n  });\n\n  if (typeof options.insert === 'function') {\n    options.insert(style);\n  } else {\n    var target = getTarget(options.insert || 'head');\n\n    if (!target) {\n      throw new Error(\"Couldn't find a style target. This probably means that the value for the 'insert' parameter is invalid.\");\n    }\n\n    target.appendChild(style);\n  }\n\n  return style;\n}\n\nfunction removeStyleElement(style) {\n  // istanbul ignore if\n  if (style.parentNode === null) {\n    return false;\n  }\n\n  style.parentNode.removeChild(style);\n}\n/* istanbul ignore next  */\n\n\nvar replaceText = function replaceText() {\n  var textStore = [];\n  return function replace(index, replacement) {\n    textStore[index] = replacement;\n    return textStore.filter(Boolean).join('\\n');\n  };\n}();\n\nfunction applyToSingletonTag(style, index, remove, obj) {\n  var css = remove ? '' : obj.media ? \"@media \".concat(obj.media, \" {\").concat(obj.css, \"}\") : obj.css; // For old IE\n\n  /* istanbul ignore if  */\n\n  if (style.styleSheet) {\n    style.styleSheet.cssText = replaceText(index, css);\n  } else {\n    var cssNode = document.createTextNode(css);\n    var childNodes = style.childNodes;\n\n    if (childNodes[index]) {\n      style.removeChild(childNodes[index]);\n    }\n\n    if (childNodes.length) {\n      style.insertBefore(cssNode, childNodes[index]);\n    } else {\n      style.appendChild(cssNode);\n    }\n  }\n}\n\nfunction applyToTag(style, options, obj) {\n  var css = obj.css;\n  var media = obj.media;\n  var sourceMap = obj.sourceMap;\n\n  if (media) {\n    style.setAttribute('media', media);\n  } else {\n    style.removeAttribute('media');\n  }\n\n  if (sourceMap && typeof btoa !== 'undefined') {\n    css += \"\\n/*# sourceMappingURL=data:application/json;base64,\".concat(btoa(unescape(encodeURIComponent(JSON.stringify(sourceMap)))), \" */\");\n  } // For old IE\n\n  /* istanbul ignore if  */\n\n\n  if (style.styleSheet) {\n    style.styleSheet.cssText = css;\n  } else {\n    while (style.firstChild) {\n      style.removeChild(style.firstChild);\n    }\n\n    style.appendChild(document.createTextNode(css));\n  }\n}\n\nvar singleton = null;\nvar singletonCounter = 0;\n\nfunction addStyle(obj, options) {\n  var style;\n  var update;\n  var remove;\n\n  if (options.singleton) {\n    var styleIndex = singletonCounter++;\n    style = singleton || (singleton = insertStyleElement(options));\n    update = applyToSingletonTag.bind(null, style, styleIndex, false);\n    remove = applyToSingletonTag.bind(null, style, styleIndex, true);\n  } else {\n    style = insertStyleElement(options);\n    update = applyToTag.bind(null, style, options);\n\n    remove = function remove() {\n      removeStyleElement(style);\n    };\n  }\n\n  update(obj);\n  return function updateStyle(newObj) {\n    if (newObj) {\n      if (newObj.css === obj.css && newObj.media === obj.media && newObj.sourceMap === obj.sourceMap) {\n        return;\n      }\n\n      update(obj = newObj);\n    } else {\n      remove();\n    }\n  };\n}\n\nmodule.exports = function (list, options) {\n  options = options || {}; // Force single-tag solution on IE6-9, which has a hard limit on the # of <style>\n  // tags it will allow on a page\n\n  if (!options.singleton && typeof options.singleton !== 'boolean') {\n    options.singleton = isOldIE();\n  }\n\n  list = list || [];\n  var lastIdentifiers = modulesToDom(list, options);\n  return function update(newList) {\n    newList = newList || [];\n\n    if (Object.prototype.toString.call(newList) !== '[object Array]') {\n      return;\n    }\n\n    for (var i = 0; i < lastIdentifiers.length; i++) {\n      var identifier = lastIdentifiers[i];\n      var index = getIndexByIdentifier(identifier);\n      stylesInDom[index].references--;\n    }\n\n    var newLastIdentifiers = modulesToDom(newList, options);\n\n    for (var _i = 0; _i < lastIdentifiers.length; _i++) {\n      var _identifier = lastIdentifiers[_i];\n\n      var _index = getIndexByIdentifier(_identifier);\n\n      if (stylesInDom[_index].references === 0) {\n        stylesInDom[_index].updater();\n\n        stylesInDom.splice(_index, 1);\n      }\n    }\n\n    lastIdentifiers = newLastIdentifiers;\n  };\n};\n\n//# sourceURL=webpack://linguistic-flux-capacitor/./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js?");

/***/ }),

/***/ "./src/index.js":
/*!**********************!*\
  !*** ./src/index.js ***!
  \**********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _styles_css__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./styles.css */ \"./src/styles.css\");\n/* harmony import */ var _styles_css__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_styles_css__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var _assets_googlebooks_metadata_1800to2008_vocabsize30000_bin__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../assets/googlebooks_metadata_1800to2008_vocabsize30000.bin */ \"./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin\");\n\n\n\n//import FaceBookIcon from './facebook-icon.png'\n//import TwitterIcon from './facebook-icon.png'\n\n// Wasm modules must be imported asynchronously.\nlet backendPromise = __webpack_require__.e(/*! import() */ \"src_backend_js\").then(__webpack_require__.bind(__webpack_require__, /*! ./backend.js */ \"./src/backend.js\"));\n\n(async function () {\n    if (typeof WebAssembly !== \"object\" || typeof WebAssembly.instantiate !== \"function\") {\n        // Error message will be unveiled from main page because it's unclear whether\n        // ancient browsers that don't support WebAssembly can even parse this JS file.\n        return;\n    }\n\n    const Plotter = await __webpack_require__.e(/*! import() */ \"src_plotting_main_mjs\").then(__webpack_require__.bind(__webpack_require__, /*! ./plotting/main.mjs */ \"./src/plotting/main.mjs\"));\n    if (document.readyState === 'loading') {\n        await new Promise(function (resolve, _reject) {\n            window.addEventListener('DOMContentLoaded', resolve);\n        });\n    }\n\n    let years = [];\n    let ticksX = [];\n    for (let year = 1800; year <= 2008; year += 1) {\n        years.push(year);\n        if (year % 20 === 0) {\n            ticksX.push(year);\n        }\n    }\n\n    let currentWord = ''; // Invariant: `currentWord` is always either '' or a valid word from the vocabulary.\n    let manualComparisons = [];\n    let manualComparisonIds = [];\n\n    let legend = document.getElementById('mainLegend');\n    let suggestedComparisonItems = document.getElementById('suggestedComparisons').querySelectorAll('li');\n    let manualComparisonItems = document.getElementById('manualComparisons').querySelectorAll('li');\n    let suggestedComparisonIds = null;\n    let manualComparisonInputs = [];\n    let manualComparisonRemoveButtons = [];\n    let allComparisonItems = [...suggestedComparisonItems, ...manualComparisonItems];\n\n    let inputWidthMeasure = document.querySelector('.inputWidthMeasure');\n\n    let updateTooltip = (function () {\n        let tooltip = document.getElementById('tooltipTemplate');\n        let tooltipContent = tooltip.querySelector('.tooltipContent');\n        let yearPlaceholder = tooltip.querySelector('.year');\n        let word1Placeholder = tooltip.querySelector('.word1');\n        let word2Placeholder = tooltip.querySelector('.word2>a');\n        let relatedPlaceholders = [];\n        let relatedRemoveButtons = [];\n        let relatedTimeout = null;\n        let relatedCache = [{}, {}];\n        let relatedCacheFilling = [0, 0];\n        let relatedCacheGeneration = 0;\n        const MAX_CACHE_FILLING = 1024;\n\n        tooltip.querySelectorAll('.suggestion.left>a').forEach(el => {\n            relatedPlaceholders.push(el);\n            el.addEventListener('click', ev => {\n                ev.preventDefault();\n                el.blur();\n                updatePlot(el.innerText, null);\n            });\n        });\n        tooltip.querySelectorAll('.suggestion.right>a').forEach(el => {\n            relatedPlaceholders.push(el);\n            el.addEventListener('click', ev => {\n                ev.preventDefault();\n                el.blur();\n                updatePlot(el.innerText, null);\n            });\n        });\n        word2Placeholder.addEventListener('click', ev => {\n            ev.preventDefault();\n            word2Placeholder.blur();\n            updatePlot(word2Placeholder.innerText, null);\n        });\n\n        return function (tooltip, line, indexX) {\n            clearTimeout(relatedTimeout);\n            let payload = line.payload;\n            yearPlaceholder.innerText = years[indexX];\n            word1Placeholder.innerText = payload.word1;\n            word2Placeholder.innerText = payload.word2;\n\n            // TODO: look up word1 and word2 in cache independently.\n            let cacheKey = payload.word1Id + '-' + payload.word2Id + '-' + indexX;\n            let cachedCurrent = relatedCache[relatedCacheGeneration][cacheKey];\n            let cached = cachedCurrent || relatedCache[1 - relatedCacheGeneration][cacheKey];\n            if (typeof cached !== 'undefined') {\n                cached.forEach((r, i) => {\n                    relatedPlaceholders[i].innerText = metaData.vocab[r];\n                });\n                tooltipContent.classList.remove('waiting');\n\n                if (typeof cachedCurrent === 'undefined') {\n                    // Entry was found in old generation of the cache. Add it also to the current\n                    // generation so that it continues to stay cached for a while. If this would\n                    // overflow the current generation of the cache then flip generation instead.\n                    if (relatedCacheFilling[relatedCacheGeneration] === MAX_CACHE_FILLING) {\n                        relatedCacheGeneration = 1 - relatedCacheGeneration;\n                        relatedCache[relatedCacheGeneration] = {};\n                        relatedCacheFilling[relatedCacheGeneration] = 0;\n                    }\n                    relatedCache[relatedCacheGeneration][cacheKey] = cached;\n                    relatedCacheFilling[relatedCacheGeneration] += 1;\n                }\n            } else {\n                tooltipContent.classList.add('waiting');\n                relatedTimeout = setTimeout(() => {\n                    let related = handle.most_related_to_at_t([payload.word1Id, payload.word2Id], indexX, 7);\n                    related.forEach((r, i) => {\n                        relatedPlaceholders[i].innerText = metaData.vocab[r];\n                    });\n                    tooltipContent.classList.remove('waiting');\n\n                    if (relatedCacheFilling[relatedCacheGeneration] == MAX_CACHE_FILLING) {\n                        relatedCacheGeneration = 1 - relatedCacheGeneration;\n                        relatedCache[relatedCacheGeneration] = {};\n                        relatedCacheFilling[relatedCacheGeneration] = 0;\n                    }\n                    relatedCache[relatedCacheGeneration][cacheKey] = related;\n                    relatedCacheFilling[relatedCacheGeneration] += 1;\n                }, 0);\n            }\n        };\n    }());\n\n    let lineMouseover = function (lineId) {\n        allComparisonItems[lineId].classList.add('hovering');\n    };\n\n    let lineMouseout = function (lineId) {\n        allComparisonItems[lineId].classList.remove('hovering');\n    };\n\n    const mainPlot = Plotter.createPlot(\n        document.getElementById('mainPlot'), years, ticksX, updateTooltip,\n        document.getElementById('tooltipTemplate'), lineMouseover, lineMouseout);\n\n    document.getElementById('mainLegend').querySelectorAll('ul').forEach(\n        element => element.addEventListener('mouseout', () => mainPlot.lineToFront())\n    );\n\n    allComparisonItems.forEach((element, index) => {\n        element.addEventListener('mouseover', () => { mainPlot.lineToFront(index); mainPlot.hoverLine(index) });\n        element.addEventListener('mouseout', () => mainPlot.unhoverLine(index));\n        element.addEventListener('click', () => mainPlot.setMainLine(index));\n\n        const legendLink = element.querySelector('a');\n        if (legendLink) {\n            legendLink.addEventListener('click', ev => {\n                ev.preventDefault();\n                legendLink.blur();\n                updatePlot(legendLink.innerText, null);\n            });\n        }\n\n        const inputs = element.querySelectorAll('input');\n        if (inputs.length !== 0) {\n            const [otherWordInput, removeButton] = inputs;\n            let manualIndex = manualComparisonInputs.length;\n            manualComparisonInputs.push(otherWordInput);\n            manualComparisonRemoveButtons.push(removeButton);\n\n            let inputEventHandler = event => manualComparisonChanged(event, otherWordInput, manualIndex);\n            otherWordInput.onkeydown = inputEventHandler;\n            otherWordInput.onchange = inputEventHandler;\n            otherWordInput.onclick = inputEventHandler;\n            otherWordInput.onblur = inputEventHandler;\n\n            removeButton.onclick = () => removeManualComparison(manualIndex);\n\n            if (manualIndex === 0) {\n                otherWordInput.style.width = '0';\n                removeButton.style.display = 'none';\n            } else {\n                element.style.display = 'none';\n            }\n        }\n    });\n\n    let [handle, metaData] = await Promise.all([\n        backendPromise.then(backend => backend.loadFile()),\n        fetch(_assets_googlebooks_metadata_1800to2008_vocabsize30000_bin__WEBPACK_IMPORTED_MODULE_1__[\"default\"]).then(file => file.json())\n    ]);\n    document.getElementById('downloadProgressPane').style.display = 'none';\n    document.querySelector('.app').style.display = 'block';\n\n    let inverseVocab = {};\n    metaData.vocab.forEach((word, index) => inverseVocab[word] = index);\n\n\n    let wordInput = document.querySelector('.wordInput');\n    let wordInputError = document.querySelector('.wordInputError');\n    // We listen to several events to make the UI snappier. For example,\n    // `onkeydown` fires earlier than `onchange` but it misses some changes such\n    // as \"right-click --> paste\". Listening to several events does not\n    // significantly increase  computational cost because the event handler\n    // performs expensive calculations only if anything actually changed.\n    wordInput.onkeydown = wordChanged;\n    wordInput.onchange = wordChanged;\n    wordInput.onclick = wordChanged;\n    wordInput.onblur = wordChanged;\n\n    document.getElementById('shareFacebookButton').onclick = shareOnFacebook;\n    document.getElementById('shareTwitterButton').onclick = shareOnTwitter;\n    document.getElementById('copyLinkButton').onclick = copyLink;\n\n    let shareTwitterButton = document.getElementById('shareTwitterButton');\n    shareTwitterButton.onclick = shareOnTwitter;\n\n    window.addEventListener('popstate', on_popstate);\n    setTimeout(() => {\n        on_popstate();\n        if (currentWord === '') {\n            mainPlot.showInputPrompt();\n        }\n        wordInput.selectionStart = wordInput.selectionEnd = wordInput.value.length;\n        wordInput.focus();\n    }, 0);\n\n    function getLinkAndDescription() {\n        let link = 'https://robamler.github.io/linguistic-flux-capacitor';\n        if (currentWord !== '') {\n            link = link + location.hash;\n        }\n        let description = (\n            'Explore how the meaning of ' +\n            (currentWord === '' ? 'words' : 'the word \"' + currentWord + '\"') +\n            ' has changed over the past two centuries'\n        );\n        return [link, description];\n    }\n\n    function shareOnFacebook(event) {\n        event.preventDefault();\n        let [link, description] = getLinkAndDescription();\n        let url = (\n            'https://www.facebook.com/share.php?u=' + encodeURIComponent(link)\n            + '&quote=' + encodeURIComponent(description + ' using this web app.')\n        );\n        window.open(url, 'share-dialog', 'width=626,height=436');\n    }\n\n    function shareOnTwitter(event) {\n        event.preventDefault();\n        let [link, description] = getLinkAndDescription();\n        window.open(\n            'https://twitter.com/intent/tweet?text=' + encodeURIComponent(description + ': ' + link),\n            'share-dialog',\n            'width=626,height=436'\n        );\n    }\n\n    async function copyLink(event) {\n        event.preventDefault();\n        let [link, description] = getLinkAndDescription();\n        await navigator.clipboard.writeText(description + ': ' + link);\n        let toast = document.querySelector('.toast');\n        toast.style.display = 'inline-block';\n        toast.style.opacity = 1;\n        setTimeout(() => toast.style.opacity = 0, 3000);\n        setTimeout(() => toast.style.display = 'none', 3900);\n    }\n\n    function on_popstate() {\n        let newMainWord = \"\";\n        let newManualComparisons = [];\n        for (let url_component of window.location.hash.substr(1).split(\"&\")) {\n            let [key, value] = url_component.split(\"=\");\n            if (key === \"w\") {\n                newMainWord = decodeURIComponent(value);\n            } else if (key === \"o\" && value !== \"\") {\n                newManualComparisons = value.split(\"+\").map(decodeURIComponent);\n            }\n        }\n\n        updatePlot(newMainWord, newManualComparisons, true);\n    }\n\n    function wordChanged() {\n        let handler = () => updatePlot(wordInput.value.trim(), null);\n\n        // Wait for next turn in JS executor to let change take effect.\n        setTimeout(handler, 0);\n\n        // Fire one more time with some delay. This is an ugly hack to work around an\n        // unresolved issue where sometimes the last keystroke does not get registered\n        // (mainly on Safari, but sometimes also on other browsers). The handler doesn't\n        // do much work if `updatePlot` realizes that nothing actually changed.\n        setTimeout(handler, 300);\n    }\n\n    function manualComparisonChanged(event, inputField, index) {\n        let handler = () => {\n            let otherWord = inputField.value.trim();\n\n            // Make a *copy* of the array so that `updatePlot` can check if anything changed.\n            let newManualComparisons = [...manualComparisons];\n            if (index >= newManualComparisons.length - 1 && otherWord === '') {\n                // Last nonempty input box was emptied out. Remove the word. The input box\n                // will still stick around anyway.\n                newManualComparisons.splice(index, 1);\n            } else if (index < newManualComparisons.length) {\n                newManualComparisons[index] = otherWord;\n            } else {\n                newManualComparisons.push(otherWord);\n            }\n            updatePlot(null, newManualComparisons);\n\n            if (event.type !== 'blur' && event.type !== 'change') {\n                mainPlot.setMainLine(suggestedComparisonItems.length + index);\n            }\n        };\n\n        // Wait for next turn in JS executor to let change take effect.\n        setTimeout(handler, 0);\n\n        // Fire one more time with some delay. This is an ugly hack to work around an\n        // unresolved issue where sometimes the last keystroke does not get registered.\n        // (mainly on Safari, but sometimes also on other browsers). The handler doesn't\n        // do much work if `updatePlot` realizes that nothing actually changed.\n        setTimeout(handler, 300);\n    }\n\n    function removeManualComparison(index) {\n        // Make a *copy* of the array so that `updatePlot` can check if anything changed.\n        let newManualComparisons = [...manualComparisons];\n        if (index < newManualComparisons.length) {\n            newManualComparisons.splice(index, 1); // Removes the element.\n            updatePlot(null, newManualComparisons);\n        }\n    }\n\n    function updatePlot(newMainWord, newManualComparisons, suppress_save_state = false) {\n        let mainWordChanged = false;\n        let manualComparisonsChanged = false;\n\n        if (newMainWord !== null) {\n            if (wordInput.value.trim() !== newMainWord) {\n                wordInput.value = newMainWord;\n            }\n            let newMainWordId = inverseVocab[newMainWord];\n            if (newMainWord === '' || typeof newMainWordId !== 'undefined') {\n                wordInput.classList.remove('invalid');\n                wordInputError.style.display = 'none';\n                if (newMainWord !== currentWord) {\n                    mainWordChanged = true;\n                    currentWord = newMainWord;\n                    suggestedComparisonIds = handle.largest_changes_wrt(newMainWordId, suggestedComparisonItems.length, 2, 2);\n                }\n            } else {\n                // Out of vocabulary word entered. Treat as if `currentWord` did not change. \n                // We may still want to update the plot in case `manualComparisons` changed.\n                wordInput.classList.add('invalid');\n                wordInputError.style.display = 'inline-block';\n            }\n        }\n\n        if (newManualComparisons !== null) {\n            let newManualComparisonIds = [];\n            if (newManualComparisons.length > manualComparisonItems.length) {\n                newManualComparisons.splice(manualComparisonItems.length); // Removes everything that flows over.\n            }\n\n            // Update input boxes in legend.\n            for (let i = 0; i < newManualComparisons.length; i += 1) {\n                let otherWord = newManualComparisons[i];\n                let otherWordId = inverseVocab[otherWord];\n                newManualComparisonIds.push(otherWordId);\n\n                if (i >= manualComparisons.length || manualComparisons[i] !== otherWord) {\n                    manualComparisonsChanged = true;\n                    if (typeof otherWordId === 'undefined') {\n                        manualComparisonInputs[i].classList.add('invalid');\n                        manualComparisonInputs[i].setAttribute('title', 'word not found');\n                        manualComparisonInputs[i].parentElement.removeAttribute('title');\n                        manualComparisonInputs[i].parentElement.classList.add('inactive');\n                    } else {\n                        manualComparisonInputs[i].classList.remove('invalid');\n                        manualComparisonInputs[i].removeAttribute('title');\n                        manualComparisonInputs[i].parentElement.setAttribute(\n                            'title', 'Click and move mouse across diagram to explore further.'\n                        );\n                        manualComparisonInputs[i].parentElement.classList.remove('inactive');\n                    }\n                    manualComparisonItems[i].style.display = 'list-item';\n                    manualComparisonRemoveButtons[i].style.display = 'inline';\n                    if (manualComparisonInputs[i].value.trim() !== otherWord) {\n                        manualComparisonInputs[i].value = otherWord;\n                    }\n                    inputWidthMeasure.textContent = otherWord;\n                    manualComparisonInputs[i].style.width = inputWidthMeasure.offsetWidth + 'px';\n                }\n            }\n\n            if (newManualComparisons.length !== manualComparisons.length) {\n                manualComparisonsChanged = true;\n\n                if (newManualComparisons.length < manualComparisonItems.length) {\n                    // There's still room for additional manual comparisons, so show an empty input box.\n                    manualComparisonItems[newManualComparisons.length].style.display = 'list-item';\n                    manualComparisonInputs[newManualComparisons.length].value = '';\n                    manualComparisonInputs[newManualComparisons.length].style.width = '0';\n                    manualComparisonInputs[newManualComparisons.length].classList.remove('invalid');\n                    manualComparisonInputs[newManualComparisons.length].setAttribute(\n                        'title', 'Enter a secondary word here.'\n                    );\n                    manualComparisonInputs[newManualComparisons.length].parentElement.classList.add('inactive');\n                    manualComparisonInputs[newManualComparisons.length].parentElement.removeAttribute('title');\n                    manualComparisonRemoveButtons[newManualComparisons.length].style.display = 'none';\n\n                    // Remove all input boxes below.\n                    for (let i = newManualComparisons.length + 1; i < manualComparisonItems.length; i += 1) {\n                        manualComparisonItems[i].style.display = 'none';\n                    }\n                }\n            }\n\n            manualComparisons = newManualComparisons;\n            manualComparisonIds = newManualComparisonIds;\n        }\n\n        // Do the expensive stuff only if anything actually changed. This allows us to\n        // attach this function on lots of events to catch changes as early as possible\n        // without firing multiple times on the same change.\n        if (mainWordChanged || manualComparisonsChanged) {\n            mainPlot.clear();\n\n            if (currentWord === '') {\n                document.title = \"The Linguistic Flux Capacitor\";\n                mainPlot.showInputPrompt();\n                legend.classList.add('empty');\n                if (!suppress_save_state) {\n                    history.pushState(null, \"The Linguistic Flux Capacitor\", \"#\");\n                }\n                return;\n            }\n\n            document.title = \"The Linguistic Flux Capacitor: \" + currentWord;\n\n            if (!suppress_save_state) {\n                let stateUrl = \"#v=0&c=en&w=\" + encodeURIComponent(currentWord);\n                if (manualComparisons.length != 0) {\n                    stateUrl = stateUrl + \"&o=\" + manualComparisons.map(encodeURIComponent).join(\"+\");\n                }\n                history.pushState(null, \"The Linguistic Flux Capacitor: \" + currentWord, stateUrl);\n            }\n\n            legend.classList.remove('empty');\n            allComparisonItems.forEach(el => {\n                el.classList.remove('hovering');\n                el.firstElementChild.textContent = currentWord;\n            });\n\n            let otherWordIds = [...suggestedComparisonIds];\n            let comparisonColors = [];\n            for (let i = 0; i < otherWordIds.length; i += 1) {\n                comparisonColors.push(i);\n            }\n            manualComparisonIds.forEach((id, index) => {\n                if (typeof id !== 'undefined') {\n                    otherWordIds.push(id)\n                    comparisonColors.push(suggestedComparisonIds.length + index);\n                }\n            });\n\n            let mainWordId = inverseVocab[currentWord];\n            let wordIdRepeated = Array(otherWordIds.length).fill(mainWordId);\n            let concatenatedTrajectories = handle.pairwise_trajectories(wordIdRepeated, otherWordIds);\n            let trajectoryLength = concatenatedTrajectories.length / otherWordIds.length;\n\n            otherWordIds.forEach((otherWordId, index) => {\n                let otherWord = metaData.vocab[otherWordId];\n                mainPlot.plotLine(\n                    concatenatedTrajectories.subarray(index * trajectoryLength, (index + 1) * trajectoryLength),\n                    comparisonColors[index],\n                    0,\n                    {\n                        word1: currentWord,\n                        word2: otherWord,\n                        word1Id: mainWordId,\n                        word2Id: otherWordId,\n                    },\n                    false,\n                    '\"' + currentWord + '\" ↔ \"' + otherWord + '\"\\n(click on line to explore relationship)'\n                );\n\n                if (index < suggestedComparisonItems.length) {\n                    allComparisonItems[index].firstElementChild.nextElementSibling.textContent = otherWord;\n                }\n            });\n        }\n    }\n}())\n\n\n//# sourceURL=webpack://linguistic-flux-capacitor/./src/index.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			loaded: false,
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/async module */
/******/ 	(() => {
/******/ 		var webpackThen = typeof Symbol === "function" ? Symbol("webpack then") : "__webpack_then__";
/******/ 		var webpackExports = typeof Symbol === "function" ? Symbol("webpack exports") : "__webpack_exports__";
/******/ 		var webpackError = typeof Symbol === "function" ? Symbol("webpack error") : "__webpack_error__";
/******/ 		var completeQueue = (queue) => {
/******/ 			if(queue) {
/******/ 				queue.forEach((fn) => (fn.r--));
/******/ 				queue.forEach((fn) => (fn.r-- ? fn.r++ : fn()));
/******/ 			}
/******/ 		}
/******/ 		var completeFunction = (fn) => (!--fn.r && fn());
/******/ 		var queueFunction = (queue, fn) => (queue ? queue.push(fn) : completeFunction(fn));
/******/ 		var wrapDeps = (deps) => (deps.map((dep) => {
/******/ 			if(dep !== null && typeof dep === "object") {
/******/ 				if(dep[webpackThen]) return dep;
/******/ 				if(dep.then) {
/******/ 					var queue = [];
/******/ 					dep.then((r) => {
/******/ 						obj[webpackExports] = r;
/******/ 						completeQueue(queue);
/******/ 						queue = 0;
/******/ 					}, (e) => {
/******/ 						obj[webpackError] = e;
/******/ 						completeQueue(queue);
/******/ 						queue = 0;
/******/ 					});
/******/ 					var obj = {};
/******/ 					obj[webpackThen] = (fn, reject) => (queueFunction(queue, fn), dep['catch'](reject));
/******/ 					return obj;
/******/ 				}
/******/ 			}
/******/ 			var ret = {};
/******/ 			ret[webpackThen] = (fn) => (completeFunction(fn));
/******/ 			ret[webpackExports] = dep;
/******/ 			return ret;
/******/ 		}));
/******/ 		__webpack_require__.a = (module, body, hasAwait) => {
/******/ 			var queue = hasAwait && [];
/******/ 			var exports = module.exports;
/******/ 			var currentDeps;
/******/ 			var outerResolve;
/******/ 			var reject;
/******/ 			var isEvaluating = true;
/******/ 			var nested = false;
/******/ 			var whenAll = (deps, onResolve, onReject) => {
/******/ 				if (nested) return;
/******/ 				nested = true;
/******/ 				onResolve.r += deps.length;
/******/ 				deps.map((dep, i) => (dep[webpackThen](onResolve, onReject)));
/******/ 				nested = false;
/******/ 			};
/******/ 			var promise = new Promise((resolve, rej) => {
/******/ 				reject = rej;
/******/ 				outerResolve = () => (resolve(exports), completeQueue(queue), queue = 0);
/******/ 			});
/******/ 			promise[webpackExports] = exports;
/******/ 			promise[webpackThen] = (fn, rejectFn) => {
/******/ 				if (isEvaluating) { return completeFunction(fn); }
/******/ 				if (currentDeps) whenAll(currentDeps, fn, rejectFn);
/******/ 				queueFunction(queue, fn);
/******/ 				promise['catch'](rejectFn);
/******/ 			};
/******/ 			module.exports = promise;
/******/ 			body((deps) => {
/******/ 				currentDeps = wrapDeps(deps);
/******/ 				var fn;
/******/ 				var getResult = () => (currentDeps.map((d) => {
/******/ 					if(d[webpackError]) throw d[webpackError];
/******/ 					return d[webpackExports];
/******/ 				}))
/******/ 				var promise = new Promise((resolve, reject) => {
/******/ 					fn = () => (resolve(getResult));
/******/ 					fn.r = 0;
/******/ 					whenAll(currentDeps, fn, reject);
/******/ 				});
/******/ 				return fn.r ? promise : getResult();
/******/ 			}, (err) => (err && reject(promise[webpackError] = err), outerResolve()));
/******/ 			isEvaluating = false;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/compat get default export */
/******/ 	(() => {
/******/ 		// getDefaultExport function for compatibility with non-harmony modules
/******/ 		__webpack_require__.n = (module) => {
/******/ 			var getter = module && module.__esModule ?
/******/ 				() => (module['default']) :
/******/ 				() => (module);
/******/ 			__webpack_require__.d(getter, { a: getter });
/******/ 			return getter;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	(() => {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = (chunkId) => {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce((promises, key) => {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	(() => {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = (chunkId) => {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".js";
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/harmony module decorator */
/******/ 	(() => {
/******/ 		__webpack_require__.hmd = (module) => {
/******/ 			module = Object.create(module);
/******/ 			if (!module.children) module.children = [];
/******/ 			Object.defineProperty(module, 'exports', {
/******/ 				enumerable: true,
/******/ 				set: () => {
/******/ 					throw new Error('ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: ' + module.id);
/******/ 				}
/******/ 			});
/******/ 			return module;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/load script */
/******/ 	(() => {
/******/ 		var inProgress = {};
/******/ 		var dataWebpackPrefix = "linguistic-flux-capacitor:";
/******/ 		// loadScript function to load a script via script tag
/******/ 		__webpack_require__.l = (url, done, key, chunkId) => {
/******/ 			if(inProgress[url]) { inProgress[url].push(done); return; }
/******/ 			var script, needAttach;
/******/ 			if(key !== undefined) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				for(var i = 0; i < scripts.length; i++) {
/******/ 					var s = scripts[i];
/******/ 					if(s.getAttribute("src") == url || s.getAttribute("data-webpack") == dataWebpackPrefix + key) { script = s; break; }
/******/ 				}
/******/ 			}
/******/ 			if(!script) {
/******/ 				needAttach = true;
/******/ 				script = document.createElement('script');
/******/ 		
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.setAttribute("data-webpack", dataWebpackPrefix + key);
/******/ 				script.src = url;
/******/ 			}
/******/ 			inProgress[url] = [done];
/******/ 			var onScriptComplete = (prev, event) => {
/******/ 				// avoid mem leaks in IE.
/******/ 				script.onerror = script.onload = null;
/******/ 				clearTimeout(timeout);
/******/ 				var doneFns = inProgress[url];
/******/ 				delete inProgress[url];
/******/ 				script.parentNode && script.parentNode.removeChild(script);
/******/ 				doneFns && doneFns.forEach((fn) => (fn(event)));
/******/ 				if(prev) return prev(event);
/******/ 			}
/******/ 			;
/******/ 			var timeout = setTimeout(onScriptComplete.bind(null, undefined, { type: 'timeout', target: script }), 120000);
/******/ 			script.onerror = onScriptComplete.bind(null, script.onerror);
/******/ 			script.onload = onScriptComplete.bind(null, script.onload);
/******/ 			needAttach && document.head.appendChild(script);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm loading */
/******/ 	(() => {
/******/ 		__webpack_require__.v = (exports, wasmModuleId, wasmModuleHash, importsObj) => {
/******/ 			var req = fetch(__webpack_require__.p + "" + wasmModuleHash + ".module.wasm");
/******/ 			if (typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 				return WebAssembly.instantiateStreaming(req, importsObj)
/******/ 					.then((res) => (Object.assign(exports, res.instance.exports)));
/******/ 			}
/******/ 			return req
/******/ 				.then((x) => (x.arrayBuffer()))
/******/ 				.then((bytes) => (WebAssembly.instantiate(bytes, importsObj)))
/******/ 				.then((res) => (Object.assign(exports, res.instance.exports)));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript)
/******/ 				scriptUrl = document.currentScript.src
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) scriptUrl = scripts[scripts.length - 1].src
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/jsonp chunk loading */
/******/ 	(() => {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded and loading chunks
/******/ 		// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 		// [resolve, reject, Promise] = chunk loading, 0 = chunk loaded
/******/ 		var installedChunks = {
/******/ 			"main": 0
/******/ 		};
/******/ 		
/******/ 		__webpack_require__.f.j = (chunkId, promises) => {
/******/ 				// JSONP chunk loading for javascript
/******/ 				var installedChunkData = __webpack_require__.o(installedChunks, chunkId) ? installedChunks[chunkId] : undefined;
/******/ 				if(installedChunkData !== 0) { // 0 means "already installed".
/******/ 		
/******/ 					// a Promise means "currently loading".
/******/ 					if(installedChunkData) {
/******/ 						promises.push(installedChunkData[2]);
/******/ 					} else {
/******/ 						if(true) { // all chunks have JS
/******/ 							// setup Promise in chunk cache
/******/ 							var promise = new Promise((resolve, reject) => (installedChunkData = installedChunks[chunkId] = [resolve, reject]));
/******/ 							promises.push(installedChunkData[2] = promise);
/******/ 		
/******/ 							// start chunk loading
/******/ 							var url = __webpack_require__.p + __webpack_require__.u(chunkId);
/******/ 							// create error before stack unwound to get useful stacktrace later
/******/ 							var error = new Error();
/******/ 							var loadingEnded = (event) => {
/******/ 								if(__webpack_require__.o(installedChunks, chunkId)) {
/******/ 									installedChunkData = installedChunks[chunkId];
/******/ 									if(installedChunkData !== 0) installedChunks[chunkId] = undefined;
/******/ 									if(installedChunkData) {
/******/ 										var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 										var realSrc = event && event.target && event.target.src;
/******/ 										error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 										error.name = 'ChunkLoadError';
/******/ 										error.type = errorType;
/******/ 										error.request = realSrc;
/******/ 										installedChunkData[1](error);
/******/ 									}
/******/ 								}
/******/ 							};
/******/ 							__webpack_require__.l(url, loadingEnded, "chunk-" + chunkId, chunkId);
/******/ 						} else installedChunks[chunkId] = 0;
/******/ 					}
/******/ 				}
/******/ 		};
/******/ 		
/******/ 		// no prefetching
/******/ 		
/******/ 		// no preloaded
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		// install a JSONP callback for chunk loading
/******/ 		var webpackJsonpCallback = (parentChunkLoadingFunction, data) => {
/******/ 			var [chunkIds, moreModules, runtime] = data;
/******/ 			// add "moreModules" to the modules object,
/******/ 			// then flag all "chunkIds" as loaded and fire callback
/******/ 			var moduleId, chunkId, i = 0;
/******/ 			if(chunkIds.some((id) => (installedChunks[id] !== 0))) {
/******/ 				for(moduleId in moreModules) {
/******/ 					if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 						__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 					}
/******/ 				}
/******/ 				if(runtime) var result = runtime(__webpack_require__);
/******/ 			}
/******/ 			if(parentChunkLoadingFunction) parentChunkLoadingFunction(data);
/******/ 			for(;i < chunkIds.length; i++) {
/******/ 				chunkId = chunkIds[i];
/******/ 				if(__webpack_require__.o(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 					installedChunks[chunkId][0]();
/******/ 				}
/******/ 				installedChunks[chunkId] = 0;
/******/ 			}
/******/ 		
/******/ 		}
/******/ 		
/******/ 		var chunkLoadingGlobal = self["webpackChunklinguistic_flux_capacitor"] = self["webpackChunklinguistic_flux_capacitor"] || [];
/******/ 		chunkLoadingGlobal.forEach(webpackJsonpCallback.bind(null, 0));
/******/ 		chunkLoadingGlobal.push = webpackJsonpCallback.bind(null, chunkLoadingGlobal.push.bind(chunkLoadingGlobal));
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./src/index.js");
/******/ 	
/******/ })()
;