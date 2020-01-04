import * as wasm from "word-history-explorer-backend";
import { memory } from "word-history-explorer-backend/word_history_explorer_backend_bg";

console.log(gzFile);
import gzFile from "../assets/enwiki-4mb.txt.gz";

wasm.set_panic_hook();

// run();

async function run() {
    const buf = wasm.GzCompressedBuffer.new();
    const bufPtr = buf.get_mut_ptr();
    const capacity = buf.capacity();
    let bufArray = new Uint8Array(memory.buffer, bufPtr, capacity);
    let writeHead = 0;

    let response = await fetch(gzFile);
    let total_size = response.headers.get("content-length");
    let total_read = 0;
    let reader = response.body.getReader();

    while (true) {
        let { value, done } = await reader.read();
        if (done) {
            break;
        }

        console.log('NEW CHUNK OF SIZE ' + value.length);

        let readHead = 0;
        while (readHead < value.length) {
            if (bufArray.length === 0) {
                // `bufArray` got detached because a preceding call
                // to `buf.avail()` grew the wasm memory.
                bufArray = new Uint8Array(memory.buffer, bufPtr, capacity);
            }

            let numBytes = Math.min(value.length - readHead, capacity - writeHead);
            bufArray.set(value.subarray(readHead, readHead + numBytes), writeHead);
            writeHead = buf.avail(numBytes);
            readHead += numBytes;
            total_read += numBytes;
            console.log('Wrote ' + numBytes + ' (' + total_read + ' of ' + total_size + ')');
        }
    }

    console.log('Done writing.');
    console.log(buf.finish_and_peek());
}
