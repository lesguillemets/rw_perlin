import init, * as wasm from './pkg/rw_perlin.js';
async function run() {
    await init();
    let canvas = document.getElementById('world');
    let ctx = canvas.getContext('2d')
    wasm.draw(ctx, canvas.width, canvas.height);
}

run();
