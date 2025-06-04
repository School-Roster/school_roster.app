<script lang="ts">
  import "$styles/buttons/color-picker.scss";
  import { clickOutside } from '$lib/utilities/clickOutside';
  import { tick } from 'svelte';
  import { usedColors } from '$lib/stores/usedColors';
  import { get } from 'svelte/store';

  export let id: string = 'color-picker';
  export let value: string = '#5E7ABC';

  // Matriz de colores disponibles para selección
let values: string[][] = [
  ['#DAAFE9', '#C7DBF5', '#AAD5FB', '#ADE5DA', '#B0EDC3', '#FDF0A4', '#F8D6A2', '#FFC0CB', '#FFE4B5', '#98FB98', '#AFEEEE', '#E6E6FA', '#FFD700', '#D3D3D3'],
  ['#C47ADA', '#90BAEE', '#75BAFA', '#72D5BF', '#73DE8C', '#FBE66E', '#F5B969', '#FF69B4', '#FFA07A', '#00FA9A', '#40E0D0', '#BA55D3', '#FFDAB9', '#A9A9A9'],
  ['#AE44B7', '#5E7ABC', '#5E7ABC', '#4DACA9', '#63B75A', '#EDBD4A', '#EC9740', '#9932CC', '#4682B4', '#66CDAA', '#9ACD32', '#DAA520', '#BC8F8F', '#808080'],
  ['#501B87', '#021B6B', '#0C2794', '#337277', '#2F6A52', '#AE802F', '#AD6127', '#8A2BE2', '#4169E1', '#20B2AA', '#556B2F', '#8B4513', '#D2691E', '#696969']
];


  let trigger = 'Escape';
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === trigger) ddActive = false;
  }

  let windowHeight: number;
  let top: boolean;
  let ddActive = false;
  let ddHeight = 158;
  let inputHeight: number;

  async function toggleDropdown(e: MouseEvent) {
    top = !((e.clientY + inputHeight) < ddHeight || (windowHeight - ddHeight - inputHeight - e.clientY) <= 0);
    ddActive = !ddActive;
    await tick();
  }

  // Cambiar el color seleccionado y actualizar colores usados
  function changeValue(innerValue: string) {
    value = innerValue;
    ddActive = false;

    usedColors.update((colors) => {
      if (!colors.includes(innerValue)) {
        return [...colors, innerValue];
      }
      return colors;
    });
  }

  // Navegación con teclado en la cuadrícula de colores
  function keyboardGridNav(e: KeyboardEvent) {
    const focussedElement = document.activeElement?.id || '';
    if (!focussedElement) return;
    
    // Los ids tienen formato id-row-col, ej: color-picker-2-3
    const parts = focussedElement.split('-');
    if (parts.length < 3) return;

    let myRow = parseInt(parts[parts.length - 2]);
    let myIndex = parseInt(parts[parts.length - 1]);

    switch (e.key) {
      case "ArrowLeft":
        if (myIndex > 0) document.getElementById(`${id}-${myRow}-${myIndex - 1}`)?.focus();
        e.preventDefault();
        break;
      case "ArrowRight":
        if (myIndex < values[myRow].length - 1) document.getElementById(`${id}-${myRow}-${myIndex + 1}`)?.focus();
        e.preventDefault();
        break;
      case "ArrowUp":
        if (myRow > 0) document.getElementById(`${id}-${myRow - 1}-${myIndex}`)?.focus();
        e.preventDefault();
        break;
      case "ArrowDown":
        if (myRow < values.length - 1) document.getElementById(`${id}-${myRow + 1}-${myIndex}`)?.focus();
        e.preventDefault();
        break;
    }
  }
</script>

<svelte:window bind:innerHeight={windowHeight} on:keydown={handleKeydown} />

<div class="color-picker-holder">
  <div class="color-picker-inner">
    <label>
      <button
        bind:clientHeight={inputHeight}
        class="select-color"
        on:click={toggleDropdown}
        class:fake-focus={ddActive}
        aria-haspopup="listbox"
        aria-expanded={ddActive}
        aria-labelledby={`${id}-label`}
      >
        <div style="background: {value};" class="color-block"></div>
      </button>
    </label>
    <input type="text" bind:value aria-label="Color hexadecimal" />
  </div>

  {#if ddActive}
    <div class:top={top} bind:clientHeight={ddHeight} class="values-dropdown" use:clickOutside on:click={() => (ddActive = false)}>
      <div class="values-dropdown-grid" role="listbox" aria-activedescendant={`${id}-0-0`}>
        {#each values as val, rowIndex}
          {#each val as innerValue, colIndex}
            {#if !get(usedColors).includes(innerValue) || innerValue === value}
              <button
                id={`${id}-${rowIndex}-${colIndex}`}
                class="color-option"
                class:active={innerValue === value}
                on:keydown={keyboardGridNav}
                style="background: {innerValue};"
                on:click={() => changeValue(innerValue)}
                role="option"
                aria-selected={innerValue === value}
                tabindex={innerValue === value ? "0" : "-1"}
                type="button"
              ></button>
            {/if}
          {/each}
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>

  .color-picker-holder {
    margin-left: 10px;
    position: relative;
    width: 180px;
  }
  .color-picker-inner {
    display: flex;
    align-items: center;
  }
  .select-color {
    margin-top: 4px;
    width: 24px;
    height: 24px;
    cursor: pointer;
    padding: 0;
    margin-right: 8px;
    border-radius: 4px;
  }
  .color-block {
    width: 100%;
    height: 100%;
    border-radius: 4px;
  }
  .values-dropdown {
    position: absolute;
    background: white;
    border: 1px solid #ccc;
    margin-top: 4px;
    border-radius: 4px;
    z-index: 100;
    display: grid;
    padding: 8px;
  }
  .values-dropdown.top {
    bottom: 100%;
    margin-top: 0;
    margin-bottom: 4px;
  }
  .values-dropdown-grid {
    display: grid;
    grid-template-columns: repeat(7, 24px);
    grid-gap: 8px;
  }
  .color-option {
    width: 24px;
    height: 24px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .color-option.active {
    box-shadow: 0 0 0 3px #00000088;
  }
  .color-option:focus {
    outline: 2px solid #333;
  }
</style>
