<script lang="ts">
  import { onMount } from "svelte";
  import {
    classrooms,
    loadClassrooms,
  } from "$lib/modules/entities/classroomStore";
  let assigningLetter = false;
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

  let showLetterModal = false;
  let shapeToAssign: Shape | null = null;
  let selectedLetter = "";

  $: buildingOptions = $classrooms.map((c) => c.building_id);

  onMount(() => {
    loadClassrooms();
  });

  type Shape =
    | {
        type: "square";
        x: number;
        y: number;
        width: number;
        height: number;
        color: string;
        letter?: string;
      }
    | {
        type: "circle";
        cx: number;
        cy: number;
        r: number;
        color: string;
        letter?: string;
      };

  let shapes: Shape[] = [];
  let shapeType: "square" | "circle" | "" = "square";

  let currentShape: Shape | null = null;

  function startDraw(event: MouseEvent): void {
    if (deleting || grouping || assigningLetter) return;
    drawing = true;
    const { offsetX, offsetY } = event;
    startX = offsetX;
    startY = offsetY;

    if (shapeType === "square") {
      currentShape = {
        type: "square",
        x: startX,
        y: startY,
        width: 0,
        height: 0,
        color: selectedColor,
      };
    } else {
      currentShape = {
        type: "circle",
        cx: startX,
        cy: startY,
        r: 0,
        color: selectedColor,
      };
    }
  }

  function endDraw(): void {
    if (!drawing) return;
    drawing = false;
    if (currentShape) {
      shapes = [...shapes, currentShape];

      if (currentShape.type === "square") {
        console.log(
          "Rectángulo terminado:",
          "x:",
          currentShape.x,
          "y:",
          currentShape.y,
          "width:",
          currentShape.width,
          "height:",
          currentShape.height,
        );
      } else if (currentShape.type === "circle") {
        console.log(
          "Círculo terminado:",
          "cx:",
          currentShape.cx,
          "cy:",
          currentShape.cy,
          "radio:",
          currentShape.r,
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
        const radius =
          Math.sqrt((mouseX - startX) ** 2 + (mouseY - startY) ** 2) / 2;
        currentShape.r = radius;
        currentShape.cx = (startX + mouseX) / 2;
        currentShape.cy = (startY + mouseY) / 2;
      }
    }
  }

  function toggleDeleteMode() {
    deleting = !deleting;
    grouping = false;
    assigningLetter = false;
    shapeType = "";
    activeButton = deleting ? "delete" : "";
  }

  function toggleGroupingMode() {
    grouping = !grouping;
    deleting = false;
    assigningLetter = false;
    selectedShapes = [];
    showAcceptButton = false;
    shapeType = "";
    activeButton = grouping ? "group" : "";
  }

  function handleShapeClick(shape: Shape) {
    if (deleting) {
      if (selectedShapes.includes(shape)) {
        shapes = shapes.filter((s) => !selectedShapes.includes(s));
      } else {
        shapes = shapes.filter((s) => s !== shape);
      }
    } else if (grouping) {
      selectedShapes = [...selectedShapes, shape];
      showAcceptButton = selectedShapes.length >= 2;
    } else if (assigningLetter) {
      shapeToAssign = shape;
      selectedLetter = "";
      showLetterModal = true;
    }
  }

  function acceptGrouping() {
    grouping = false;
    showAcceptButton = false;
  }

  function toggleAssignLetterMode() {
    assigningLetter = !assigningLetter;
    deleting = false;
    grouping = false;
    shapeType = "";
    activeButton = assigningLetter ? "assign" : "";
  }

  function assignLetterToShape() {
    if (!selectedLetter || !shapeToAssign) return;
    shapeToAssign.letter = selectedLetter;
    shapes = [...shapes];
    showLetterModal = false;
    shapeToAssign = null;
    console.log("Shapes actuales:", shapes);
  }
</script>

<div class="toolbar">
  <div class="container">
    <button
      class:selected={activeButton === "square"}
      on:click={() => {
        shapeType = "square";
        deleting = false;
        grouping = false;
        assigningLetter = false;
        activeButton = "square";
      }}>Cuadrado</button
    >
    <button
      class:selected={activeButton === "circle"}
      on:click={() => {
        shapeType = "circle";
        deleting = false;
        grouping = false;
        assigningLetter = false;
        activeButton = "circle";
      }}>Círculo</button
    >
    <button
      class:selected={activeButton === "delete"}
      on:click={toggleDeleteMode}>Eliminar</button
    >
    <button
      class:selected={activeButton === "group"}
      on:click={toggleGroupingMode}>Agrupar</button
    >
    <button
      class:selected={activeButton === "assign"}
      on:click={toggleAssignLetterMode}>Asignar Letra</button
    >
    {#if showAcceptButton}
      <button on:click={acceptGrouping} class="aceptar">Aceptar</button>
    {/if}
  </div>
  <div>
    <p>Seleccionar color</p>
    <input type="color" bind:value={selectedColor} />
  </div>
</div>

{#if showLetterModal}
  <div
    style="position: fixed; top: 0; left: 0; right: 0; bottom: 0;
              background-color: rgba(0, 0, 0, 0.5); display: flex;
              justify-content: center; align-items: center; z-index: 1000;"
  >
    <div
      style="background: white; padding: 20px; border-radius: 10px; width: 300px; text-align: center;"
    >
      <h3>Asignar Letra</h3>
      <select bind:value={selectedLetter}>
        <option value="" disabled selected>Selecciona una letra</option>
        {#each buildingOptions as letra}
          <option value={letra}>{letra}</option>
        {/each}
      </select>
      <br /><br />
      <button on:click={assignLetterToShape}>Aceptar</button>
      <button
        on:click={() => {
          showLetterModal = false;
          shapeToAssign = null;
        }}>Cancelar</button
      >
    </div>
  </div>
{/if}

<svg
  class="canvas"
  role="application"
  aria-label="Área de dibujo"
  on:mousedown={startDraw}
  on:mouseup={endDraw}
  on:mousemove={mouseMove}
>
  {#if mouseX && mouseY}
    <line
      x1={mouseX - 10}
      y1={mouseY}
      x2={mouseX + 10}
      y2={mouseY}
      stroke="black"
      stroke-width="2"
    />
    <line
      x1={mouseX}
      y1={mouseY - 10}
      x2={mouseX}
      y2={mouseY + 10}
      stroke="black"
      stroke-width="2"
    />
  {/if}

  {#if currentShape}
    {#if currentShape.type === "square"}
      <rect
        x={currentShape.x}
        y={currentShape.y}
        width={currentShape.width}
        height={currentShape.height}
        fill={currentShape.color}
      />
    {:else if currentShape.type === "circle"}
      <circle
        cx={currentShape.cx}
        cy={currentShape.cy}
        r={currentShape.r}
        fill={currentShape.color}
      />
    {/if}
  {/if}

  {#each shapes as shape}
    {#if shape.type === "square"}
      <rect
        x={shape.x}
        y={shape.y}
        width={shape.width}
        height={shape.height}
        fill={shape.color}
        class:selected={selectedShapes.includes(shape) && grouping}
        on:click={() => handleShapeClick(shape)}
      />
      {#if shape.letter}
        <text
          x={shape.x + shape.width / 2}
          y={shape.y + shape.height / 2}
          text-anchor="middle"
          dominant-baseline="middle"
          font-size="16"
          fill="white">{shape.letter}</text
        >
      {/if}
    {:else if shape.type === "circle"}
      <circle
        cx={shape.cx}
        cy={shape.cy}
        r={shape.r}
        fill={shape.color}
        class:selected={selectedShapes.includes(shape) && grouping}
        on:click={() => handleShapeClick(shape)}
      />
      {#if shape.letter}
        <text
          x={shape.cx}
          y={shape.cy}
          text-anchor="middle"
          dominant-baseline="middle"
          font-size="16"
          fill="white">{shape.letter}</text
        >
      {/if}
    {/if}
  {/each}
</svg>

<style>
  .canvas {
    width: 96%;
    height: 500px;
    background-color: #f0f0f0;
    border: 1px solid #ccc;
    margin-left: 2%;
  }

  select {
    width: 100%;
    padding: 8px;
    font-size: 16px;
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
    background-color: #4caf50;
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
