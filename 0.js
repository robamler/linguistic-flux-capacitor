(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/linguistic_flux_capacitor_backend.js":
/*!***************************************************!*\
  !*** ../pkg/linguistic_flux_capacitor_backend.js ***!
  \***************************************************/
/*! exports provided: set_panic_hook, EmbeddingFileBuilder, EmbeddingHandle, PointerAndLen, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./linguistic_flux_capacitor_backend_bg.wasm */ \"../pkg/linguistic_flux_capacitor_backend_bg.wasm\");\n/* harmony import */ var _linguistic_flux_capacitor_backend_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./linguistic_flux_capacitor_backend_bg.js */ \"../pkg/linguistic_flux_capacitor_backend_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"set_panic_hook\", function() { return _linguistic_flux_capacitor_backend_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"set_panic_hook\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"EmbeddingFileBuilder\", function() { return _linguistic_flux_capacitor_backend_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"EmbeddingFileBuilder\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"EmbeddingHandle\", function() { return _linguistic_flux_capacitor_backend_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"EmbeddingHandle\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"PointerAndLen\", function() { return _linguistic_flux_capacitor_backend_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"PointerAndLen\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _linguistic_flux_capacitor_backend_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/linguistic_flux_capacitor_backend.js?");

/***/ }),

/***/ "../pkg/linguistic_flux_capacitor_backend_bg.js":
/*!******************************************************!*\
  !*** ../pkg/linguistic_flux_capacitor_backend_bg.js ***!
  \******************************************************/
/*! exports provided: set_panic_hook, EmbeddingFileBuilder, EmbeddingHandle, PointerAndLen, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"set_panic_hook\", function() { return set_panic_hook; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"EmbeddingFileBuilder\", function() { return EmbeddingFileBuilder; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"EmbeddingHandle\", function() { return EmbeddingHandle; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"PointerAndLen\", function() { return PointerAndLen; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./linguistic_flux_capacitor_backend_bg.wasm */ \"../pkg/linguistic_flux_capacitor_backend_bg.wasm\");\n\n\nlet cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet cachegetUint32Memory0 = null;\nfunction getUint32Memory0() {\n    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory0 = new Uint32Array(_linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory0;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArray32ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 4);\n    getUint32Memory0().set(arg, ptr / 4);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nlet cachegetFloat32Memory0 = null;\nfunction getFloat32Memory0() {\n    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetFloat32Memory0 = new Float32Array(_linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetFloat32Memory0;\n}\n\nfunction getArrayF32FromWasm0(ptr, len) {\n    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);\n}\n\nfunction getArrayU32FromWasm0(ptr, len) {\n    return getUint32Memory0().subarray(ptr / 4, ptr / 4 + len);\n}\n/**\n*/\nfunction set_panic_hook() {\n    _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"set_panic_hook\"]();\n}\n\n/**\n*/\nclass EmbeddingFileBuilder {\n\n    static __wrap(ptr) {\n        const obj = Object.create(EmbeddingFileBuilder.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_embeddingfilebuilder_free\"](ptr);\n    }\n    /**\n    * @returns {EmbeddingFileBuilder}\n    */\n    static new() {\n        var ret = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"embeddingfilebuilder_new\"]();\n        return EmbeddingFileBuilder.__wrap(ret);\n    }\n    /**\n    * Reserves space to write at least `additional_bytes` more bytes.\n    *\n    * # Returns\n    *\n    * A pointer to the *start* of the buffer (which may have changed since the\n    * buffer may have been reallocated at a new position in memory.\n    *\n    * # Panics\n    *\n    * If `additional_bytes` is zero and no bytes have yet been written to the\n    * builder.\n    * @param {number} additional_bytes\n    * @returns {number}\n    */\n    reserve(additional_bytes) {\n        var ret = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"embeddingfilebuilder_reserve\"](this.ptr, additional_bytes);\n        return ret;\n    }\n    /**\n    * Tells the buffer that `amt` new bytes have been written in a contiguous\n    * sequence and are available for consumption.\n    *\n    * # Returns\n    *\n    * `Some(pointer_and_len)` if the written data completed the file header. In\n    * this case, the buffer has been resized to the exact file size, which is\n    * returned as `pointer_and_len.len`. Further, `pointer_and_len.pointer` will\n    * point to the new start of the allocated buffer.\n    *\n    * The builder will report a `Some` value only once. After it reported a `Some`\n    * value, `reserve` should not be called any more  and, in total, exactly\n    * `file_size` bytes have to be written to the builder (including the ones\n    * already written).\n    *\n    * # Safety\n    *\n    * The builder trusts the caller that it really has initialized `amt`\n    * additional bytes before this method is called.\n    * @param {number} amt\n    * @returns {PointerAndLen | undefined}\n    */\n    avail(amt) {\n        var ret = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"embeddingfilebuilder_avail\"](this.ptr, amt);\n        return ret === 0 ? undefined : PointerAndLen.__wrap(ret);\n    }\n    /**\n    * Parse the fully filled buffer as an `EmbeddingFile`\n    *\n    * # Safety\n    *\n    * Before calling this method, the caller must have:\n    * * called `reserve` with a nonzero value, then written the announced number\n    *   of bytes and called `avail` with the same number;\n    * * repeated the last step until `avail` returned a `Some`, enclosing the file\n    *   size read out of the file header; then\n    * * filled in the rest of the buffer with exactly the right amount of bytes,\n    *   without any more calls to `reserve` or `avail`.\n    * * filled some bytes into the buffer and called avail `avail`, and repeated\n    *   this process until `avail` returned a `Some` variant; then\n    * * filled exactly as many bytes as returned inside the `Some` from the last\n    *   call to `avail`.\n    *\n    * After calling this method, the caller may no longer write to the buffer.\n    * @returns {EmbeddingHandle}\n    */\n    finish() {\n        var ptr = this.ptr;\n        this.ptr = 0;\n        var ret = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"embeddingfilebuilder_finish\"](ptr);\n        return EmbeddingHandle.__wrap(ret);\n    }\n}\n/**\n*/\nclass EmbeddingHandle {\n\n    static __wrap(ptr) {\n        const obj = Object.create(EmbeddingHandle.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_embeddinghandle_free\"](ptr);\n    }\n    /**\n    * @param {Uint32Array} words1\n    * @param {Uint32Array} words2\n    * @returns {Float32Array}\n    */\n    pairwise_trajectories(words1, words2) {\n        try {\n            const retptr = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n            var ptr0 = passArray32ToWasm0(words1, _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n            var len0 = WASM_VECTOR_LEN;\n            var ptr1 = passArray32ToWasm0(words2, _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n            var len1 = WASM_VECTOR_LEN;\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"embeddinghandle_pairwise_trajectories\"](retptr, this.ptr, ptr0, len0, ptr1, len1);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v2 = getArrayF32FromWasm0(r0, r1).slice();\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 4);\n            return v2;\n        } finally {\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n        }\n    }\n    /**\n    * @param {Uint32Array} words\n    * @param {number} t\n    * @param {number} amt\n    * @returns {Uint32Array}\n    */\n    most_related_to_at_t(words, t, amt) {\n        try {\n            const retptr = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n            var ptr0 = passArray32ToWasm0(words, _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n            var len0 = WASM_VECTOR_LEN;\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"embeddinghandle_most_related_to_at_t\"](retptr, this.ptr, ptr0, len0, t, amt);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v1 = getArrayU32FromWasm0(r0, r1).slice();\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 4);\n            return v1;\n        } finally {\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n        }\n    }\n    /**\n    * @param {number} target_word\n    * @param {number} amt\n    * @param {number} min_increasing\n    * @param {number} min_decreasing\n    * @returns {Uint32Array}\n    */\n    largest_changes_wrt(target_word, amt, min_increasing, min_decreasing) {\n        try {\n            const retptr = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"embeddinghandle_largest_changes_wrt\"](retptr, this.ptr, target_word, amt, min_increasing, min_decreasing);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v0 = getArrayU32FromWasm0(r0, r1).slice();\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 4);\n            return v0;\n        } finally {\n            _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n        }\n    }\n}\n/**\n*/\nclass PointerAndLen {\n\n    static __wrap(ptr) {\n        const obj = Object.create(PointerAndLen.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_pointerandlen_free\"](ptr);\n    }\n    /**\n    * @returns {number}\n    */\n    get pointer() {\n        var ret = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_pointerandlen_pointer\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set pointer(arg0) {\n        _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_pointerandlen_pointer\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {number}\n    */\n    get len() {\n        var ret = _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_pointerandlen_len\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set len(arg0) {\n        _linguistic_flux_capacitor_backend_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_pointerandlen_len\"](this.ptr, arg0);\n    }\n}\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack:///../pkg/linguistic_flux_capacitor_backend_bg.js?");

/***/ }),

/***/ "../pkg/linguistic_flux_capacitor_backend_bg.wasm":
/*!********************************************************!*\
  !*** ../pkg/linguistic_flux_capacitor_backend_bg.wasm ***!
  \********************************************************/
/*! exports provided: memory, __wbg_embeddingfilebuilder_free, embeddingfilebuilder_new, embeddingfilebuilder_reserve, embeddingfilebuilder_avail, embeddingfilebuilder_finish, __wbg_pointerandlen_free, __wbg_get_pointerandlen_pointer, __wbg_set_pointerandlen_pointer, __wbg_get_pointerandlen_len, __wbg_set_pointerandlen_len, __wbg_embeddinghandle_free, embeddinghandle_pairwise_trajectories, embeddinghandle_most_related_to_at_t, embeddinghandle_largest_changes_wrt, set_panic_hook, __wbindgen_export_0, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./linguistic_flux_capacitor_backend_bg.js */ \"../pkg/linguistic_flux_capacitor_backend_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/linguistic_flux_capacitor_backend_bg.wasm?");

/***/ }),

/***/ "./assets/step100000_T209_V30000_K100_q32.dwe":
/*!****************************************************!*\
  !*** ./assets/step100000_T209_V30000_K100_q32.dwe ***!
  \****************************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (__webpack_require__.p + \"4e007933e0582bf86c0ca0a41eb8b99f.dwe\");\n\n//# sourceURL=webpack:///./assets/step100000_T209_V30000_K100_q32.dwe?");

/***/ }),

/***/ "./src/backend.js":
/*!************************!*\
  !*** ./src/backend.js ***!
  \************************/
/*! exports provided: loadFile */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"loadFile\", function() { return loadFile; });\n/* harmony import */ var linguistic_flux_capacitor_backend__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! linguistic-flux-capacitor-backend */ \"../pkg/linguistic_flux_capacitor_backend.js\");\n/* harmony import */ var linguistic_flux_capacitor_backend_linguistic_flux_capacitor_backend_bg__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! linguistic-flux-capacitor-backend/linguistic_flux_capacitor_backend_bg */ \"../pkg/linguistic_flux_capacitor_backend_bg.wasm\");\n/* harmony import */ var _assets_step100000_T209_V30000_K100_q32_dwe__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../assets/step100000_T209_V30000_K100_q32.dwe */ \"./assets/step100000_T209_V30000_K100_q32.dwe\");\n\n\n\n\n\nlinguistic_flux_capacitor_backend__WEBPACK_IMPORTED_MODULE_0__[\"set_panic_hook\"]();\n\nasync function loadFile() {\n    const builder = linguistic_flux_capacitor_backend__WEBPACK_IMPORTED_MODULE_0__[\"EmbeddingFileBuilder\"].new();\n\n    let response = await fetch(_assets_step100000_T209_V30000_K100_q32_dwe__WEBPACK_IMPORTED_MODULE_2__[\"default\"]);\n    let fileSizeStr = response.headers.get('content-length');\n    let pointerAndLen = undefined;\n    let totalWritten = 0;\n    let reader = response.body.getReader();\n\n    let progressBar = document.getElementById('downloadProgressBar');\n    let progressText = document.getElementById('downloadProgressText');\n\n    while (typeof pointerAndLen === 'undefined') {\n        let { value, done } = await reader.read();\n        if (done) {\n            throw \"Exited before header was read\";\n        }\n\n        if (value.length !== 0) {\n            let ptr = builder.reserve(value.length);\n            let targetArray = new Uint8Array(linguistic_flux_capacitor_backend_linguistic_flux_capacitor_backend_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer, ptr, totalWritten + value.length);\n            targetArray.set(value, totalWritten);\n            totalWritten += value.length;\n            pointerAndLen = builder.avail(value.length);\n        }\n    }\n\n    if (!!fileSizeStr && (fileSizeStr != pointerAndLen.len)) { // Yes, we want != and not !== here.\n        throw \"File size in HTTP header does not match file size in file header.\";\n    }\n\n    if (totalWritten > pointerAndLen.len) {\n        throw \"File larger than expected.\";\n    }\n\n    let targetArray = new Uint8Array(linguistic_flux_capacitor_backend_linguistic_flux_capacitor_backend_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer, pointerAndLen.pointer, pointerAndLen.len);\n\n    while (true) {\n        let progressPercent = 100 * totalWritten / pointerAndLen.len;\n        progressBar.style.width = progressPercent + \"%\";\n        progressText.innerText = Math.floor(progressPercent) + \" %\";\n\n        let { value, done } = await reader.read();\n        if (done) {\n            break;\n        }\n\n        if (totalWritten + value.length > pointerAndLen.len) {\n            throw \"File larger than expected.\";\n        }\n\n        if (targetArray.length === 0) {\n            // `targetArray` got detached because the wasm memory grew for some reason, \n            // so we have to reattach it (`pointerAndLen.pointer` is still valid, though).\n            targetArray = new Uint8Array(linguistic_flux_capacitor_backend_linguistic_flux_capacitor_backend_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer, pointerAndLen.pointer, pointerAndLen.len);\n        }\n\n        targetArray.set(value, totalWritten);\n        totalWritten += value.length;\n    }\n\n    if (totalWritten != pointerAndLen.len) {\n        throw \"File smaller than expected.\";\n    }\n\n    let handle = builder.finish();\n\n    return handle;\n}\n\n\n//# sourceURL=webpack:///./src/backend.js?");

/***/ })

}]);