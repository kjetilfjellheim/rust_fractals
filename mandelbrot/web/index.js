import * as wasm from "mandelbrot_wasm"

const canvas = document.getElementById("mandelbrotCanvas");

const ctx = canvas.getContext("2d");

wasm.render(4000, 4000, ctx);
