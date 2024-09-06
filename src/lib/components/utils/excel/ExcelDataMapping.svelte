<script lang="ts">
  import { createEventDispatcher } from "svelte";
  
  type EntityField = {
    name: string;
    key: string;
    required?: boolean;
    type?: "string" | "number" | "color";
  };
  
  export let entityName: string;
  export let fields: EntityField[] = [];
  export let data: { [key: string]: string | number }[] = [];
  export let excelSelection: { range: string, values: any[] } | null = null;
  
  const dispatch = createEventDispatcher();
  
  let currentField: EntityField | null = null;
  let mappings: { [key: string]: { range: string, values: any[] } } = {};
  let mappingPreview: { [key: string]: any } = {};
  
  function selectFieldForMapping(field: EntityField) {
    currentField = field;
    dispatch("requestSelection", { field });
  }
  
  function applyMapping(): void {
    if (!currentField || !excelSelection) return;
    
    mappings[currentField.key] = excelSelection;
    updateMappingPreview();
    
    // Reset current selection
    currentField = null;
    dispatch("mappingApplied", { field: currentField, mapping: excelSelection });
  }
  
  function cancelMapping(): void {
    currentField = null;
    dispatch("mappingCancelled");
  }
  
  function updateMappingPreview(): void {
    // Create a preview of how the mapped data would look
    mappingPreview = {};
    
    Object.keys(mappings).forEach(key => {
      const values = mappings[key].values;
      if (values.length === 1) {
        mappingPreview[key] = values[0];
      } else if (values.length > 1) {
        mappingPreview[key] = `[${values.length} values]`;
      }
    });
  }
  
  function completeMapping(): void {
    // Create final mapping for import
    const result = {
      entityName,
      mappings,
      // Generate a sample of the first entity that would be created
      sample: mappingPreview
    };

    console.log(result);
    
    dispatch("mappingComplete", result);
  }
  
  function clearMapping(field: EntityField) {
    if (mappings[field.key]) {
      delete mappings[field.key];
      updateMappingPreview();
    }
  }
</script>

<div class="mapping-toolbox">
  <h3>Importar datos para: {entityName}</h3>
  
  {#if currentField}
    <div class="active-mapping">
      <p>Seleccion de datos para: <strong>{currentField.name}</strong></p>
      <div class="selection-actions">
        <button on:click={applyMapping} class="apply-btn" disabled={!excelSelection}>Vincular</button>
        <button on:click={cancelMapping} class="cancel-btn">Cancelar</button>
      </div>
    </div>
  {:else}
    <div class="field-list">
      {#each fields as field}
        <div class="field-item">
          <div class="field-info">
            <span class="field-name">{field.name}</span>
            {#if field.required}
              <span class="required-badge">Obligatorio</span>
            {/if}
          </div>
          <div class="field-actions">
            {#if mappings[field.key]}
              <span class="mapping-info">{mappings[field.key].range}</span>
              <button on:click={() => clearMapping(field)} class="clear-btn">Limpiar</button>
            {:else}
              <button on:click={() => selectFieldForMapping(field)} class="select-btn">Seleccionar</button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
    
    <div class="mapping-preview">
      <h4>Vista previa de datos a importar</h4>
      {#if Object.keys(mappingPreview).length > 0}
        <div class="preview-card">
          {#each Object.entries(mappingPreview) as [key, value]}
            <div class="preview-item">
              <span class="preview-key">{fields.find(f => f.key === key)?.name || key}:</span> 
              <span class="preview-value">{value}</span>
            </div>
          {/each}
        </div>
        <button on:click={completeMapping} class="complete-btn">Importar datos</button>
      {:else}
        <p class="no-preview">Aún no hay asignaciones. Seleccione los datos de los campos para previsualizarlos.</p>
      {/if}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "../../../../styles/_variables.scss";
  
  .mapping-toolbox {
    background-color: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    padding: 20px;
    margin: 20px 0;
  }
  
  h3 {
    margin-top: 0;
    color: variables.$black;
    border-bottom: 1px solid #eee;
    padding-bottom: 10px;
  }
  
  .field-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 20px;
  }
  
  .field-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    border-radius: 4px;
    background-color: #f9f9f9;
    transition: background-color 0.2s;
    
    &:hover {
      background-color: #f0f0f0;
    }
  }
  
  .field-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .field-name {
    font-weight: 500;
  }
  
  .required-badge {
    background-color: #ff6b6b;
    color: white;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 12px;
  }
  
  .field-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  
  .mapping-info {
    font-family: monospace;
    background-color: #e9f5ff;
    padding: 3px 6px;
    border-radius: 4px;
    font-size: 14px;
    color: variables.$primary-color;
  }
  
  .select-btn, .apply-btn, .cancel-btn, .clear-btn, .complete-btn {
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .select-btn {
    background-color: variables.$primary-color;
    color: white;
    
    &:hover {
      background-color: darken(variables.$primary-color, 10%);
    }
  }
  
  .apply-btn {
    background-color: #4caf50;
    color: white;
    
    &:hover {
      background-color: darken(#4caf50, 10%);
    }
    
    &:disabled {
      background-color: #ccc;
      cursor: not-allowed;
    }
  }
  
  .cancel-btn {
    background-color: #f0f0f0;
    
    &:hover {
      background-color: darken(#f0f0f0, 10%);
    }
  }
  
  .clear-btn {
    background-color: #ff6b6b;
    color: white;
    
    &:hover {
      background-color: darken(#ff6b6b, 10%);
    }
  }
  
  .complete-btn {
    background-color: #2196f3;
    color: white;
    margin-top: 15px;
    
    &:hover {
      background-color: darken(#2196f3, 10%);
    }
  }
  
  .active-mapping {
    background-color: #e9f5ff;
    padding: 15px;
    border-radius: 6px;
    margin-bottom: 20px;
    
    p {
      margin-top: 0;
    }
  }
  
  .selection-actions {
    display: flex;
    gap: 10px;
    margin-top: 10px;
  }
  
  .mapping-preview {
    margin-top: 20px;
    padding-top: 15px;
    border-top: 1px dashed #ddd;
    
    h4 {
      margin-top: 0;
    }
  }
  
  .preview-card {
    background-color: #f9f9f9;
    border-radius: 6px;
    padding: 15px;
  }
  
  .preview-item {
    margin-bottom: 8px;
    display: flex;
    
    &:last-child {
      margin-bottom: 0;
    }
  }
  
  .preview-key {
    font-weight: 500;
    min-width: 120px;
  }
  
  .preview-value {
    color: #555;
  }
  
  .no-preview {
    color: #888;
    font-style: italic;
  }
</style>
