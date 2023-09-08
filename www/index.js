// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "../pkg/BacteriaGraphing.js";



const canvas = document.getElementById("canvas");



function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
  const aspectRatio = canvas.width / canvas.height;
  const size = canvas.parentNode.offsetWidth * 0.8;
  canvas.style.width = size + "px";
  canvas.style.height = size / aspectRatio + "px";
  canvas.width = size;
  canvas.height = size / aspectRatio;
  
  updatePlot();
}


const runWasm = async () => {
  // Instantiate our wasm module
  const bacteriaGraphing = await init("../pkg/BacteriaGraphing_bg.wasm");

  // Call the Add function export from wasm, save the result
  const addResult = bacteriaGraphing.add(24, 24);

  // Set the result onto the body
  document.body.textContent = `Hello World! addResult: ${addResult}`;
};

const updatePlot = async () => {
  const selected = plotType.selectedOptions[0];
  status.innerText = `Rendering ${selected.innerText}...`;
  chart = null;
  const start = performance.now();

  
  const bacteriaGraphing = await init("../pkg/BacteriaGraphing_bg.wasm");


  control.classList.add("hide");
  chart = bacteriaGraphing.power("canvas", Number(selected.value))


  const end = performance.now();
  status.innerText = `Rendered ${selected.innerText} in ${Math.ceil(end - start)}ms`;
}




setupCanvas();

updatePlot();

//runWasm();