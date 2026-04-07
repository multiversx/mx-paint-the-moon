// If you only use `npm` you can simply
// import { Chart } from "wasm-demo" and remove `setup` call from `bootstrap.js`.
class Chart {}

import { renderSphere, generateChecker, loadImage } from "./moon-js.js";

// Cached JS-side sources (avoids re-decoding on every render).
const jsCheckerSource = generateChecker(1000, 500);
let jsMoonSource = null;
const jsMoonSourceReady = loadImage("lroc_color_poles_1k.jpg").then(src => { jsMoonSource = src; });

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
const plotType = document.getElementById("plot-type");
const pitch = document.getElementById("pitch");
const control = document.getElementById("3d-control");
const status = document.getElementById("status");

let chart = null;

/** Main entry point */
export function main() {
	let hash = location.hash.substr(1);
	for(var i = 0; i < plotType.options.length; i++) {
		if(hash == plotType.options[i].value) {
			plotType.value = hash;
		}
	}
    setupUI();
    setupCanvas();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
    Chart = WasmChart;
}

/** Add event listeners. */
function setupUI() {
    status.innerText = "WebAssembly loaded!";
    plotType.addEventListener("change", updatePlot);
	pitch.addEventListener("change", updatePlot);
	pitch.addEventListener("input", scheduleUpdatePlot);
    window.addEventListener("resize", setupCanvas);
    window.addEventListener("mousemove", onMouseMove);
}

let _rafId = null;
function scheduleUpdatePlot() {
    if (_rafId !== null) return;
    _rafId = requestAnimationFrame(() => {
        _rafId = null;
        updatePlot();
    });
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
    const aspectRatio = canvas.width / canvas.height;
    const size = canvas.parentNode.offsetWidth * 0.5;
    canvas.style.width = size + "px";
    canvas.style.height = size / aspectRatio + "px";
    canvas.width = size;
    canvas.height = size / aspectRatio;
    updatePlot();
}

/** Update displayed coordinates. */
function onMouseMove(event) {
    if (chart) {
		var text = "Mouse pointer is out of range";

		if(event.target == canvas) {
			let actualRect = canvas.getBoundingClientRect();
			let logicX = event.offsetX * canvas.width / actualRect.width;
			let logicY = event.offsetY * canvas.height / actualRect.height;
			const point = chart.coord(logicX, logicY);
			text = (point) 
				? `(${point.x.toFixed(3)}, ${point.y.toFixed(3)})`
				: text;
		}
        coord.innerText = text;
    }
}

function updateMoon3d() {
	let pitch_value = Number(pitch.value) / 100.0;
	Chart.moon3d(canvas, pitch_value);
	coord.innerText = `Pitch:${pitch_value}`;
}

function updateChecker3d() {
	let pitch_value = Number(pitch.value) / 100.0;
	Chart.checker3d(canvas, pitch_value);
	coord.innerText = `Pitch:${pitch_value}`;
}

async function updateMoonJs() {
	if (!jsMoonSource) await jsMoonSourceReady;
	const pitch_value = Number(pitch.value) / 100.0;
	const size = Math.min(canvas.width, canvas.height);
	const buf = renderSphere(size, -pitch_value, jsMoonSource);
	canvas.getContext("2d").putImageData(new ImageData(buf, size, size), 0, 0);
	coord.innerText = `Pitch:${pitch_value}`;
}

function updateCheckerJs() {
	const pitch_value = Number(pitch.value) / 100.0;
	const size = Math.min(canvas.width, canvas.height);
	const buf = renderSphere(size, -pitch_value, jsCheckerSource);
	canvas.getContext("2d").putImageData(new ImageData(buf, size, size), 0, 0);
	coord.innerText = `Pitch:${pitch_value}`;
}

/** Redraw currently selected plot. */
async function updatePlot() {
    const selected = plotType.selectedOptions[0];
    status.innerText = `Rendering ${selected.innerText}...`;
    chart = null;
    const start = performance.now();
	switch(selected.value) {
		case "moon":
			updateMoon3d();
			break;
		case "checker":
			updateChecker3d();
			break;
		case "moon-js":
			await updateMoonJs();
			break;
		case "checker-js":
			updateCheckerJs();
			break;
	}
    const end = performance.now();
    status.innerText = `Rendered ${selected.innerText} in ${Math.ceil(end - start)}ms`;
}
