<script lang="ts">
  import { onMount } from "svelte";
  import { slide } from "svelte/transition";
  export let columns: any[] = [];

  let show_options = false;
  let selectedColumn: string = "";
  let dropdown: HTMLDivElement; // Referencia al contenedor del dropdown

  // Función para alternar la selección de columna
  const toggleColumn = (columnName: string) => {
    selectedColumn = columnName;
    // Lógica para agregar/quitar columnas a la tabla
    console.log(`Column toggled: ${columnName}`);
  };

  // Función para manejar clics fuera del dropdown
  const handleClickOutside = (event: MouseEvent) => {
    if (dropdown && !dropdown.contains(event.target as Node)) {
      show_options = false;
    }
  };

  // Añadimos el listener de clic en el montaje del componente
  onMount(() => {
    document.addEventListener("click", handleClickOutside);
    return () => {
      document.removeEventListener("click", handleClickOutside);
    };
  });
</script>

<!-- Botón de búsqueda -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  on:click={(event) => {
    event.stopPropagation();
    show_options = !show_options;
  }}
  class="search"
  style="position: relative; width: 110px; margin-right: 12px; border-radius: 10px; background-color: #f1f1f1; padding: 10px; cursor: pointer; display: flex; align-items: center; justify-content: center;  height: 25px;"
>
  <div
    class="symbol"
    style="display: flex; align-items: center; justify-content: center;"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 512 512"
      style="width: 16px; height: 16px;"
    >
      <path
        d="M3.9 54.9C10.5 40.9 24.5 32 40 32l432 0c15.5 0 29.5 8.9 36.1 22.9s4.6 30.5-5.2 42.5L320 320.9 320 448c0 12.1-6.8 23.2-17.7 28.6s-23.8 4.3-33.5-3l-64-48c-8.1-6-12.8-15.5-12.8-25.6l0-79.1L9 97.3C-.7 85.4-2.8 68.8 3.9 54.9z"
      />
    </svg>
  </div>
</div>

<!-- Opciones del dropdown -->
{#if show_options}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    bind:this={dropdown}
    class="options"
    transition:slide={{ duration: 300 }}
    on:click|stopPropagation
    style="position: absolute; top: 185px; background-color: #fff; border: 4px solid #004d98; border-radius: 10px; padding: 10px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); z-index: 1000;"
  >
    {#each columns as column (column.name)}
      <label style="display: flex; align-items: center; padding: 10px 0; ">
        <input
          type="radio"
          name="columns"
          value={column.name}
          checked={selectedColumn === column.name}
          on:change={() => toggleColumn(column.name)}
          style="appearance: none; width: 16px; height: 16px; border: 2px solid #007bff; border-radius: 50%; margin-right: 10px; position: relative; outline: none; cursor: pointer; transition: background-color 0.3s ease;"
        />
        {column.name}
        <style>
          input[type="radio"]:checked {
            background-color: #007bff;
            border-color: #007bff;
          }

          input[type="radio"]:checked::after {
            content: "";
            width: 8px;
            height: 8px;
            background-color: #fff;
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            border-radius: 50%;
          }

          input[type="radio"]:hover {
            border-color: #0056b3;
          }

          label:hover {
            background-color: #f9f9f9;
          }
        </style>
      </label>
    {/each}
  </div>
{/if}
