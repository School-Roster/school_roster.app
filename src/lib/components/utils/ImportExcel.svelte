<script lang="ts">
  import { onMount } from "svelte";
  import * as XLSX from "xlsx";

  // Interfaces y tipos
  interface CellPosition {
    col: number;
    row: number;
    colLetter: string;
  }
  let fileInput: HTMLInputElement;

  let preview_container: HTMLDivElement;
  let workbook: XLSX.WorkBook | null = null;
  let activeSheet: string | null = null;
  let sheetNames: string[] = [];

  /*
  let startCell: HTMLTableCellElement | null = null;
  let endCell: HTMLTableCellElement | null = null;
  */

  function upload_file(event: Event): void {
    const target = event.target as HTMLInputElement;
    const file: File | undefined = target.files?.[0];
    if (!file) return;

    const reader = new FileReader();

    reader.onload = function (e: ProgressEvent<FileReader>): void {
      try {
        const result = e.target?.result;
        if (!result) return;

        const data = new Uint8Array(result as ArrayBuffer);
        workbook = XLSX.read(data, { type: "array" });
        sheetNames = workbook.SheetNames;

        if (sheetNames.length > 0) {
          activeSheet = sheetNames[0];
          render_sheet(activeSheet);
        }
      } catch (error) {
        console.error("Error parsing Excel file:", error);
        alert("Error al procesar el archivo Excel");
      }
    };

    reader.readAsArrayBuffer(file);
  }

  function change_sheet(event: Event): void {
    const select = event.target as HTMLSelectElement;
    activeSheet = select.value;
    if (activeSheet) {
      render_sheet(activeSheet);
    }
  }

  function render_sheet(sheetName: string): void {
    if (!workbook) return;

    const worksheet = workbook.Sheets[sheetName];
    const htmlTable: string = XLSX.utils.sheet_to_html(worksheet, {
      id: "excel-preview-table",
    });

    preview_container.innerHTML = htmlTable;

    // Obtener la tabla recién insertada
    const table = preview_container.querySelector(
      "#excel-preview-table",
    ) as HTMLTableElement;
    if (!table) return;

    // Añadir atributos de data-col y data-row a cada celda
    process_table_cells(table);

    // Añadir eventos de selección directamente a cada celda para garantizar que funcionen
    /*
    table.querySelectorAll("td").forEach((cell: HTMLTableCellElement): void => {
        // TODO: AQUI AGREGUEN LA SELECCION DE CELDA
        // usen cell.addEventListener
      });
    });
    */
  }

  function process_table_cells(table: HTMLTableElement): void {
    const rows = table.querySelectorAll("tr");

    rows.forEach((row: HTMLTableRowElement, rowIndex: number): void => {
      const cells: NodeListOf<HTMLTableCellElement> =
        row.querySelectorAll("td");
      cells.forEach((cell, colIndex) => {
        const realColIndex = colIndex + 1;
        cell.dataset.col = realColIndex.toString();
        cell.dataset.row = rowIndex.toString();
        cell.dataset.colLetter = getColumnLetter(realColIndex);

        // Hacer explícitamente seleccionables las celdas
        cell.style.userSelect = "none";
        cell.style.cursor = "cell";
      });
    });
  }

  function getColumnLetter(columnIndex: number): string {
    let temp: number;
    let letter: string = "";
    let colNum: number = columnIndex;

    while (colNum > 0) {
      temp = (colNum - 1) % 26;
      letter = String.fromCharCode(temp + 65) + letter;
      colNum = Math.floor((colNum - temp - 1) / 26);
    }

    return letter;
  }
</script>

<div class="excel-preview-container">
  <div class="controls">
    <input
      type="file"
      accept=".xlsx,.xls,.csv"
      bind:this={fileInput}
      on:change={upload_file}
    />

    {#if workbook && sheetNames.length > 0}
      <div class="sheet-selector">
        <label for="sheet-select">Hoja:</label>
        <select
          id="sheet-select"
          value={activeSheet || ""}
          on:change={change_sheet}
        >
          {#each sheetNames as name}
            <option value={name}>{name}</option>
          {/each}
        </select>
      </div>
    {/if}
  </div>

  <div class="preview" bind:this={preview_container}></div>
</div>

<style lang="scss">
  :global(#excel-preview-table) {
    border-collapse: collapse;
    width: 100%;
    table-layout: fixed;
  }
  :global(#excel-preview-table td),
  :global(#excel-preview-table th) {
    border: 1px solid #ddd;
    padding: 4px 8px;
    min-width: 80px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(#excel-preview-table th) {
    background-color: #f2f2f2;
    position: sticky;
    top: 0;
    z-index: 1;
    font-weight: bold;
  }

  :global(#excel-preview-table td.selected) {
    background-color: rgba(66, 133, 244, 0.3);
  }
</style>
