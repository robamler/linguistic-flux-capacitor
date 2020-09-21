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
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/word_history_explorer_backend_bg.wasm":"dca38eadc93ce9dadcac"}[wasmModuleId] + ".module.wasm");
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

eval("// Imports\nvar ___CSS_LOADER_API_IMPORT___ = __webpack_require__(/*! ../node_modules/css-loader/dist/runtime/api.js */ \"./node_modules/css-loader/dist/runtime/api.js\");\nexports = ___CSS_LOADER_API_IMPORT___(false);\n// Module\nexports.push([module.i, \"body {\\n    font-family: Arial, Helvetica, sans-serif;\\n    position: absolute;\\n    left: 0;\\n    right: 0;\\n    top: 0;\\n    bottom: 0;\\n}\\n\\n.pageContainer {\\n    width: 48em;\\n    position: absolute;\\n    left: 50%;\\n    transform: translateX(-70%);\\n}\\n\\nh1 {\\n    text-align: center;\\n    font-weight: normal;\\n    font-size: 350%;\\n    font-family: 'DM Serif Display', serif;\\n    margin: 0.3em 0 0.7em 0;\\n    padding: 0;\\n}\\n\\nh2 {\\n    font-weight: normal;\\n    text-align: center;\\n    font-size: 200%;\\n    margin: 0.4em 0\\n}\\n\\n.plotAndLegend {\\n    position: relative\\n}\\n\\n.plotContainer {\\n    position: absolute;\\n    left: -2em;\\n    width: 48em;\\n}\\n\\n.centered {\\n    text-align: center;\\n}\\n\\n.wordInput {\\n    margin: 0 0 1em 0;\\n    font-size: 200%;\\n    width: 40%;\\n    text-align: center;\\n    border: solid 0.05em #ccc;\\n    padding: 0.2em;\\n}\\n\\n.wordInput:focus {\\n    border-color: #86bdf0;\\n    outline: none;\\n}\\n\\n.wordInput.invalid {\\n    background-color: #fdebeb;\\n}\\n\\n.tooltipContent .explanation {\\n    font-size: 90%;\\n}\\n\\n.tooltipContent th {\\n    font-size: 120%;\\n}\\n\\n.tooltipContent th, .tooltipContent td {\\n    vertical-align: baseline;\\n}\\n\\n.tooltipContent th.left, .tooltipContent td.left {\\n    padding-right: 0.3em;\\n}\\n\\n.tooltipContent th.right, .tooltipContent td.right {\\n    padding-left: 0.3em;\\n}\\n\\n.tooltipContent .year {\\n    font-size: 140%;\\n    font-weight: bold;\\n}\\n\\n.tooltipContent .interleave {\\n    padding: 0 0 0.3em 0;\\n}\\n\\n.tooltipContent .hint {\\n    opacity: 0.7;\\n    font-size: 80%;\\n    margin-top: 0.1em;\\n}\\n\\n.tooltipContent table {\\n    width: 100%;\\n}\\n\\n.tooltipContent .wait {\\n    display: none;\\n    position: absolute;\\n    text-align: center;\\n    width: 10em;\\n    white-space: normal;\\n    left: 50%;\\n    top: 50%;\\n    transform: translate(-50%, -50%);\\n    opacity: 0.8;\\n    font-size: 80%;\\n}\\n\\n.tooltipContent.waiting .wait {\\n    display: block;\\n}\\n\\n.tooltipContent .suggestionsTable {\\n    visibility: visible;\\n}\\n\\n.tooltipContent.waiting .suggestionsTable {\\n    visibility: hidden;\\n}\\n\\n.tooltipContent .suggestion {\\n    padding: 0.05em 0;\\n    font-size: 80%;\\n}\\n\\n.tooltipContent .left {\\n    text-align: left;\\n}\\n\\n.tooltipContent .right {\\n    text-align: right;\\n}\\n\\n.tooltipContent a {\\n    text-decoration: none;\\n    color: #152d74;\\n}\\n\\n.legend {\\n    visibility: hidden;\\n    display: table-caption;\\n    background-color: #e8e8e8;\\n    border-radius: 0.5em;\\n    padding: 0.7em 0.9em;\\n    width: auto;\\n    position: absolute;\\n    left: 47em;\\n    top: 0;\\n    min-width: 10em;\\n}\\n\\n.legendText {\\n    color: #333333;\\n}\\n\\n.legend>ul {\\n    list-style-type: none;\\n    margin: 0 0 0.5em 0;\\n    padding: 0;\\n    font-size: 120%;\\n}\\n\\n.legend>ul>li {\\n    margin: -0.2em -0.4em 0.2em -0.4em;\\n    padding: 0.2em 0.4em 0.4em 0.4em;\\n    border-radius: 0.2em;\\n    white-space: nowrap;\\n    cursor: default;\\n}\\n\\n.legend>ul>li:hover, .legend>ul>li.hovering {\\n    background-color: #f8f8f8;\\n}\\n\\n.legend>ul>li>a {\\n    text-decoration: none;\\n    color: #152d74;\\n}\\n\\n.legend>ul>li::before {\\n    content: \\\"⸺\\\";\\n    margin-right: 0.5em;\\n    font-weight: bold;\\n}\\n\\n.legend>ul>li.color0::before {\\n    color: #f94a01;\\n}\\n\\n.legend>ul>li.color1::before {\\n    color: #6b42b6;\\n}\\n\\n.legend>ul>li.color2::before {\\n    color: #11a854;\\n}\\n\\n.legend>ul>li.color3::before {\\n    color: #128db2;\\n}\\n\\n.legend>ul>li.color4::before {\\n    color: #e12fbc;\\n}\\n\\n.legend>ul>li.color5::before {\\n    color: #e6ab02;\\n}\\n\\n.legend>ul>li.color6::before {\\n    color: #9ed034;\\n}\\n\\n.legend>ul>li.color7::before {\\n    color: #666666;\\n}\\n\", \"\"]);\n// Exports\nmodule.exports = exports;\n\n\n//# sourceURL=webpack:///./src/styles.css?./node_modules/css-loader/dist/cjs.js");

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
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _styles_css__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./styles.css */ \"./src/styles.css\");\n/* harmony import */ var _styles_css__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_styles_css__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var _assets_googlebooks_metadata_1800to2008_vocabsize30000_bin__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../assets/googlebooks_metadata_1800to2008_vocabsize30000.bin */ \"./assets/googlebooks_metadata_1800to2008_vocabsize30000.bin\");\n\n\n\n\n// Wasm modules must be imported asynchronously.\nlet backendPromise = __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./backend.js */ \"./src/backend.js\"));\n\n(async function () {\n    const Plotter = await __webpack_require__.e(/*! import() */ 1).then(__webpack_require__.bind(null, /*! ./plotting/main.mjs */ \"./src/plotting/main.mjs\"));\n    if (document.readyState === 'loading') {\n        await new Promise(function (resolve, _reject) {\n            window.addEventListener('DOMContentLoaded', resolve);\n        });\n    }\n\n    let years = [];\n    let pointsY1 = [];\n    let pointsY2 = [];\n    let ticksX = [];\n    for (let year = 1800; year <= 2008; year += 1) {\n        years.push(year);\n        if (year % 20 === 0) {\n            ticksX.push(year);\n        }\n\n        pointsY1.push(0.3 * Math.sin(0.1 * year));\n        pointsY2.push(0.2 * Math.sin(0.2 * year) + 0.002 * (year - 1900));\n    }\n    let currentWord = null;\n\n    const mainLegend = document.getElementById('mainLegend');\n    const mainLegendItems = mainLegend.querySelectorAll('li');\n\n    let updateTooltip = (function () {\n        let tooltip = document.getElementById('tooltipTemplate');\n        let tooltipContent = tooltip.querySelector('.tooltipContent');\n        let yearPlaceholder = tooltip.querySelector('.year');\n        let word1Placeholder = tooltip.querySelector('.word1');\n        let word2Placeholder = tooltip.querySelector('.word2>a');\n        let relatedPlaceholders = [];\n        let relatedTimeout = null;\n        let relatedCache = {};\n        tooltip.querySelectorAll('.suggestion.left>a').forEach(el => {\n            relatedPlaceholders.push(el);\n            el.addEventListener('click', ev => {\n                ev.preventDefault();\n                el.blur();\n                exploreWord(el.innerText);\n            });\n        });\n        tooltip.querySelectorAll('.suggestion.right>a').forEach(el => {\n            relatedPlaceholders.push(el);\n            el.addEventListener('click', ev => {\n                ev.preventDefault();\n                el.blur();\n                exploreWord(el.innerText);\n            });\n        });\n        word2Placeholder.addEventListener('click', ev => {\n            ev.preventDefault();\n            word2Placeholder.blur();\n            exploreWord(word2Placeholder.innerText);\n        });\n\n\n        return function (tooltip, line, indexX) {\n            clearTimeout(relatedTimeout);\n            let payload = line.payload;\n            yearPlaceholder.innerText = years[indexX];\n            word1Placeholder.innerText = payload.word1;\n            word2Placeholder.innerText = payload.word2;\n\n            // TODO: look up word1 and word2 in cache independently.\n            // TODO: clear old entries from cache at some point.\n            let cacheKey = payload.word1Id + '-' + payload.word2Id + '-' + indexX;\n            let cached = relatedCache[cacheKey];\n            if (typeof (cached) !== 'undefined') {\n                cached.forEach((r, i) => {\n                    relatedPlaceholders[i].innerText = metaData.vocab[r];\n                });\n                tooltipContent.classList.remove('waiting');\n            } else {\n                tooltipContent.classList.add('waiting');\n                relatedTimeout = setTimeout(() => {\n                    tooltipContent.classList.remove('waiting');\n                    let related = handle.most_related_to_at_t([payload.word1Id, payload.word2Id], indexX, 7);\n                    relatedCache[cacheKey] = related;\n                    related.forEach((r, i) => {\n                        relatedPlaceholders[i].innerText = metaData.vocab[r];\n                    });\n                }, 0);\n            }\n        };\n    }());\n\n    let lineMouseover = function (lineId) {\n        mainLegendItems[lineId].classList.add('hovering');\n    };\n\n    let lineMouseout = function (lineId) {\n        mainLegendItems[lineId].classList.remove('hovering');\n    };\n\n    const mainPlot = Plotter.createPlot(\n        document.getElementById('mainPlot'), years, ticksX, updateTooltip,\n        document.getElementById('tooltipTemplate'), lineMouseover, lineMouseout);\n\n    mainLegendItems.forEach((element, index) => {\n        element.addEventListener('mouseover', () => mainPlot.hoverLine(index));\n        element.addEventListener('mouseout', () => mainPlot.unhoverLine(index));\n\n        const legendLink = element.querySelector('a');\n        legendLink.addEventListener('click', ev => {\n            ev.preventDefault();\n            legendLink.blur();\n            exploreWord(legendLink.innerText);\n        });\n    });\n\n    let backend = await backendPromise;\n    let handle = await backend.loadFile();\n    let metaData = await (await fetch(_assets_googlebooks_metadata_1800to2008_vocabsize30000_bin__WEBPACK_IMPORTED_MODULE_1__[\"default\"])).json();\n    let inverseVocab = {};\n    metaData.vocab.forEach((word, index) => inverseVocab[word] = index);\n\n    let wordInput = document.querySelector('.wordInput');\n    wordInput.onkeydown = wordChanged;\n    wordInput.onkeypress = wordChanged;\n    wordInput.onchange = wordChanged;\n\n    wordChanged();\n    wordInput.focus();\n\n    function wordChanged() {\n        // Wait for next turn in JS executor to let change take effect.\n        setTimeout(() => exploreWord(wordInput.value), 0);\n    }\n\n    function exploreWord(word) {\n        if (word !== currentWord) {\n            currentWord = word;\n\n            mainLegendItems.forEach(el => el.classList.remove('hovering'));\n\n            let wordId = inverseVocab[word];\n            if (typeof wordId === 'undefined') {\n                wordInput.classList.add('invalid');\n            } else {\n                wordInput.classList.remove('invalid');\n                if (wordInput.value !== word) {\n                    wordInput.value = word;\n                }\n                mainPlot.clear();\n                let otherWords = handle.largest_changes_wrt(wordId, 6, 2, 2);\n                let wordIdRepeated = Array(6).fill(wordId);\n                let concatenatedTrajectories = handle.pairwise_trajectories(wordIdRepeated, otherWords);\n                let trajectoryLength = concatenatedTrajectories.length / 6;\n\n                otherWords.forEach((otherWordId, index) => {\n                    let otherWord = metaData.vocab[otherWordId];\n                    mainPlot.plotLine(\n                        concatenatedTrajectories.subarray(index * trajectoryLength, (index + 1) * trajectoryLength),\n                        index,\n                        0,\n                        {\n                            word1: word,\n                            word2: otherWord,\n                            word1Id: wordId,\n                            word2Id: otherWordId\n                        },\n                        false\n                    );\n\n                    const legendWordLabel = mainLegendItems[index].firstElementChild;\n                    legendWordLabel.textContent = word;\n                    legendWordLabel.nextElementSibling.textContent = otherWord;\n                });\n\n                mainLegend.style.visibility = 'visible';\n            }\n        }\n    }\n}())\n\n\n//# sourceURL=webpack:///./src/index.js?");

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