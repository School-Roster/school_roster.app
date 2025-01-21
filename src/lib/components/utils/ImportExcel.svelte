<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api";
  import { createEventDispatcher, type EventDispatcher } from "svelte";

  let dispatch: EventDispatcher<any> = createEventDispatcher();

  // Datos de excel
  type ColumnMapping = {
    field: {
      name: string;
      key: string;
    };
    range: {
      column: string;
      startRow: number;
      endRow: number | null;
    };
  };

  let previewData: any[] = [];
  let mappings: ColumnMapping[] = [];
  let showPreview: boolean = false;
  let loading: boolean = false;

  // Datos que el usuario debera seleccionar
  export let availableData: any[] = [];

  $: {
    if (availableData.length > 0 && mappings.length === 0) {
      mappings = availableData.map((field) => ({
        field,
        range: {
          column: "",
          startRow: 1,
          endRow: null,
        },
      }));
    }
  }

  function getExcelColumn(index: number): string {
    let column: string = "";
    while (index >= 0) {
      column = String.fromCharCode(65 + (index % 26)) + column;
      index = Math.floor(index / 26) - 1;
    }
    return column;
  }

  // Funcion para obtener el archivo xlsx
  async function openFile(): Promise<void> {
    try {
      loading = true;

      // Abre el explorador para seleccionar el archivo
      const filePath: string | string[] | null = await open({
        filters: [{ name: "Excel Files", extensions: ["xlsx"] }],
      });

      if (filePath) {
        // Llama a rust para que lea el archivo dado
        const rows: any[] = await invoke("read_xlsx", {
          filePath,
          previewRows: 5,
        });

        // Rust consiguio datos
        if (rows.length > 0) {
          previewData = rows;
          showPreview = true;
        }
      }
    } catch (e) {
      console.log(e);
    } finally {
      loading = false;
    }
  }

  function handleImport(): void {
    const validMappings: Record<string, any> = mappings
      .filter(
        (m: ColumnMapping): number | "" => m.range.column && m.range.startRow,
      )
      .reduce(
        (
          acc: Record<string, any>,
          mapping: ColumnMapping,
        ): Record<string, any> => {
          acc[mapping.field.key] = {
            column: mapping.range.column,
            startRow: mapping.range.startRow,
            endRow: mapping.range.endRow,
          };
          return acc;
        },
        {} as Record<string, any>,
      );

    dispatch("import", { mappings: validMappings });
    showPreview = false;
  }
</script>

<div class="import-container">
  <button class="import-button" on:click={openFile} disabled={loading}>
    {loading ? "Cargando..." : "Importar desde Excel"}
  </button>

  {#if showPreview}
    <div class="preview-container">
      <h2>Vista previa y asignar columnas</h2>

      <!-- Asignar columna -->
      <div class="mapping-section">
        <h3>Asignar columnas</h3>
        <div class="columns-grid">
          {#each mappings as m}
            <div class="column-mapping">
              <span class="field-name">{m.field.name}</span>
              <div class="range-imputs">
                <div class="column-input">
                  <label>Columna:</label>
                  <input
                    type="text"
                    bind:value={m.range.column}
                    placeholder="A"
                    maxlength="3"
                    style="text-transform: uppercase;"
                  />
                </div>

                <div class="row-inputs">
                  <div>
                    <label>Fila inicial:</label>
                    <input
                      type="number"
                      bind:value={m.range.startRow}
                      min="1"
                    />
                  </div>
                  <div>
                    <label>Fila final:</label>
                    <input
                      type="number"
                      bind:value={m.range.endRow}
                      min={m.range.startRow || 1}
                      placeholder="Opcional"
                    />
                  </div>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
      <!-- end: Asignar columna -->

      <!-- Vista previa de los datos -->
      <div class="preview-section">
        <h3>Vista previa de los datos (Primeras 5 filas)</h3>
        <div class="preview-table-container">
          <table class="preview-table">
            <thead>
              <tr>
                {#each Array.from( { length: 26 }, (_, i) => getExcelColumn(i), ) as col}
                  <th>{col}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each previewData as row}
                <tr>
                  {#each Object.values(row) as cell}
                    <td>{cell}</td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
      <!-- end: Vista previa de los datos -->

      <div class="actions">
        <button
          class="import-button"
          on:click={handleImport}
          disabled={!mappings.some((m) => m.range.column && m.range.startRow)}
        >
          Importar columnas seleccionadas
        </button>
        <button class="cancel-button" on:click={() => (showPreview = false)}>
          Cancelar
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .import-container {
    padding: 1rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .import-button {
    background-color: #4caf50;
    color: white;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }

  .import-button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
  }

  .preview-container {
    margin-top: 2rem;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1rem;
  }

  .mapping-section {
    margin-bottom: 2rem;
  }

  .columns-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .column-mapping {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
    border: 1px solid #eee;
    border-radius: 4px;
  }
</style>
