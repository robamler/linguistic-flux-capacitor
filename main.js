/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + ({}[chunkId]||chunkId) + ".js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/word_history_explorer_backend_bg.wasm": function() {
/******/ 			return {
/******/ 				"./word_history_explorer_backend_bg.js": {
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["../pkg/word_history_explorer_backend_bg.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/word_history_explorer_backend_bg.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/word_history_explorer_backend_bg.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/word_history_explorer_backend_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/word_history_explorer_backend_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/word_history_explorer_backend_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/word_history_explorer_backend_bg.wasm":"57d2e7dce903d1fc0256"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./src/index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin":
/*!*******************************************************************!*\
  !*** ./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin ***!
  \*******************************************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (__webpack_require__.p + \"e5d63ffe4b441ec1b9ff0f187ed189b7.bin\");\n\n//# sourceURL=webpack:///./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin?");

/***/ }),

/***/ "./node_modules/css-loader/dist/cjs.js!./src/styles.css":
/*!**************************************************************!*\
  !*** ./node_modules/css-loader/dist/cjs.js!./src/styles.css ***!
  \**************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// Imports\nvar ___CSS_LOADER_API_IMPORT___ = __webpack_require__(/*! ../node_modules/css-loader/dist/runtime/api.js */ \"./node_modules/css-loader/dist/runtime/api.js\");\nexports = ___CSS_LOADER_API_IMPORT___(false);\n// Module\nexports.push([module.i, \"body {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 20px;\\n    position: absolute;\\n    left: 0;\\n    right: 0;\\n    top: 0;\\n    bottom: 0;\\n}\\n\\n.pageContainer {\\n    width: 50em;\\n    position: absolute;\\n    left: 50%;\\n    transform: translateX(-50%);\\n}\\n\\nh1 {\\n    text-align: center;\\n    font-weight: normal;\\n    font-size: 280%;\\n    font-family: 'DM Serif Display', serif;\\n    margin: 0.3em 0 0.7em 0;\\n    padding: 0;\\n}\\n\\n.app>h2 {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-weight: normal;\\n    text-align: center;\\n    font-size: 160%;\\n    margin: 0.2em 0;\\n}\\n\\n.legend>h3 {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 110%;\\n    font-weight: bold;\\n    text-align: center;\\n    margin: 0 0 0.3em 0;\\n    display: block;\\n    white-space: nowrap;\\n}\\n\\nh2 {\\n    font-size: 230%;\\n    font-weight: normal;\\n    font-family: 'DM Serif Display', serif;\\n    margin: 1.4em 0 0.6em 0;\\n    line-height: 100%;\\n}\\n\\nh3 {\\n    font-family: 'DM Serif Display', serif;\\n    font-size: 125%;\\n    font-weight: 700;\\n    margin: 1em 0.4em -0.3em 0;\\n    padding: 0;\\n    display: inline;\\n}\\n\\n.prose {\\n    color: #333;\\n    line-height: 150%;\\n}\\n\\nfooter {\\n    opacity: 0.7;\\n    font-size: 80%;\\n    margin: 2em 0 1em 0;\\n}\\n\\n.subtitle {\\n    font-size: 60%;\\n}\\n\\n.splashScreen {\\n    text-align: center;\\n    font-size: 120%;\\n    margin: 4em 2em 7em 2em;\\n}\\n\\n.appContainer {\\n    width: 40em;\\n}\\n\\n.app {\\n    display: none;\\n}\\n\\n#downloadProgressPane {\\n    color: #0d441d;\\n}\\n\\n.progressBarContainer {\\n    margin: 0.7em;\\n    border: #36864e solid 0.1em;\\n    border-radius: 0.3em;\\n    position: relative;\\n    height: 1.5em;\\n}\\n\\n.progressBar {\\n    background: #bfdfc9;\\n    position: absolute;\\n    left: 0;\\n    top: 0;\\n    bottom: 0;\\n    width: 0;\\n    z-index: 0;\\n    border-radius: 0.15em;\\n}\\n\\n.progressText {\\n    position: absolute;\\n    margin: auto;\\n    left: 0;\\n    right: 0;\\n    top: 0.1em;\\n    text-shadow: 0 0 0.2em #fff;\\n}\\n\\n.plotAndLegend {\\n    position: relative;\\n    margin-top: 1.3em;\\n}\\n\\n.plotContainer {\\n    position: relative;\\n    left: -2em;\\n    width: 41em;\\n}\\n\\n.centered {\\n    text-align: center;\\n}\\n\\n.wordInput {\\n    margin: 0 0 0.1em 0;\\n    font-size: 160%;\\n    width: 40%;\\n    text-align: center;\\n}\\n\\ninput {\\n    border: solid 0.05em #ccc;\\n    border-radius: 0.2em;\\n    padding: 0.2em;\\n}\\n\\ninput:focus {\\n    border-color: #86bdf0;\\n    background: #fafbfd;\\n    outline: none;\\n}\\n\\ninput.invalid {\\n    background-color: #fdebeb;\\n}\\n\\ninput.invalid:focus {\\n    border-color: #9c5555;\\n}\\n\\n.tooltipContent .explanation {\\n    font-size: 90%;\\n}\\n\\n.tooltipContent th {\\n    font-size: 120%;\\n}\\n\\n.tooltipContent th, .tooltipContent td {\\n    vertical-align: baseline;\\n}\\n\\n.tooltipContent th.left, .tooltipContent td.left {\\n    padding-right: 0.3em;\\n}\\n\\n.tooltipContent th.right, .tooltipContent td.right {\\n    padding-left: 0.3em;\\n}\\n\\n.tooltipContent .year {\\n    font-size: 140%;\\n    font-weight: bold;\\n}\\n\\n.tooltipContent .interleave {\\n    padding: 0.2em 0 0.5em 0;\\n}\\n\\n.hint {\\n    opacity: 0.7;\\n    font-size: 80%;\\n    margin-top: 0.1em;\\n}\\n\\n.tooltipContent table {\\n    width: 100%;\\n}\\n\\n.tooltipContent .wait {\\n    display: none;\\n    position: absolute;\\n    text-align: center;\\n    width: 10em;\\n    white-space: normal;\\n    left: 50%;\\n    top: 50%;\\n    transform: translate(-50%, -50%);\\n    opacity: 0.8;\\n    font-size: 80%;\\n}\\n\\n.tooltipContent.waiting .wait {\\n    display: block;\\n}\\n\\n.tooltipContent .suggestionsTable {\\n    visibility: visible;\\n}\\n\\n.tooltipContent.waiting .suggestionsTable {\\n    visibility: hidden;\\n}\\n\\n.tooltipContent .suggestion {\\n    padding: 0.05em 0;\\n    font-size: 90%;\\n}\\n\\n.tooltipContent .left {\\n    text-align: left;\\n}\\n\\n.tooltipContent .right {\\n    text-align: right;\\n}\\n\\n.tooltipContent .removeWordButton {\\n    background-color: #BCB8AE;\\n    /*Mindful Gray*/\\n    color: white;\\n    font-size: 80%;\\n}\\n\\na {\\n    text-decoration: none;\\n    color: #1135a0;\\n}\\n\\n.legend {\\n    display: none;\\n    background-color: #e8e8e8;\\n    border-radius: 0.5em;\\n    padding: 0.6em 0.7em;\\n    width: auto;\\n    position: absolute;\\n    left: 40em;\\n    top: 0;\\n    min-width: 10em;\\n}\\n\\n.legend>ul {\\n    list-style-type: none;\\n    margin: 0 0 0.5em 0;\\n    padding: 0;\\n}\\n\\n.legend>ul>li {\\n    margin: -0.2em -0.4em 0.0em -0.4em;\\n    padding: 0.2em 0.4em 0.3em 0.4em;\\n    border-radius: 0.2em;\\n    white-space: nowrap;\\n    cursor: default;\\n}\\n\\n.legend>ul>li:hover, .legend>ul>li.hovering {\\n    background-color: #f8f8f8;\\n}\\n\\n.legend>ul>li>a {\\n    text-decoration: none;\\n    color: #152d74;\\n}\\n\\n.legend>ul>li::before {\\n    content: \\\"⸺\\\";\\n    margin-right: 0.5em;\\n    font-weight: bold;\\n}\\n\\n.legend>ul>li.color0::before {\\n    color: #f94a01;\\n}\\n\\n.legend>ul>li.color1::before {\\n    color: #6b42b6;\\n}\\n\\n.legend>ul>li.color2::before {\\n    color: #11a854;\\n}\\n\\n.legend>ul>li.color3::before {\\n    color: #128db2;\\n}\\n\\n.legend>ul>li.color4::before {\\n    color: #e12fbc;\\n}\\n\\n.legend>ul>li.color5::before {\\n    color: #e6ab02;\\n}\\n\\n.legend>ul>li.color6::before {\\n    color: #9ed034;\\n}\\n\\n.legend>ul>li.color7::before {\\n    color: #fabebe;\\n}\\n\\n.legend>ul>li.color8::before {\\n    color: #ffd8b1;\\n}\\n\\n.legend>ul>li.color9::before {\\n    color: #808000;\\n}\\n\\n.legend>ul>li.color10::before {\\n    color: #666666;\\n}\\n\\n.legend input[type=\\\"text\\\"] {\\n    font-size: 80%;\\n    min-width: 5.2em;\\n}\\n\\n.legend input[type=\\\"button\\\"] {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 90%;\\n    width: 1.4em;\\n    height: 1.4em;\\n    line-height: 0;\\n}\\n\\n.inputWidthMeasure {\\n    font-family: \\\"Roboto\\\", \\\"Helvetica Neue\\\", Arial, sans-serif;\\n    font-size: 90%;\\n    white-space: nowrap;\\n    position: absolute;\\n    opacity: 0;\\n}\\n\\n.pinWordButton {\\n    background-color: #BCB8AE;\\n    /*Mindful Gray*/\\n    border: none;\\n    color: white;\\n    padding: 10px 32px;\\n    text-align: center;\\n    text-decoration: none;\\n    display: inline-block;\\n    font-size: 80%;\\n}\\n\\n.getUrlButton {\\n    background-color: #BCB8AE;\\n    /*Mindful Gray*/\\n    border: none;\\n    color: white;\\n    padding: 5px 5px;\\n    text-align: center;\\n    text-decoration: none;\\n    display: inline-block;\\n    font-size: 80%;\\n}\\n\", \"\"]);\n// Exports\nmodule.exports = exports;\n\n\n//# sourceURL=webpack:///./src/styles.css?./node_modules/css-loader/dist/cjs.js");

/***/ }),

/***/ "./node_modules/css-loader/dist/runtime/api.js":
/*!*****************************************************!*\
  !*** ./node_modules/css-loader/dist/runtime/api.js ***!
  \*****************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
eval("\n\n/*\n  MIT License http://www.opensource.org/licenses/mit-license.php\n  Author Tobias Koppers @sokra\n*/\n// css base code, injected by the css-loader\n// eslint-disable-next-line func-names\nmodule.exports = function (useSourceMap) {\n  var list = []; // return the list of modules as css string\n\n  list.toString = function toString() {\n    return this.map(function (item) {\n      var content = cssWithMappingToString(item, useSourceMap);\n\n      if (item[2]) {\n        return \"@media \".concat(item[2], \" {\").concat(content, \"}\");\n      }\n\n      return content;\n    }).join('');\n  }; // import a list of modules into the list\n  // eslint-disable-next-line func-names\n\n\n  list.i = function (modules, mediaQuery) {\n    if (typeof modules === 'string') {\n      // eslint-disable-next-line no-param-reassign\n      modules = [[null, modules, '']];\n    }\n\n    for (var i = 0; i < modules.length; i++) {\n      var item = [].concat(modules[i]);\n\n      if (mediaQuery) {\n        if (!item[2]) {\n          item[2] = mediaQuery;\n        } else {\n          item[2] = \"\".concat(mediaQuery, \" and \").concat(item[2]);\n        }\n      }\n\n      list.push(item);\n    }\n  };\n\n  return list;\n};\n\nfunction cssWithMappingToString(item, useSourceMap) {\n  var content = item[1] || ''; // eslint-disable-next-line prefer-destructuring\n\n  var cssMapping = item[3];\n\n  if (!cssMapping) {\n    return content;\n  }\n\n  if (useSourceMap && typeof btoa === 'function') {\n    var sourceMapping = toComment(cssMapping);\n    var sourceURLs = cssMapping.sources.map(function (source) {\n      return \"/*# sourceURL=\".concat(cssMapping.sourceRoot).concat(source, \" */\");\n    });\n    return [content].concat(sourceURLs).concat([sourceMapping]).join('\\n');\n  }\n\n  return [content].join('\\n');\n} // Adapted from convert-source-map (MIT)\n\n\nfunction toComment(sourceMap) {\n  // eslint-disable-next-line no-undef\n  var base64 = btoa(unescape(encodeURIComponent(JSON.stringify(sourceMap))));\n  var data = \"sourceMappingURL=data:application/json;charset=utf-8;base64,\".concat(base64);\n  return \"/*# \".concat(data, \" */\");\n}\n\n//# sourceURL=webpack:///./node_modules/css-loader/dist/runtime/api.js?");

/***/ }),

/***/ "./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js":
/*!****************************************************************************!*\
  !*** ./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js ***!
  \****************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
eval("\n\nvar isOldIE = function isOldIE() {\n  var memo;\n  return function memorize() {\n    if (typeof memo === 'undefined') {\n      // Test for IE <= 9 as proposed by Browserhacks\n      // @see http://browserhacks.com/#hack-e71d8692f65334173fee715c222cb805\n      // Tests for existence of standard globals is to allow style-loader\n      // to operate correctly into non-standard environments\n      // @see https://github.com/webpack-contrib/style-loader/issues/177\n      memo = Boolean(window && document && document.all && !window.atob);\n    }\n\n    return memo;\n  };\n}();\n\nvar getTarget = function getTarget() {\n  var memo = {};\n  return function memorize(target) {\n    if (typeof memo[target] === 'undefined') {\n      var styleTarget = document.querySelector(target); // Special case to return head of iframe instead of iframe itself\n\n      if (window.HTMLIFrameElement && styleTarget instanceof window.HTMLIFrameElement) {\n        try {\n          // This will throw an exception if access to iframe is blocked\n          // due to cross-origin restrictions\n          styleTarget = styleTarget.contentDocument.head;\n        } catch (e) {\n          // istanbul ignore next\n          styleTarget = null;\n        }\n      }\n\n      memo[target] = styleTarget;\n    }\n\n    return memo[target];\n  };\n}();\n\nvar stylesInDom = {};\n\nfunction modulesToDom(moduleId, list, options) {\n  for (var i = 0; i < list.length; i++) {\n    var part = {\n      css: list[i][1],\n      media: list[i][2],\n      sourceMap: list[i][3]\n    };\n\n    if (stylesInDom[moduleId][i]) {\n      stylesInDom[moduleId][i](part);\n    } else {\n      stylesInDom[moduleId].push(addStyle(part, options));\n    }\n  }\n}\n\nfunction insertStyleElement(options) {\n  var style = document.createElement('style');\n  var attributes = options.attributes || {};\n\n  if (typeof attributes.nonce === 'undefined') {\n    var nonce =  true ? __webpack_require__.nc : undefined;\n\n    if (nonce) {\n      attributes.nonce = nonce;\n    }\n  }\n\n  Object.keys(attributes).forEach(function (key) {\n    style.setAttribute(key, attributes[key]);\n  });\n\n  if (typeof options.insert === 'function') {\n    options.insert(style);\n  } else {\n    var target = getTarget(options.insert || 'head');\n\n    if (!target) {\n      throw new Error(\"Couldn't find a style target. This probably means that the value for the 'insert' parameter is invalid.\");\n    }\n\n    target.appendChild(style);\n  }\n\n  return style;\n}\n\nfunction removeStyleElement(style) {\n  // istanbul ignore if\n  if (style.parentNode === null) {\n    return false;\n  }\n\n  style.parentNode.removeChild(style);\n}\n/* istanbul ignore next  */\n\n\nvar replaceText = function replaceText() {\n  var textStore = [];\n  return function replace(index, replacement) {\n    textStore[index] = replacement;\n    return textStore.filter(Boolean).join('\\n');\n  };\n}();\n\nfunction applyToSingletonTag(style, index, remove, obj) {\n  var css = remove ? '' : obj.css; // For old IE\n\n  /* istanbul ignore if  */\n\n  if (style.styleSheet) {\n    style.styleSheet.cssText = replaceText(index, css);\n  } else {\n    var cssNode = document.createTextNode(css);\n    var childNodes = style.childNodes;\n\n    if (childNodes[index]) {\n      style.removeChild(childNodes[index]);\n    }\n\n    if (childNodes.length) {\n      style.insertBefore(cssNode, childNodes[index]);\n    } else {\n      style.appendChild(cssNode);\n    }\n  }\n}\n\nfunction applyToTag(style, options, obj) {\n  var css = obj.css;\n  var media = obj.media;\n  var sourceMap = obj.sourceMap;\n\n  if (media) {\n    style.setAttribute('media', media);\n  } else {\n    style.removeAttribute('media');\n  }\n\n  if (sourceMap && btoa) {\n    css += \"\\n/*# sourceMappingURL=data:application/json;base64,\".concat(btoa(unescape(encodeURIComponent(JSON.stringify(sourceMap)))), \" */\");\n  } // For old IE\n\n  /* istanbul ignore if  */\n\n\n  if (style.styleSheet) {\n    style.styleSheet.cssText = css;\n  } else {\n    while (style.firstChild) {\n      style.removeChild(style.firstChild);\n    }\n\n    style.appendChild(document.createTextNode(css));\n  }\n}\n\nvar singleton = null;\nvar singletonCounter = 0;\n\nfunction addStyle(obj, options) {\n  var style;\n  var update;\n  var remove;\n\n  if (options.singleton) {\n    var styleIndex = singletonCounter++;\n    style = singleton || (singleton = insertStyleElement(options));\n    update = applyToSingletonTag.bind(null, style, styleIndex, false);\n    remove = applyToSingletonTag.bind(null, style, styleIndex, true);\n  } else {\n    style = insertStyleElement(options);\n    update = applyToTag.bind(null, style, options);\n\n    remove = function remove() {\n      removeStyleElement(style);\n    };\n  }\n\n  update(obj);\n  return function updateStyle(newObj) {\n    if (newObj) {\n      if (newObj.css === obj.css && newObj.media === obj.media && newObj.sourceMap === obj.sourceMap) {\n        return;\n      }\n\n      update(obj = newObj);\n    } else {\n      remove();\n    }\n  };\n}\n\nmodule.exports = function (moduleId, list, options) {\n  options = options || {}; // Force single-tag solution on IE6-9, which has a hard limit on the # of <style>\n  // tags it will allow on a page\n\n  if (!options.singleton && typeof options.singleton !== 'boolean') {\n    options.singleton = isOldIE();\n  }\n\n  moduleId = options.base ? moduleId + options.base : moduleId;\n  list = list || [];\n\n  if (!stylesInDom[moduleId]) {\n    stylesInDom[moduleId] = [];\n  }\n\n  modulesToDom(moduleId, list, options);\n  return function update(newList) {\n    newList = newList || [];\n\n    if (Object.prototype.toString.call(newList) !== '[object Array]') {\n      return;\n    }\n\n    if (!stylesInDom[moduleId]) {\n      stylesInDom[moduleId] = [];\n    }\n\n    modulesToDom(moduleId, newList, options);\n\n    for (var j = newList.length; j < stylesInDom[moduleId].length; j++) {\n      stylesInDom[moduleId][j]();\n    }\n\n    stylesInDom[moduleId].length = newList.length;\n\n    if (stylesInDom[moduleId].length === 0) {\n      delete stylesInDom[moduleId];\n    }\n  };\n};\n\n//# sourceURL=webpack:///./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js?");

/***/ }),

/***/ "./src/index.js":
/*!**********************!*\
  !*** ./src/index.js ***!
  \**********************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _styles_css__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./styles.css */ \"./src/styles.css\");\n/* harmony import */ var _styles_css__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_styles_css__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var _assets_googlebooks_metadata_1800to2008_vocabsize30000_bin__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../assets/googlebooks_metadata_1800to2008_vocabsize30000.bin */ \"./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin\");\n\n\n\n//import FaceBookIcon from './facebook-icon.png'\n//import TwitterIcon from './facebook-icon.png'\n\n// Wasm modules must be imported asynchronously.\nlet backendPromise = __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./backend.js */ \"./src/backend.js\"));\n\n(async function () {\n    const Plotter = await __webpack_require__.e(/*! import() */ 1).then(__webpack_require__.bind(null, /*! ./plotting/main.mjs */ \"./src/plotting/main.mjs\"));\n    if (document.readyState === 'loading') {\n        await new Promise(function (resolve, _reject) {\n            window.addEventListener('DOMContentLoaded', resolve);\n        });\n    }\n\n    let years = [];\n    let ticksX = [];\n    for (let year = 1800; year <= 2008; year += 1) {\n        years.push(year);\n        if (year % 20 === 0) {\n            ticksX.push(year);\n        }\n    }\n\n    let currentWord = ''; // Invariant: `currentWord` is always either '' or a valid word from the vocabulary.\n    let manualComparisons = [];\n    let manualComparisonIds = [];\n\n    let legend = document.getElementById('mainLegend');\n    let suggestedComparisonItems = document.getElementById('suggestedComparisons').querySelectorAll('li');\n    let manualComparisonItems = document.getElementById('manualComparisons').querySelectorAll('li');\n    let suggestedComparisonIds = null;\n    let manualComparisonInputs = [];\n    let manualComparisonRemoveButtons = [];\n    let allComparisonItems = [...suggestedComparisonItems, ...manualComparisonItems];\n\n    let inputWidthMeasure = document.querySelector('.inputWidthMeasure');\n\n    let updateTooltip = (function () {\n        let tooltip = document.getElementById('tooltipTemplate');\n        let tooltipContent = tooltip.querySelector('.tooltipContent');\n        let yearPlaceholder = tooltip.querySelector('.year');\n        let word1Placeholder = tooltip.querySelector('.word1');\n        let word2Placeholder = tooltip.querySelector('.word2>a');\n        let relatedPlaceholders = [];\n        let relatedRemoveButtons = [];\n        let relatedTimeout = null;\n        let relatedCache = [{}, {}];\n        let relatedCacheFilling = [0, 0];\n        let relatedCacheGeneration = 0;\n        const MAX_CACHE_FILLING = 1024;\n\n        tooltip.querySelectorAll('.suggestion.left>a').forEach(el => {\n            relatedPlaceholders.push(el);\n            el.addEventListener('click', ev => {\n                ev.preventDefault();\n                el.blur();\n                updatePlot(el.innerText, null);\n            });\n        });\n        tooltip.querySelectorAll('.suggestion.right>a').forEach(el => {\n            relatedPlaceholders.push(el);\n            el.addEventListener('click', ev => {\n                ev.preventDefault();\n                el.blur();\n                updatePlot(el.innerText, null);\n            });\n        });\n        word2Placeholder.addEventListener('click', ev => {\n            ev.preventDefault();\n            word2Placeholder.blur();\n            updatePlot(word2Placeholder.innerText, null);\n        });\n\n        return function (tooltip, line, indexX) {\n            clearTimeout(relatedTimeout);\n            let payload = line.payload;\n            yearPlaceholder.innerText = years[indexX];\n            word1Placeholder.innerText = payload.word1;\n            word2Placeholder.innerText = payload.word2;\n\n            // TODO: look up word1 and word2 in cache independently.\n            let cacheKey = payload.word1Id + '-' + payload.word2Id + '-' + indexX;\n            let cachedCurrent = relatedCache[relatedCacheGeneration][cacheKey];\n            let cached = cachedCurrent || relatedCache[1 - relatedCacheGeneration][cacheKey];\n            if (typeof cached !== 'undefined') {\n                cached.forEach((r, i) => {\n                    relatedPlaceholders[i].innerText = metaData.vocab[r];\n                });\n                tooltipContent.classList.remove('waiting');\n\n                if (typeof cachedCurrent === 'undefined') {\n                    // Entry was found in old generation of the cache. Add it also to the current\n                    // generation so that it continues to stay cached for a while. If this would\n                    // overflow the current generation of the cache then flip generation instead.\n                    if (relatedCacheFilling[relatedCacheGeneration] === MAX_CACHE_FILLING) {\n                        relatedCacheGeneration = 1 - relatedCacheGeneration;\n                        relatedCache[relatedCacheGeneration] = {};\n                        relatedCacheFilling[relatedCacheGeneration] = 0;\n                    }\n                    relatedCache[relatedCacheGeneration][cacheKey] = cached;\n                    relatedCacheFilling[relatedCacheGeneration] += 1;\n                }\n            } else {\n                tooltipContent.classList.add('waiting');\n                relatedTimeout = setTimeout(() => {\n                    let related = handle.most_related_to_at_t([payload.word1Id, payload.word2Id], indexX, 7);\n                    related.forEach((r, i) => {\n                        relatedPlaceholders[i].innerText = metaData.vocab[r];\n                    });\n                    tooltipContent.classList.remove('waiting');\n\n                    if (relatedCacheFilling[relatedCacheGeneration] == MAX_CACHE_FILLING) {\n                        relatedCacheGeneration = 1 - relatedCacheGeneration;\n                        relatedCache[relatedCacheGeneration] = {};\n                        relatedCacheFilling[relatedCacheGeneration] = 0;\n                    }\n                    relatedCache[relatedCacheGeneration][cacheKey] = related;\n                    relatedCacheFilling[relatedCacheGeneration] += 1;\n                }, 0);\n            }\n        };\n    }());\n\n    let lineMouseover = function (lineId) {\n        allComparisonItems[lineId].classList.add('hovering');\n    };\n\n    let lineMouseout = function (lineId) {\n        allComparisonItems[lineId].classList.remove('hovering');\n    };\n\n    const mainPlot = Plotter.createPlot(\n        document.getElementById('mainPlot'), years, ticksX, updateTooltip,\n        document.getElementById('tooltipTemplate'), lineMouseover, lineMouseout);\n\n    allComparisonItems.forEach((element, index) => {\n        element.addEventListener('mouseover', () => mainPlot.hoverLine(index));\n        element.addEventListener('mouseout', () => mainPlot.unhoverLine(index));\n        element.addEventListener('click', () => mainPlot.setMainLine(index));\n\n        const legendLink = element.querySelector('a');\n        if (legendLink) {\n            legendLink.addEventListener('click', ev => {\n                ev.preventDefault();\n                legendLink.blur();\n                updatePlot(legendLink.innerText, null);\n            });\n        }\n\n        const inputs = element.querySelectorAll('input');\n        if (inputs.length !== 0) {\n            const [otherWordInput, removeButton] = inputs;\n            let manualIndex = manualComparisonInputs.length;\n            manualComparisonInputs.push(otherWordInput);\n            manualComparisonRemoveButtons.push(removeButton);\n\n            let inputEventHandler = () => manualComparisonChanged(otherWordInput, manualIndex);\n            otherWordInput.onkeydown = inputEventHandler;\n            otherWordInput.onchange = inputEventHandler;\n            otherWordInput.onclick = inputEventHandler;\n            otherWordInput.onblur = inputEventHandler;\n\n            removeButton.onclick = () => removeManualComparison(manualIndex);\n\n            if (manualIndex === 0) {\n                otherWordInput.style.width = '0';\n                removeButton.style.display = 'none';\n            } else {\n                element.style.display = 'none';\n            }\n        }\n    });\n\n    let [handle, metaData] = await Promise.all([\n        backendPromise.then(backend => backend.loadFile()),\n        fetch(_assets_googlebooks_metadata_1800to2008_vocabsize30000_bin__WEBPACK_IMPORTED_MODULE_1__[\"default\"]).then(file => file.json())\n    ]);\n    document.getElementById('downloadProgressPane').style.display = 'none';\n    document.querySelector('.app').style.display = 'block';\n\n    let inverseVocab = {};\n    metaData.vocab.forEach((word, index) => inverseVocab[word] = index);\n\n\n    let wordInput = document.querySelector('.wordInput');\n    // We listen to several events to make the UI snappier. For example,\n    // `onkeydown` fires earlier than `onchange` but it misses some changes such\n    // as \"right-click --> paste\". Listening to several events does not\n    // significantly increase  computational cost because the event handler\n    // performs expensive calculations only if anything actually changed.\n    wordInput.onkeydown = wordChanged;\n    wordInput.onchange = wordChanged;\n    wordInput.onclick = wordChanged;\n    wordInput.onblur = wordChanged;\n\n    let shareFacebookButton = document.getElementById('shareFacebookButton');\n    shareFacebookButton.onclick = shareFaceBook;\n\n    let shareTwitterButton = document.getElementById('shareTwitterButton');\n    shareTwitterButton.onclick = shareTwitter;\n\n    let showUrlButton = document.getElementById('showUrlButton');\n    //console.log(\"here\", showUrlButton);\n    showUrlButton.onclick = showUrl;\n\n    let dynamicMainLegendDOMs = [];//to keep track of dynamically added entries\n\n    window.addEventListener('popstate', on_popstate);\n    setTimeout(() => {\n        on_popstate();\n        wordInput.selectionStart = wordInput.selectionEnd = wordInput.value.length;\n        wordInput.focus();\n    }, 0);\n\n    let colorsAvail = ['color6', 'color7', 'color8', 'color9'];\n\n    function shareFaceBook() {\n        //console.log(\"//TODO: copy current link to url2\");\n        window.open(\n            'https://www.facebook.com/sharer/sharer.php?u=' + encodeURIComponent(location.href),\n            'facebook-share-dialog',\n            'width=626,height=436');\n    }\n\n    function shareTwitter() {\n        //console.log(\"//TODO: copy current link to url\");\n        window.open(\n            \"https://twitter.com/intent/tweet?text=check this out! -> \" + encodeURIComponent(location.href),\n            'facebook-share-dialog',\n            'width=626,height=436');\n    }\n\n    function showUrl() {\n        //console.log(\"//TODO: copy show this url to user\");\n        alert(\"copy this link to share -> \".concat(location.href.toString()));\n    }\n\n    function on_popstate() {\n        let newMainWord = \"\";\n        let newManualComparisons = [];\n        for (let url_component of window.location.hash.substr(1).split(\"&\")) {\n            let [key, value] = url_component.split(\"=\");\n            if (key === \"w\") {\n                newMainWord = decodeURIComponent(value);\n            } else if (key === \"o\" && value !== \"\") {\n                newManualComparisons = value.split(\"+\").map(decodeURIComponent);\n            }\n        }\n\n        updatePlot(newMainWord, newManualComparisons, true);\n    }\n\n    function wordChanged() {\n        // Wait for next turn in JS executor to let change take effect.\n        setTimeout(() => updatePlot(wordInput.value.trim(), null), 0);\n    }\n\n    function manualComparisonChanged(inputField, index) {\n        // Wait for next turn in JS executor to let change take effect.\n        setTimeout(() => {\n            let otherWord = inputField.value.trim();\n\n            // Make a *copy* of the array so that `updatePlot` can check if anything changed.\n            let newManualComparisons = [...manualComparisons];\n            if (index >= newManualComparisons.length - 1 && otherWord === '') {\n                // Last nonempty input box was emptied out. Remove the word. The input box\n                // will still stick around anyway.\n                newManualComparisons.splice(index, 1);\n            } else if (index < newManualComparisons.length) {\n                newManualComparisons[index] = otherWord;\n            } else {\n                newManualComparisons.push(otherWord);\n            }\n            updatePlot(null, newManualComparisons);\n            mainPlot.setMainLine(suggestedComparisonItems.length + index);\n        }, 0);\n    }\n\n    function removeManualComparison(index) {\n        // Make a *copy* of the array so that `updatePlot` can check if anything changed.\n        let newManualComparisons = [...manualComparisons];\n        if (index < newManualComparisons.length) {\n            newManualComparisons.splice(index, 1); // Removes the element.\n            updatePlot(null, newManualComparisons);\n        }\n    }\n\n    function updatePlot(newMainWord, newManualComparisons, suppress_save_state = false) {\n        let mainWordChanged = false;\n        let manualComparisonsChanged = false;\n\n        if (newMainWord !== null) {\n            if (wordInput.value.trim() !== newMainWord) {\n                wordInput.value = newMainWord;\n            }\n            let newMainWordId = inverseVocab[newMainWord];\n            if (newMainWord === '' || typeof newMainWordId !== 'undefined') {\n                wordInput.classList.remove('invalid');\n                if (newMainWord !== currentWord) {\n                    mainWordChanged = true;\n                    currentWord = newMainWord;\n                    suggestedComparisonIds = handle.largest_changes_wrt(newMainWordId, suggestedComparisonItems.length, 2, 2);\n                }\n            } else {\n                // Out of vocabulary word entered. Treat as if `currentWord` did not change. \n                // We may still want to update the plot in case `manualComparisons` changed.\n                wordInput.classList.add('invalid');\n            }\n        }\n\n        if (newManualComparisons !== null) {\n            let newManualComparisonIds = [];\n            if (newManualComparisons.length > manualComparisonItems.length) {\n                newManualComparisons.splice(manualComparisonItems.length); // Removes everything that flows over.\n            }\n\n            // Update input boxes in legend.\n            for (let i = 0; i < newManualComparisons.length; i += 1) {\n                let otherWord = newManualComparisons[i];\n                let otherWordId = inverseVocab[otherWord];\n                newManualComparisonIds.push(otherWordId);\n\n                if (i >= manualComparisons.length || manualComparisons[i] !== otherWord) {\n                    manualComparisonsChanged = true;\n                    if (typeof otherWordId === 'undefined') {\n                        manualComparisonInputs[i].classList.add('invalid');\n                    } else {\n                        manualComparisonInputs[i].classList.remove('invalid');\n                    }\n                    manualComparisonItems[i].style.display = 'list-item';\n                    manualComparisonRemoveButtons[i].style.display = 'inline';\n                    if (manualComparisonInputs[i].value.trim() !== otherWord) {\n                        manualComparisonInputs[i].value = otherWord;\n                    }\n                    inputWidthMeasure.textContent = otherWord;\n                    manualComparisonInputs[i].style.width = inputWidthMeasure.offsetWidth + 'px';\n                }\n            }\n            manualComparisonIds = newManualComparisonIds;\n\n            if (newManualComparisons.length !== manualComparisons.length) {\n                manualComparisonsChanged = true;\n\n                if (newManualComparisons.length < manualComparisonItems.length) {\n                    // There's still room for additional manual comparisons, so show an empty input box.\n                    manualComparisonItems[newManualComparisons.length].style.display = 'list-item';\n                    manualComparisonInputs[newManualComparisons.length].value = '';\n                    manualComparisonInputs[newManualComparisons.length].style.width = '0';\n                    manualComparisonInputs[newManualComparisons.length].classList.remove('invalid');\n                    manualComparisonRemoveButtons[newManualComparisons.length].style.display = 'none';\n\n                    // Remove all input boxes below.\n                    for (let i = newManualComparisons.length + 1; i < manualComparisonItems.length; i += 1) {\n                        manualComparisonItems[i].style.display = 'none';\n                    }\n                }\n            }\n\n            manualComparisons = newManualComparisons;\n            manualComparisonIds = newManualComparisonIds;\n        }\n\n        // Do the expensive stuff only if anything actually changed. This allows us to\n        // attach this function on lots of events to catch changes as early as possible\n        // without firing multiple times on the same change.\n        if (mainWordChanged || manualComparisonsChanged) {\n            mainPlot.clear();\n\n            if (currentWord === '') {\n                legend.style.display = 'none';\n                if (!suppress_save_state) {\n                    history.pushState(null, \"The Linguistic Time Capsule\", \"#\");\n                }\n                return;\n            }\n\n            if (!suppress_save_state) {\n                let stateUrl = \"#v=0&c=en&w=\" + encodeURIComponent(currentWord);\n                if (manualComparisons.length != 0) {\n                    stateUrl = stateUrl + \"&o=\" + manualComparisons.map(encodeURIComponent).join(\"+\");\n                }\n                history.pushState(null, \"The Linguistic Time Capsule: \" + currentWord, stateUrl);\n            }\n\n            legend.style.display = 'block';\n            allComparisonItems.forEach(el => {\n                el.classList.remove('hovering');\n                el.firstElementChild.textContent = currentWord;\n            });\n\n            let otherWordIds = [...suggestedComparisonIds];\n            let comparisonColors = [];\n            for (let i = 0; i < otherWordIds.length; i += 1) {\n                comparisonColors.push(i);\n            }\n            manualComparisonIds.forEach((id, index) => {\n                if (typeof id !== 'undefined') {\n                    otherWordIds.push(id)\n                    comparisonColors.push(suggestedComparisonIds.length + index);\n                }\n            });\n\n            let mainWordId = inverseVocab[currentWord];\n            let wordIdRepeated = Array(otherWordIds.length).fill(mainWordId);\n            let concatenatedTrajectories = handle.pairwise_trajectories(wordIdRepeated, otherWordIds);\n            let trajectoryLength = concatenatedTrajectories.length / otherWordIds.length;\n\n            otherWordIds.forEach((otherWordId, index) => {\n                let otherWord = metaData.vocab[otherWordId];\n                mainPlot.plotLine(\n                    concatenatedTrajectories.subarray(index * trajectoryLength, (index + 1) * trajectoryLength),\n                    comparisonColors[index],\n                    0,\n                    {\n                        word1: currentWord,\n                        word2: otherWord,\n                        word1Id: mainWordId,\n                        word2Id: otherWordId,\n                    },\n                    false,\n                    '\"' + currentWord + '\" ↔ \"' + otherWord + '\"\\n(click on line to explore relationship)'\n                );\n\n                if (index < suggestedComparisonItems.length) {\n                    allComparisonItems[index].firstElementChild.nextElementSibling.textContent = otherWord;\n                }\n            });\n        }\n    }\n}())\n\n\n//# sourceURL=webpack:///./src/index.js?");

/***/ }),

/***/ "./src/styles.css":
/*!************************!*\
  !*** ./src/styles.css ***!
  \************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var api = __webpack_require__(/*! ../node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js */ \"./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js\");\n            var content = __webpack_require__(/*! !../node_modules/css-loader/dist/cjs.js!./styles.css */ \"./node_modules/css-loader/dist/cjs.js!./src/styles.css\");\n\n            content = content.__esModule ? content.default : content;\n\n            if (typeof content === 'string') {\n              content = [[module.i, content, '']];\n            }\n\nvar options = {};\n\noptions.insert = \"head\";\noptions.singleton = false;\n\nvar update = api(module.i, content, options);\n\nvar exported = content.locals ? content.locals : {};\n\n\n\nmodule.exports = exported;\n\n//# sourceURL=webpack:///./src/styles.css?");

/***/ })

/******/ });