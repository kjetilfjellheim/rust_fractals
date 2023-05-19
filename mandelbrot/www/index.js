import * as wasm from "mandelbrot_wasm"
import $ from "jquery";

function render() {
    const canvas = document.getElementById("mandelbrotCanvas");
  
    const ctx = canvas.getContext("2d");    
    
    wasm.render($("#iwidth").val(), $("#iheight").val(), $("#iiter").val(), ctx);
  }
  
  $('#renderBtn').on('click', function () {
    render();
  });
