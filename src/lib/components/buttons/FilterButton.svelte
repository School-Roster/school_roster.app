<script lang="ts">
  import { onMount } from "svelte";
  import { slide } from "svelte/transition";

  export let filter: string = "";
  export let columns: any[] = [];

  let show_options: boolean = false;
  let selectedColumn: string = "";
  let dropdown: HTMLDivElement;

  const toggleColumn = (columnName: string) => {
    selectedColumn = columnName;
    filter = columnName;
  };
  const clickOutside = (event: MouseEvent) => {
    if (dropdown && !dropdown.contains(event.target as Node)) {
      show_options = false;
    }
  };

  onMount(() => {
    document.addEventListener("click", clickOutside);
    return () => {
      document.removeEventListener("click", clickOutside);
    };
  });
</script>

<!-- Botón para filtrar -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  on:click={(event) => {
    event.stopPropagation();
    show_options = !show_options;
  }}
  class="search"
>
  <div class="symbol">
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="icon">
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
  >
    {#each columns as column (column.name)}
      <label>
        <input
          type="radio"
          name="columns"
          value={column.name}
          on:change={() => {
            toggleColumn(column.name);
            show_options = false;
          }}
          checked={selectedColumn === column.name}
        />
        {column.name}
      </label>
    {/each}
  </div>
{/if}

<style lang="scss">
  @use "../../../styles/variables";
  .search {
    position: relative;
    width: 18px;
    height: 42px;
    padding: 0 12px;
    margin-right: 10px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    overflow: hidden;
    align-items: center;
    justify-content: center;

    .symbol {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 100%;
      width: 100%;
      position: absolute;
      top: 0;
      z-index: 1;

      .icon {
        position: absolute;
        fill: variables.$white-overlay;
        stroke: none;
        width: 12px;
        height: 12px;
        z-index: 2;
      }
    }
  }

  .options {
    position: absolute;
    top: 185px;
    right: 0;
    margin-right: 16px;
    background-color: variables.$white-background;
    border: 1px solid variables.$black;
    color: variables.$black;
    border-radius: 10px;
    padding: 10px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    z-index: 1000;

    label {
      display: flex;
      align-items: center;
      padding: 5px 0;

      &:hover {
        cursor: pointer;
        text-decoration: underline;
      }

      input[type="radio"] {
        appearance: none;
        width: 16px;
        height: 16px;
        border: 2px solid variables.$blue;
        border-radius: 50%;
        margin-right: 10px;
        position: relative;
        outline: none;
        cursor: pointer;
        transition: background-color 0.3s ease;

        &:checked {
          background-color: #007bff;
          border-color: #007bff;

          &::after {
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
        }

        &:hover {
          border-color: #0056b3;
        }
      }
    }
  }
</style>
