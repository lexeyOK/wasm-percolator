import init, { render } from "./pkg/bare_metal_wasm.js";
async function run() {
	await init();
	const width = 600;
	const height = 600;

	const canvas = document.getElementById("canvas");
	const seed_input = document.getElementById("seed");
	const use_seed_check = document.getElementById("use_seed");
	canvas.width = width;
	canvas.height = height;
	const ctx = canvas.getContext("2d");
	document.getElementById("form").addEventListener('submit', (event) => {
		event.preventDefault();
		let seed = 1;
		if (use_seed_check.checked) {
			seed = Number(seed_input.value);
		}
		else{
			seed = window.crypto.getRandomValues(new Uint32Array(1))[0];
		}
		const image = new ImageData(
			new Uint8ClampedArray(render(seed).buffer),
			width,
		);
		ctx.putImageData(image, 0, 0);
		seed_input.value = seed;
	})
}

run();