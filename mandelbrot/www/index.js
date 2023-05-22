import * as wasm from "mandelbrot_wasm"
import $ from "jquery";

function render() {
    const canvas = document.getElementById("mandelbrotCanvas");

    const ctx = canvas.getContext("2d");    
    canvas.width = $("#iwidth").val();
    canvas.height = $("#iwidth").val();
    wasm.render($("#iwidth").val(), $("#iiter").val(), $("#iscale").val(), $("#icenterx").val(), $("#icentery").val(), ctx);

  }
  
  $('#renderBtn').on('click', function () {
    render();
  });
