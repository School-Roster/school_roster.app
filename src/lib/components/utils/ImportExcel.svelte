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

  let excelHeaders: string[] = [];

  // Datos de excel
  type ColumnMapping = {
    field: {
      name: string;
      key: string;
    };
    excelHeader?: string;
  };

  let previewData: Array<Record<string, unknown>> = [];
  let mappings: ColumnMapping[] = [];
  let showPreview: boolean = false;
  let errorMessage: string | null = null;

  $: {
    if (availableData.length > 0 && mappings.length === 0) {
      mappings = availableData.map((field) => ({
        field,
        excelHeader: undefined,
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
        let [headers, rows] = (await invoke("read_xlsx", {
          filePath,
        })) as [string[], Array<Record<string, unknown>>];

        console.log("Headers: ", headers);
        console.log("Rows: ", rows);

        excelHeaders = headers;
        previewData = rows;

        if (rows.length > 0) {
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

  // Generate the header mappings for import
  function generateHeaderMappings(): Record<string, string> {
    const headerMap: Record<string, string> = {};

    for (const mapping of mappings) {
      if (mapping.excelHeaders) {
        headerMap[mapping.field.key] = mapping.excelHeaders;
      }
    }

    return headerMap;
  }

  async function performImport(): Promise<void> {
    console.log("attempt:", mappings);
    try {
      const headerMappings = generateHeaderMappings();
      switch (defaultClass) {
        case ClassType.Groups:
          await importGroupsFromXlsx(headerMappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        case ClassType.Classrooms:
          await importClassroomsFromXlsx(headerMappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        case ClassType.Teachers:
          await importTeachersFromXlsx(headerMappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        case ClassType.Subjects:
          await importSubjectsFromXlsx(headerMappings, previewData);
          dispatch("importComplete");
          showPreview = false;
          break;
        default:
          throw new Error("Unsupported import type");
      }
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : "Import failed";
      dispatch("importError");
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

        {#if previewData.length > 0}
          <div class="preview-sample">
            <h4>Vista previa de datos</h4>
            <table class="import-table">
              <thead>
                <tr>
                  {#each excelHeaders as header}
                    <th>{header}</th>
                  {/each}
                </tr>
              </thead>
              <tbody>
                {#each previewData.slice(0, 3) as row}
                  <tr>
                    {#each excelHeaders as header}
                      <td>{row[header]}</td>
                    {/each}
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
        <div class="columns-grid">
          {#each mappings as m}
            {#if m.field.key !== "id"}
              <div class="column-mapping">
                <span class="field-name">{m.field.name}</span>
                <div class="header-selector">
                  <label>Columna del Excel:</label>
                  <select bind:value={m.excelHeader}>
                    <option value="">Seleccionar columna</option>
                    {#each excelHeaders as header}
                      <option value={header}>{header}</option>
                    {/each}
                  </select>
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
          disabled={!mappings.some((m) => m.excelHeader)}
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

<style>
  .preview-sample {
    margin-bottom: 20px;
    overflow-x: auto;
  }

  .import-table {
    &table {
      border-collapse: collapse;
      width: 100%;
      margin-bottom: 20px;
    }

    &th,
    td {
      border: 1px solid #ddd;
      padding: 8px;
      text-align: left;
    }

    &th {
      background-color: #f0f0f0;
    }
  }
  .column-mapping {
    margin-bottom: 15px;
  }

  .header-selector {
    margin-top: 5px;
  }

  .header-selector select {
    width: 100%;
    padding: 5px;
  }
</style>
