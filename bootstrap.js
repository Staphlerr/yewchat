// bootstrap.js
import init, { run_app } from "./pkg/yewchat.js";

async function main() {
    // load & instantiate the wasm module…
    await init();
    // …then call your Rust-exported entry point
    run_app();
}

main();
