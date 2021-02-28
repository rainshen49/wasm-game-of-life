import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm"
import { Universe } from "wasm-game-of-life"

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("game-of-life-canvas")
/** @type {HTMLButtonElement} */
const pauseButton = document.getElementById("play-pause")
const universe = Universe.new()

let isPaused = false

const SCALE = Math.floor(600 / universe.width)
canvas.width = universe.width * SCALE
canvas.height = universe.height * SCALE

pauseButton.onclick = () => {
    isPaused = !isPaused
    if (!isPaused)
        // kick start the animation again
        renderLoop()
}

// canvas toggle handling
canvas.onclick = (event) => {
    const boundingRect = canvas.getBoundingClientRect()

    const scaleX = canvas.width / boundingRect.width
    const scaleY = canvas.height / boundingRect.height

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX
    const canvasTop = (event.clientY - boundingRect.top) * scaleY

    const row = Math.min(Math.floor(canvasTop / SCALE), canvas.height - 1)
    const col = Math.min(Math.floor(canvasLeft / SCALE), canvas.width - 1)

    universe.toggle_cell(row, col)

    renderUniverse()
}

/** @type {CanvasRenderingContext2D} */
const context = canvas.getContext('2d')
context.scale(SCALE, SCALE)
context.imageSmoothingEnabled = false

const imgArray = new Uint8ClampedArray(universe.width * universe.height * 4).fill(255)
const renderUniverse = async () => {
    const data = new Uint8Array(memory.buffer, universe.render_canvas(), universe.width * universe.height)
    data.forEach((cell, i) => {
        const val = (1 - cell) * 255
        imgArray[4 * i] = val
        imgArray[4 * i + 1] = val
        imgArray[4 * i + 2] = val
    })
    const image = await createImageBitmap(new ImageData(imgArray, universe.width, universe.height))
    context.drawImage(image, 0, 0)
}

const renderLoop = async () => {
    if (isPaused) return

    universe.tick()
    await renderUniverse()

    fps.render()

    if (!isPaused)
        requestAnimationFrame(renderLoop)
}
requestAnimationFrame(renderLoop)

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps")
        this.frames = []
        this.lastFrameTimeStamp = performance.now()
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now()
        const delta = now - this.lastFrameTimeStamp
        this.lastFrameTimeStamp = now
        const fps = 1 / delta * 1000

        // Save only the latest 100 timings.
        this.frames.push(fps)
        if (this.frames.length > 100) {
            this.frames.shift()
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity
        let max = -Infinity
        let sum = 0
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i]
            min = Math.min(this.frames[i], min)
            max = Math.max(this.frames[i], max)
        }
        let mean = sum / this.frames.length

        // Render the statistics.
        this.fps.textContent = `
  Frames per Second:
           latest = ${Math.round(fps)}
  avg of last 100 = ${Math.round(mean)}
  min of last 100 = ${Math.round(min)}
  max of last 100 = ${Math.round(max)}
  `.trim()
    }
}
