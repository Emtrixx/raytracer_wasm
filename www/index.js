// www/index.js
import init, { Frame } from "raytracerwasm";

const canvas = document.getElementById("raytracerwasm-canvas");
const ctx = canvas.getContext('2d');
const form = document.querySelector("form");
const widthInput = document.getElementById("width");
const heightInput = document.getElementById("height");
const brightnessInput = document.getElementById("brightness");
const brightnessValueSpan = document.getElementById("brightness-value");

let memory;
let spherePos = { x: 0, y: 0, z: 0 };
const moveStep = 0.5;
const brightnessStep = 0.1;

function updateBrightnessUI(value) {
    const val = Math.max(0, Math.min(4, parseFloat(value))).toFixed(1);
    brightnessInput.value = val;
    brightnessValueSpan.textContent = val;
}

window.onload = async () => {
    try {
        const wasm = await init();
        memory = wasm.memory;
        render();
    } catch (e) {
        console.error("Error initializing wasm module", e);
    }
};

form.addEventListener("submit", event => {
    event.preventDefault();
    render();
});

// Re-render when form inputs change
widthInput.addEventListener("input", () => render());
heightInput.addEventListener("input", () => render());
brightnessInput.addEventListener("input", () => {
    updateBrightnessUI(brightnessInput.value);
    render();
});

window.addEventListener("keydown", (event) => {
    let needsRender = false;
    switch (event.key.toLowerCase()) {
        case 'w': // forward
            spherePos.z -= moveStep;
            needsRender = true;
            break;
        case 's': // backward
            spherePos.z += moveStep;
            needsRender = true;
            break;
        case 'a': // left
            spherePos.x -= moveStep;
            needsRender = true;
            break;
        case 'd': // right
            spherePos.x += moveStep;
            needsRender = true;
            break;
        case 'q': // up
            spherePos.y += moveStep;
            needsRender = true;
            break;
        case 'e': // down
            spherePos.y -= moveStep;
            needsRender = true;
            break;
        case 'x': // brightness up
            updateBrightnessUI(parseFloat(brightnessInput.value) + brightnessStep);
            needsRender = true;
            break;
        case 'z': // brightness down
            updateBrightnessUI(parseFloat(brightnessInput.value) - brightnessStep);
            needsRender = true;
            break;
    }

    if (needsRender) {
        render();
    }
});

function render() {
    // Add a small delay to debounce rapid input changes
    clearTimeout(render.timeout);
    render.timeout = setTimeout(() => {
        const width = parseInt(widthInput.value);
        const height = parseInt(heightInput.value);
        const brightness = parseFloat(brightnessInput.value);

        if (isNaN(width) || isNaN(height) || width <= 0 || height <= 0) {
            return; // Avoid rendering with invalid dimensions
        }

        canvas.width = width;
        canvas.height = height;

        const frame = Frame.create(width, height, brightness, spherePos.x, spherePos.y, spherePos.z);
        const dataPtr = frame.data();
        // The image format from Rust is RGB, but ImageData expects RGBA.
        const pixelCount = width * height;
        const input = new Uint8Array(memory.buffer, dataPtr, pixelCount * 3);
        const output = new Uint8ClampedArray(pixelCount * 4);

        for (let i = 0; i < pixelCount; i++) {
            output[i * 4 + 0] = input[i * 3 + 0]; // R
            output[i * 4 + 1] = input[i * 3 + 1]; // G
            output[i * 4 + 2] = input[i * 3 + 2]; // B
            output[i * 4 + 3] = 255;              // A
        }

        const imageData = ctx.createImageData(width, height);
        imageData.data.set(output);
        ctx.putImageData(imageData, 0, 0);
    }, 16); // roughly 60fps debounce
}