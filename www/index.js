import init, { Frame } from "raytracerwasm";

// const width = window.innerWidth;
// const height = window.innerHeight;
const width = 900;
const height = 550;
const canvas = document.getElementById("raytracerwasm-canvas");
canvas.height = height;
canvas.width = width;
const ctx = canvas.getContext('2d');

window.onload = async () => {
    const { memory } = await init();
    render(memory);
}

function render(memory) {
    const frame = Frame.create(width,height)
    const dataPtr = frame.data();
    const arr = new Uint8Array(memory.buffer, dataPtr, width * 3 * height);
    console.log(arr.length)
    for(let row = 0; row < height; row++) {
        for(let col = 0; col < width; col++) {
            const i = getIndex(row, col);
            const r = arr[i]
            const g = arr[i+1]
            const b = arr[i + 2]
            ctx.fillStyle = "rgba("+r+","+g+","+b+","+1+")";
            ctx.fillRect(col, row, 1, 1 );
        } 
    }
}

function getIndex(row, col) {
    return row * width * 3 + col * 3;
}



const renderLoop = () => {  
    requestAnimationFrame(renderLoop);
  };