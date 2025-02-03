<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import { createEventDispatcher, type EventDispatcher } from "svelte";

  import { ClassType } from "$lib/utilities/helpers";
  import { importGroupsFromXlsx } from "$lib/modules/entities/groupsStore";
  import { importClassroomsFromXlsx } from "$lib/modules/entities/classroomStore";
  import { importSubjectsFromXlsx } from "$lib/modules/entities/subjectsStore";
  import { importTeachersFromXlsx } from "$lib/modules/entities/teachersStore";

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
        case ClassType.Classrooms:
          await importClassroomsFromXlsx(mappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        case ClassType.Teachers:
          await importTeachersFromXlsx(mappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        case ClassType.Subjects:
          await importSubjectsFromXlsx(mappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        default:
          throw new Error("Unsupported import type");
      }
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : "Import failed";
      dispatch('importError');
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
