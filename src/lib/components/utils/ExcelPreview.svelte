<script lang="ts">
    export let data: { [key: string]: String | Number }[] = [];
    let headers = data.length > 0 ? Object.keys(data[0]) : [];

    interface CellPosition {
        row: number;
        col: number;
    }

    let selectedCells: Array<CellPosition> = [];
    let selectedRange = { start: null, end: null } as { start: CellPosition | null; end: CellPosition | null };
    let isSelecting = false;
    let confirmedSelection: { row: number; col: number; value: String | Number | undefined }[] = [];


    function handleMouseDown(rowIndex: number, colIndex: number) {
        isSelecting = true;
        selectedRange.start = { row: rowIndex, col: colIndex };
        selectedRange.end = { row: rowIndex, col: colIndex };
        selectCellsInRange();
    }

    function handleMouseMove(rowIndex: number, colIndex: number) {
        if (isSelecting) {
            selectedRange.end = { row: rowIndex, col: colIndex };
            selectCellsInRange();
        }
    }

    function handleMouseUp() {
        isSelecting = false;
    }

    function selectCellsInRange() {
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
        confirmedSelection = selectedCells.map(cell => ({
            row: cell.row,
            col: cell.col,
            value: data[cell.row]?.[headers[cell.col]]
        }));

        if (confirmedSelection.length > 0) {
            alert(`Datos seleccionados:\n${confirmedSelection.map(item => `Fila ${item.row + 1}, Columna ${item.col + 1}: ${item.value}`).join("\n")}`);
        }
    }
</script>

<div class="table-view">
    <table
        role="grid"
        on:mousedown={() => handleMouseDown}
        on:mousemove={() => handleMouseMove}
        on:mouseup={handleMouseUp}
    >
        <!-- Filas de la tabla -->
        {#each data as row, rowIndex}
            <tr>
                {#each headers as header, colIndex}
                    <td
                        role="gridcell"
                        class={selectedCells.some(
                            (cell) =>
                                cell.row === rowIndex && cell.col === colIndex,
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

<!-- Sección de datos seleccionados (solo después de presionar el botón) -->
{#if confirmedSelection.length > 0}
    <div class="selected-data">
        <h3>Datos confirmados:</h3>
        <ul>
            {#each confirmedSelection as item}
                <li>Fila {item.row + 1}, Columna {item.col + 1}: {item.value}</li>
            {/each}
        </ul>
    </div>
{/if}

<!-- Botón para confirmar selección -->
<button on:click={confirmSelection} class="continue-btn" disabled={selectedCells.length === 0}>
    Continuar
</button>

<style>
    .table-view {
        color: black;
        background-color: #fffafa;
    }
    table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 20px;
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
    .selected-data {
        margin-top: 20px;
        padding: 10px;
        background-color: black;
        border-radius: 5px;
    }
    .continue-btn {
        margin-top: 10px;
        padding: 10px;
        background-color: #28a745;
        color: white;
        border: none;
        cursor: pointer;
        font-size: 16px;
        border-radius: 5px;
    }
    .continue-btn:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }
</style>
