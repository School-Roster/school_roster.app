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

  import ExcelPreview from "./ExcelPreview.svelte";
  import ExcelDataMapping from "./ExcelDataMapping.svelte";

  let dispatch: EventDispatcher<any> = createEventDispatcher();

  export let defaultClass: ClassType;
  export let availableData: Array<{
    name: string;
    key: string;
    required?: boolean;
    type?: string;
  }> = [];

  let showPreview: boolean = false;
  let errorMessage: string | null = null;
  let data: { [key: string]: String | Number }[] = [];
  let fileName: string;
  let currentSelection: { range: string; values: any[] } | null = null;
  let mappingResult: any = null;
  let isSelectingForField: boolean = false;
  let entityFields: typeof availableData = [];
  let entityName: string = "";

  // Get the appropriate entity fields based on the class type
  function setupEntityFields() {
    switch (defaultClass) {
      case ClassType.Groups:
        entityName = "Grupos";
        entityFields = [
          { name: "Grade", key: "grade", required: true },
          { name: "Group", key: "group", required: true },
          { name: "Carrera", key: "career" },
          { name: "Cantidad de estudiantes", key: "students" },
          { name: "Maximo de horas por dia", key: "max_modules_per_day" },
        ];
        break;
      case ClassType.Teachers:
        entityName = "Profesores";
        entityFields = [
          { name: "Nombre", key: "name", required: true },
          { name: "Apellido paterno", key: "father_lastname", required: true },
          { name: "Apellido materno", key: "mother_lastname" },
          { name: "Correo", key: "email" },
          { name: "Teléfono", key: "phone" },
          { name: "Titulo", key: "degree" },
          { name: "Horas en comosion", key: "comissioned_hours", type: "number" },
          { name: "Rendimiento", key: "performance" },
          { name: "Dias preferidos", key: "preferred_days" },
          { name: "Modulos preferidos", key: "preferred_modules" },
        ];
        break;
      case ClassType.Subjects:
        entityName = "Materias";
        entityFields = [
          { name: "Nombre", key: "name", required: true },
          { name: "Abreviatura", key: "shorten" },
          { name: "Color", key: "color", type: "color" },
          { name: "Tipo", key: "spec" },
          { name: "Modulos por semana", key: "required_modules" },
          { name: "Prioridad (1-5)", key: "priority" },
        ];
        break;
      case ClassType.Classrooms:
        entityName = "Aulas";
        entityFields = [
          { name: "Edificio", key: "building_id", required: true },
          { name: "Aula", key: "building_number", required: true },
          { name: "Tipo de aula", key: "building_type" },
          { name: "Capacidad del aula", key: "capacity" },
        ];
        break;
      default:
        entityFields = availableData;
        entityName = "Data";
    }
  }

  async function openFile(): Promise<void> {
    try {
      const filePath: string | string[] | null = await open({
        filters: [{ name: "Excel Files", extensions: ["xlsx", "xls"] }],
      });

      if (filePath && typeof filePath === "string") {
        const dividedFilePath = filePath.split("/");
        fileName = dividedFilePath[dividedFilePath.length - 1];
        // Read the file as a binary buffer using Tauri's fs API
        const arrayBuffer = await readBinaryFile(filePath);
        const workbook = read(arrayBuffer, { type: "array" });
        const sheetName = workbook.SheetNames[0];
        const worksheet = workbook.Sheets[sheetName];
        data = utils.sheet_to_json(worksheet, { header: 1 });

        if (data.length <= 0) return alert("Excel vacío");

        setupEntityFields();
        showPreview = true;
        errorMessage = null;
      }
    } catch (e) {
      console.error(e);
      errorMessage = e instanceof Error ? e.message : "An error occurred";
    }
  }

  function handleSelectionConfirmed(event: CustomEvent) {
    const { selection, range } = event.detail;
    currentSelection = {
      range,
      values: selection.map((item) => item.value),
    };
    console.log("Current selection:", currentSelection);
  }

  function handleSelectionRequest(event: CustomEvent) {
    isSelectingForField = true;
    // Notify ExcelPreview to enable selection mode
    // This could be implemented by passing a prop to ExcelPreview
  }

  function handleMappingApplied() {
    isSelectingForField = false;
    currentSelection = null;
  }

  function handleMappingCancelled() {
    isSelectingForField = false;
    currentSelection = null;
  }

  function handleMappingComplete(event: CustomEvent) {
    mappingResult = event.detail;
    performImport(mappingResult);
  }

  async function performImport(mapping: any): Promise<void> {
    try {
      // Convert the mapping result to the format expected by your import functions
      const processedData = processDataForImport(mapping);
      console.log("Processed data:", processedData);

      switch (defaultClass) {
        case ClassType.Groups:
          await importGroupsFromXlsx(processedData.headerMappings, processedData.data);
          break;
        case ClassType.Classrooms:
          await importClassroomsFromXlsx(processedData.headerMappings, processedData.data);
          break;
        case ClassType.Teachers:
          await importTeachersFromXlsx(processedData.headerMappings, processedData.data);
          break;
        case ClassType.Subjects:
          await importSubjectsFromXlsx(processedData.headerMappings, processedData.data);
          break;
        default:
          throw new Error("Unsupported import type");
      }

      dispatch("importComplete");
      showPreview = false;
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : "Import failed";
      dispatch("importError");
    }
  }

  function processDataForImport(mapping: any): {
    headerMappings: Record<string, string>;
    data: Array<Record<string, any>>;
  } {
    const { mappings } = mapping;

    // Extract the keys (e.g., "name", "color") and their corresponding values
    const fields = Object.keys(mappings);
    const values = fields.map((field) => mappings[field].values);

    // Determine the number of rows based on the first field's values
    const rowCount = values[0]?.length || 0;

    // Create the data array by combining values for each row
    const data = Array.from({ length: rowCount }, (_, index) => {
      const rowData: Record<string, any> = {};
      fields.forEach((field, fieldIndex) => {
        rowData[field] = values[fieldIndex][index];
      });
      return rowData;
    });

    // Create header mappings (if needed)
    const headerMappings = fields.reduce(
      (acc, field) => {
        acc[field] = field; // Map each field to itself (adjust if needed)
        return acc;
      },
      {} as Record<string, string>,
    );

    return {
      headerMappings,
      data,
    };
  }

  onMount(openFile);
</script>

{#if showPreview}
  <div class="excel-import-container">
    <div class="import-layout">
      <div class="preview-section">
        <h3>{fileName}</h3>
        <ExcelPreview
          {data}
          selectionMode={isSelectingForField}
          on:selectionConfirmed={handleSelectionConfirmed}
        />
      </div>

      <div class="mapping-section">
        <ExcelDataMapping
          {entityName}
          fields={entityFields}
          {data}
          excelSelection={currentSelection}
          on:requestSelection={handleSelectionRequest}
          on:mappingApplied={handleMappingApplied}
          on:mappingCancelled={handleMappingCancelled}
          on:mappingComplete={handleMappingComplete}
        />
      </div>
    </div>

    {#if errorMessage}
      <div class="error-message">
        {errorMessage}
      </div>
    {/if}
  </div>
{/if}

<style lang="scss">
  @use "../../../../styles/_variables.scss";

  .excel-import-container {
    padding: 20px;
    max-width: 1200px;
    margin: 0 auto;
  }

  .import-layout {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 20px;
    margin-top: 20px;

    @media (max-width: 768px) {
      grid-template-columns: 1fr;
    }
  }

  .preview-section,
  .mapping-section {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 15px;
  }

  h2 {
    color: variables.$black;
    margin-top: 0;
  }

  h3 {
    margin-top: 0;
    color: variables.$black;
    border-bottom: 1px solid #eee;
    padding-bottom: 10px;
  }

  .error-message {
    background-color: #ffebee;
    color: #c62828;
    padding: 10px 15px;
    border-radius: 4px;
    margin-top: 20px;
  }
</style>
