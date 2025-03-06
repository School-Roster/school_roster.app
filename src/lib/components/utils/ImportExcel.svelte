<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { read, utils } from "xlsx";
  import { onMount } from "svelte";
  import { createEventDispatcher, type EventDispatcher } from "svelte";

  import { ClassType } from "$lib/utilities/helpers";
  import { importGroupsFromXlsx } from "$lib/modules/entities/groupsStore";
  import { importClassroomsFromXlsx } from "$lib/modules/entities/classroomStore";
  import { importSubjectsFromXlsx } from "$lib/modules/entities/subjectsStore";
  import { importTeachersFromXlsx } from "$lib/modules/entities/teachersStore";

  let dispatch: EventDispatcher<any> = createEventDispatcher();

  export let defaultClass: ClassType;
  export let availableData: Array<{ name: string; key: string }> = [];

  let excelHeaders: string[] = [];
  let previewData: Array<Record<string, unknown>> = [];
  let mappings: ColumnMapping[] = [];
  let showPreview: boolean = false;
  let errorMessage: string | null = null;

  type ColumnMapping = {
    field: { name: string; key: string };
    excelHeader?: string;
  };

  $: {
    if (availableData.length > 0 && mappings.length === 0) {
      mappings = availableData.map((field) => ({
        field,
        excelHeader: undefined,
      }));
    }
  }

  async function openFile(): Promise<void> {
  try {
    const filePath: string | string[] | null = await open({
      filters: [{ name: "Excel Files", extensions: ["xlsx"] }],
    });

    if (filePath && typeof filePath === "string") {
      // Read the file as a binary buffer using Tauri's fs API
      const arrayBuffer = await readBinaryFile(filePath);
      const workbook = read(arrayBuffer, { type: "array" });
      const sheetName = workbook.SheetNames[0];
      const worksheet = workbook.Sheets[sheetName];
      const jsonData = utils.sheet_to_json(worksheet, { header: 1 });

      if (jsonData.length > 0) {
        excelHeaders = jsonData[0] as string[];
        previewData = jsonData.slice(1).map((row: any) => {
          const rowData: Record<string, unknown> = {};
          excelHeaders.forEach((header, index) => {
            rowData[header] = row[index];
          });
          return rowData;
        });

        showPreview = true;
        errorMessage = null;
      }
    }
  } catch (e) {
    console.error(e);
    errorMessage = e instanceof Error ? e.message : "An error occurred";
  }
}

  function generateHeaderMappings(): Record<string, string> {
    const headerMap: Record<string, string> = {};
    for (const mapping of mappings) {
      if (mapping.excelHeader) {
        headerMap[mapping.field.key] = mapping.excelHeader;
      }
    }
    return headerMap;
  }

  async function performImport(): Promise<void> {
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
    border-collapse: collapse;
    width: 100%;
    margin-bottom: 20px;
  }

  .import-table th,
  .import-table td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
  }

  .import-table th {
    background-color: #f0f0f0;
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
