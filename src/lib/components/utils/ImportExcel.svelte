<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import { createEventDispatcher, type EventDispatcher } from "svelte";

  import { ClassType } from "$lib/utilities/helpers";
  import { importGroupsFromXlsx } from "$lib/modules/entities/groupsStore";

  let dispatch: EventDispatcher<any> = createEventDispatcher();

  // Nos dice para que clase se importaran los datos
  export let defaultClass: ClassType;
  // Datos que el usuario debera seleccionar
  export let availableData: Array<{
    name: string;
    key: string;
  }> = [];

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

  let previewData: Array<Record<string, unknown>> = [];
  let mappings: ColumnMapping[] = [];
  let showPreview: boolean = false;
  let errorMessage: string | null = null;

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

  // Funcion para obtener el archivo xlsx
  async function openFile(): Promise<void> {
    try {
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
          errorMessage = null;
        }
      }
    } catch (e) {
      console.log(e);
      errorMessage = e instanceof Error ? e.message : "An error occurred";
    } finally {
      console.log("Done!, Class:", defaultClass);
    }
  }

  async function performImport(): Promise<void> {
    console.log("attempt:", mappings);
    try {
      switch (defaultClass) {
        case ClassType.Groups:
          await importGroupsFromXlsx(mappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        default:
          throw new Error("Unsupported import type");
      }
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : "Import failed";
      // dispatch('importError');
    }
  }

  onMount(openFile);
</script>

{#if showPreview}
  <div class="form-editor">
    <div class="form-field">
      <h2>Vista previa y asignar columnas</h2>

      {#if errorMessage}
        <span>{errorMessage}</span>
      {/if}

      <!-- Asignar columna -->
      <div class="form-group">
        <h3>Asignar columnas</h3>
        <div class="columns-grid">
          {#each mappings as m}
            {#if m.field.key !== "id"}
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
                        placeholder="fila final"
                      />
                    </div>
                  </div>
                </div>
              </div>
            {/if}
          {/each}
        </div>
      </div>
      <!-- end: Asignar columna -->

      <div class="actions">
        <button
          class="import-button"
          on:click={performImport}
          disabled={!mappings.some((m) => m.range.column && m.range.startRow)}
        >
          Importar columnas seleccionadas
        </button>
        <button class="cancel-button" on:click={() => (showPreview = false)}>
          Cancelar
        </button>
      </div>
    </div>
  </div>
{/if}

<style lang="scss">
  @use "../../../styles/variables";
  .import-container {
    padding: 1rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .import-button {
    background-color: variables.$white-overlay;
    color: variables.$black;
    height: 42px;
    padding: 0.5rem 1rem;
    margin-left: 12px;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    font-weight: bold;
    cursor: pointer;
    border: none;
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

  .cancel-button {
    align-items: center;
    justify-content: center;
    height: 42px;
    padding: 0.5rem 1rem;
    border-radius: 5px;
    background-color: variables.$red;
    color: variables.$white-overlay;
    font-size: 16px;
    font-weight: bold;
    cursor: pointer;
    border: none;
    transition:
      background-color 0.3s,
      transform 0.5s ease-in-out;

    margin-left: 1rem;
    transition:
      opacity 0.5s,
      transform 0.5s ease-in-out;
    &:hover {
      background-color: darken(variables.$red, 6%);
      transition: background-color 0.3s ease-in-out;
    }
  }
</style>
