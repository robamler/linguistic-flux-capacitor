import * as wasm from "word-history-explorer-backend";
import { memory } from "word-history-explorer-backend/word_history_explorer_backend_bg";

// import file from "../assets/random_6_100_16.dwe";
import dweFile from "../assets/step34000_T209_V30000_K100.dwe";

wasm.set_panic_hook();

export async function loadFile() {
    const builder = wasm.EmbeddingFileBuilder.new();

    let response = await fetch(dweFile);
    let fileSizeStr = response.headers.get('content-length');
    let pointerAndLen = undefined;
    let totalWritten = 0;
    let reader = response.body.getReader();

    let progressBar = document.getElementById('downloadProgressBar');
    let progressText = document.getElementById('downloadProgressText');

    while (typeof pointerAndLen === 'undefined') {
        let { value, done } = await reader.read();
        if (done) {
            throw "Exited before header was read";
        }

        if (value.length !== 0) {
            let ptr = builder.reserve(value.length);
            let targetArray = new Uint8Array(memory.buffer, ptr, totalWritten + value.length);
            targetArray.set(value, totalWritten);
            totalWritten += value.length;
            pointerAndLen = builder.avail(value.length);
        }
    }

    if (!!fileSizeStr && (fileSizeStr != pointerAndLen.len)) { // Yes, we want != and not !== here.
        throw "File size in HTTP header does not match file size in file header.";
    }

    if (totalWritten > pointerAndLen.len) {
        throw "File larger than expected.";
    }

    let targetArray = new Uint8Array(memory.buffer, pointerAndLen.pointer, pointerAndLen.len);

    while (true) {
        let progressPercent = 100 * totalWritten / pointerAndLen.len;
        progressBar.style.width = progressPercent + "%";
        progressText.innerText = Math.floor(progressPercent) + " %";

        let { value, done } = await reader.read();
        if (done) {
            break;
        }

        if (totalWritten + value.length > pointerAndLen.len) {
            throw "File larger than expected.";
        }

        if (targetArray.length === 0) {
            // `targetArray` got detached because the wasm memory grew for some reason, 
            // so we have to reattach it (`pointerAndLen.pointer` is still valid, though).
            targetArray = new Uint8Array(memory.buffer, pointerAndLen.pointer, pointerAndLen.len);
        }

        targetArray.set(value, totalWritten);
        totalWritten += value.length;
    }

    if (totalWritten != pointerAndLen.len) {
        throw "File smaller than expected.";
    }

    let handle = builder.finish();

    return handle;
}
