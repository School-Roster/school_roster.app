<script lang="ts">
  import { createEventDispatcher } from "svelte";
  
  export let data: { [key: string]: String | Number }[] = [];
  export let selectionMode: boolean = false;
  
  const dispatch = createEventDispatcher();
  
  let headers = data.length > 0 ? Object.keys(data[0]) : [];

  interface CellPosition {
    row: number;
    col: number;
  }

  let selectedCells: Array<CellPosition> = [];
  let selectedRange = { start: null, end: null } as {
    start: CellPosition | null;
    end: CellPosition | null;
  };
  let isSelecting: boolean = false;
  let confirmedSelection: {
    row: number;
    col: number;
    value: String | Number | undefined;
  }[] = [];
  let selectionPreview: string = "";

  // Da al usuario una preview como si fuera excel (ejemplo: C1:C3)
  function getExcelReference(row: number, col: number): string {
    const colRef: string = String.fromCharCode(65 + col); // Convierte la letra
    const rowRef: number = row + 1; // En excel se empieza en 1
    return `${colRef}${rowRef}`;
  }

  function updateSelectionPreview(): void {
    if (!selectedRange.start || !selectedRange.end) {
      selectionPreview = "";
      return;
    }

    const startRef = getExcelReference(
      selectedRange.start.row,
      selectedRange.start.col,
    );
    const endRef = getExcelReference(
      selectedRange.end.row,
      selectedRange.end.col,
    );

    if (startRef === endRef) {
      selectionPreview = startRef;
    } else {
      selectionPreview = `${startRef}:${endRef}`;
    }
  }

  function handleMouseDown(rowIndex: number, colIndex: number): void {
    isSelecting = true;
    selectedRange.start = { row: rowIndex, col: colIndex };
    selectedRange.end = { row: rowIndex, col: colIndex };
    selectCellsInRange();
    updateSelectionPreview();
  }

  function handleMouseMove(rowIndex: number, colIndex: number): void {
    if (isSelecting) {
      selectedRange.end = { row: rowIndex, col: colIndex };
      selectCellsInRange();
      updateSelectionPreview();
    }
  }

  function handleMouseUp(): void {
    isSelecting = false;
  }

  function selectCellsInRange(): void {
    selectedCells = [];
    const { start, end } = selectedRange;

    if (!start || !end) return;

    const minRow = Math.min(start.row, end.row);
    const maxRow = Math.max(start.row, end.row);
    const minCol = Math.min(start.col, end.col);
    const maxCol = Math.max(start.col, end.col);

    for (let i = minRow; i <= maxRow; i++) {
      for (let j = minCol; j <= maxCol; j++) {
        selectedCells.push({ row: i, col: j });
      }
    }
  }

  function confirmSelection() {
    confirmedSelection = selectedCells.map((cell) => ({
      row: cell.row,
      col: cell.col,
      value: data[cell.row]?.[headers[cell.col]],
    }));

    if (confirmedSelection.length <= 0) {
      return;
    }
    
    // Dispatch the selection to the parent component
    dispatch("selectionConfirmed", { 
      selection: confirmedSelection,
      range: selectionPreview
    });
  }
  
  function clearSelection() {
    selectedCells = [];
    selectedRange = { start: null, end: null };
    confirmedSelection = [];
    selectionPreview = "";
  }
</script>

<!-- UI -->
<div class="preview-controls">
  {#if selectionMode || selectedCells.length > 0}
    <div class="selection-actions">
      <button
        on:click={confirmSelection}
        class="confirm-btn"
        disabled={selectedCells.length === 0}
      >
        Confirmar seleccion
      </button>
      
      <button
        on:click={clearSelection}
        class="clear-btn"
        disabled={selectedCells.length === 0}
      >
        Deshacer seleccion
      </button>
    </div>
    
    {#if selectionPreview}
      <div class="selection-range">
        Seleccion actual: <span class="range-text">{selectionPreview}</span>
      </div>
    {/if}
  {/if}
</div>

<!-- Preview del archivo excel -->
<div class="excel-preview">
  <table
    role="grid"
    class="preview-table"
    on:mouseup={handleMouseUp}
  >
    <tr class="column-headers">
      <th></th> <!-- Empty corner cell -->
      {#each headers as _, colIndex}
        <th>{String.fromCharCode(65 + colIndex)}</th>
      {/each}
    </tr>
    <!-- Filas de la tabla -->
    {#each data as row, rowIndex}
      <tr>
        <!-- Row number column -->
        <th class="row-number">{rowIndex + 1}</th>
        
        {#each headers as header, colIndex}
          <td
            role="gridcell"
            class={selectedCells.some(
              (cell) => cell.row === rowIndex && cell.col === colIndex,
            )
              ? "selected"
              : ""}
            on:mousedown={() => handleMouseDown(rowIndex, colIndex)}
            on:mousemove={() => handleMouseMove(rowIndex, colIndex)}
          >
            {row[header]}
          </td>
        {/each}
      </tr>
    {/each}
  </table>
</div>

<style lang="scss">
  @use "../../../../styles/_variables.scss";

  .preview-controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 15px;
    margin-bottom: 10px;
  }

  .selection-actions {
    display: flex;
    gap: 10px;
  }

  .selection-range {
    padding: 6px 10px;
    border-radius: 4px;
    background-color: #f0f7ff;
    font-size: 14px;
  }

  .range-text {
    font-weight: bold;
    color: variables.$primary-color;
    font-family: monospace;
  }

  .excel-preview {
    color: variables.$black;
    background-color: #fffafa;
    overflow-x: auto;
    border: 1px solid #eee;
    border-radius: 4px;

    .preview-table {
      width: 100%;
      border-collapse: collapse;
      -webkit-user-select: none;
      user-select: none;
    }

    td {
      border: 1px solid #ccc;
      padding: 4px;
      cursor: crosshair;
    }

    .selected {
      border: 1px solid #000;
      background-color: #a0c4ff;
    }
    
    .column-headers th,
    .row-number {
      background-color: variables.$white-overlay;
      padding: 4px;
      text-align: center;
      font-weight: bold;
      color: #666;
      position: sticky;
      top: 0;
      z-index: 1;
    }
    
    .row-number {
      position: sticky;
      left: 0;
      z-index: 2;
    }
    
    .column-headers th:first-child {
      z-index: 3;
    }
  }

  .confirm-btn, .clear-btn {
    padding: 6px 12px;
    border-radius: 4px;
    font-size: 14px;
    border: none;
    cursor: pointer;
  }
  
  .confirm-btn {
    background-color: variables.$primary-color;
    color: white;
    
    &:hover {
      background-color: darken(variables.$primary-color, 10%);
    }
    
    &:disabled {
      background-color: #ccc;
      cursor: not-allowed;
    }
  }
  
  .clear-btn {
    background-color: #f0f0f0;
    
    &:hover {
      background-color: darken(#f0f0f0, 10%);
    }
    
    &:disabled {
      background-color: #eee;
      color: #999;
      cursor: not-allowed;
    }
  }
</style>
