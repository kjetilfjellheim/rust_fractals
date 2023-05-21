import * as wasm from "mandelbrot_wasm"
import $ from "jquery";

function render() {
    const canvas = document.getElementById("mandelbrotCanvas");

    const ctx = canvas.getContext("2d");    
  
    wasm.render($("#iwidth").val(), $("#iiter").val(), ctx, true);

  }
  
  $('#renderBtn').on('click', function () {
    render();
  });
