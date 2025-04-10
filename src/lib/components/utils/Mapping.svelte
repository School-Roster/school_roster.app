<script lang="ts">
  import { classrooms } from "$lib/modules/entities/classroomStore";
  let drawing = false;
  let startX = 0;
  let startY = 0;
  let mouseX = 0;
  let mouseY = 0;
  let deleting = false;
  let grouping = false;
  let selectedShapes: Shape[] = [];
  let showAcceptButton = false;
  let selectedColor = "black";
  let activeButton = "square";

  const buildingLabels = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O"];

  type Shape = | { type: "square"; x: number; y: number; width: number; height: number; color: string } | { type: "circle"; cx: number; cy: number; r: number; color: string };

  let shapes: Shape[] = [];
  let shapeType: "square" | "circle" | "" = "square";

  let currentShape: Shape | null = null;

  function startDraw(event: MouseEvent): void {
    if (deleting || grouping) return;
    drawing = true;
    const { offsetX, offsetY } = event;
    startX = offsetX;
    startY = offsetY;

    if (shapeType === "square") {
        currentShape = { type: "square", x: startX, y: startY, width: 0, height: 0, color: selectedColor };
    } else {
        currentShape = { type: "circle", cx: startX, cy: startY, r: 0, color: selectedColor };
    }
  }

  function endDraw(): void {
    if (!drawing) return;
    drawing = false;
    if (currentShape) {
      shapes = [...shapes, currentShape]; 

      if (currentShape.type === "square") {
        console.log("Rectángulo terminado:", 
          "x:", currentShape.x, 
          "y:", currentShape.y, 
          "width:", currentShape.width, 
          "height:", currentShape.height
        );
      } else if (currentShape.type === "circle") {
        console.log("Círculo terminado:", 
          "cx:", currentShape.cx, 
          "cy:", currentShape.cy, 
          "radio:", currentShape.r
        );
      }

      currentShape = null; 
    }
  }

  function mouseMove(event: MouseEvent): void {
    mouseX = event.offsetX;
    mouseY = event.offsetY;

    if (drawing && currentShape) {
      if (currentShape.type === "square") {
        currentShape.width = Math.abs(mouseX - startX);
        currentShape.height = Math.abs(mouseY - startY);
      } else {
        const radius = Math.sqrt((mouseX - startX) ** 2 + (mouseY - startY) ** 2) / 2;
        currentShape.r = radius;
        currentShape.cx = (startX + mouseX) / 2;
        currentShape.cy = (startY + mouseY) / 2;
      }
    }
  }

  function toggleDeleteMode() {
    deleting = !deleting;
    grouping = false;
    shapeType = "";
    activeButton = deleting ? "delete" : "";
  }

  function toggleGroupingMode() {
    grouping = !grouping;
    deleting = false;
    selectedShapes = [];
    showAcceptButton = false;
    shapeType = "";
    activeButton = grouping ? "group" : "";
  }

  function handleShapeClick(shape: Shape) {
    if (deleting) {
      if (selectedShapes.includes(shape)) {
        shapes = shapes.filter(s => !selectedShapes.includes(s));
      } else {
        shapes = shapes.filter(s => s !== shape);
      }
    } else if (grouping) {
      selectedShapes = [...selectedShapes, shape];
      showAcceptButton = selectedShapes.length >= 2;
    }
  }

  function acceptGrouping() {
    grouping = false;
    showAcceptButton = false;
  }
</script>

<style>
  .canvas {
    width: 96%;
    height: 500px;
    background-color: #f0f0f0;
    border: 1px solid #ccc;
    margin-left: 2%;
  }

  .toolbar {
    margin-bottom: 10px;
  }

  .selected {
    stroke: green;
    stroke-width: 3;
    fill-opacity: 0.7;
  }

  .container {
    margin-top: 20px;
    margin-left: 2%;
  }

  input[type="color"] {
    width: 10vw;
    height: 40px;
    border: none;
    background: transparent;
    cursor: pointer;
    display: inline;
    margin-left: 2%;
  }

  button {
    background-color: #4CAF50;
    color: white;
    border: none; 
    padding: 10px 20px; 
    margin: 0 5px; 
    cursor: pointer; 
    border-radius: 5px;
    font-size: 14px; 
    transition: background-color 0.3s;
  }

  .aceptar {
    background-color: #06587e;
  }

  .aceptar:hover {
    background-color: #044665;
  }

  p {
    margin-left: 2%;
  }

  .selected {
    background-color: #2e6b30;
    color: white;
  }
  
</style>

<div class="toolbar">
  <div class="container">
    <button class:selected={activeButton === "square"} on:click={() => {shapeType = "square"; deleting = false; grouping = false; activeButton = "square"}}>Cuadrado</button>
    <button class:selected={activeButton === "circle"} on:click={() => {shapeType = "circle"; deleting = false; grouping = false; activeButton = "circle"}}>Círculo</button>
    <button class:selected={activeButton === "delete"} on:click={toggleDeleteMode}>Eliminar</button>
    <button class:selected={activeButton === "group"} on:click={toggleGroupingMode}>Agrupar</button>
    {#if showAcceptButton}
      <button on:click={acceptGrouping} class="aceptar">Aceptar</button>
    {/if}
  </div>
  <div>
    <p>Seleccionar color</p>
    <input type="color" bind:value={selectedColor}/>
  </div>
</div>

<svg
class="canvas"
role="application"
aria-label="Área de dibujo"
on:mousedown={startDraw}
on:mouseup={endDraw}
on:mousemove={mouseMove}
>
{#if mouseX && mouseY}
  <line x1={mouseX - 10} y1={mouseY} x2={mouseX + 10} y2={mouseY} stroke="black" stroke-width="2"/>
  <line x1={mouseX} y1={mouseY - 10} x2={mouseX} y2={mouseY + 10} stroke="black" stroke-width="2"/>
{/if}

{#if currentShape}
  {#if currentShape.type === "square"}
    <rect x={currentShape.x} y={currentShape.y} width={currentShape.width} height={currentShape.height} fill={currentShape.color} />
  {:else if currentShape.type === "circle"}
    <circle cx={currentShape.cx} cy={currentShape.cy} r={currentShape.r} fill={currentShape.color} />
  {/if}
{/if}

{#each shapes as shape}
  {#if shape.type === "square"}
    <rect x={shape.x} y={shape.y} width={shape.width} height={shape.height} fill={shape.color} class:selected={selectedShapes.includes(shape) && grouping} on:click={() => handleShapeClick(shape)} />
  {:else if shape.type === "circle"}
    <circle cx={shape.cx} cy={shape.cy} r={shape.r} fill={shape.color} class:selected={selectedShapes.includes(shape) && grouping} on:click={() => handleShapeClick(shape)} />
  {/if}
{/each}
</svg>
  