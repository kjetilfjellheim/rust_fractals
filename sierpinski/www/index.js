import * as wasm from "mandelbrot_wasm"
import $ from "jquery";

function render() {
    const canvas = document.getElementById("sierpinskiCanvas");
    canvas.width = $("#iwidth").val();
    canvas.height = $("#iwidth").val();
    const ctx = canvas.getContext("2d"); 
    ctx.fillStyle = "blue";

    wasm.render($("#iwidth").val(), $("#idepth").val(), ctx);

  }
  
  $('#renderBtn').on('click', function () {
    render();
  });
