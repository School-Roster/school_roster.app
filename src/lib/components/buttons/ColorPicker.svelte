<script lang="ts">
  import "$styles/buttons/color-picker.scss";
  import { clickOutside } from '$lib/utilities/clickOutside';
	import { tick } from 'svelte';
	
  export let id: string = 'color-picker'; // Identificador unico
	export let value: string = '#5E7ABC'; // Valor por si no se selecciona ninguno

  // TODO: Los colores seran elegidos para que no se repitan y se le de una recomendacion al usuario
	let values: string[][] = [
		['#DAAFE9', '#C7DBF5', '#AAD5FB', '#ADE5DA', '#B0EDC3', '#FDF0A4', '#F8D6A2'],
		['#C47ADA', '#90BAEE', '#75BAFA', '#72D5BF', '#73DE8C', '#FBE66E', '#F5B969'],
		['#AE44B7', '#5E7ABC', '#5E7ABC', '#4DACA9', '#63B75A', '#EDBD4A', '#EC9740'],
		['#501B87', '#021B6B', '#0C2794', '#337277', '#2F6A52', '#AE802F', '#AD6127']
  ];

	 // Atajo de teclado
  let trigger: string = 'Escape';
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === trigger) {
      ddActive = false;
    }
  }

	let windowHeight: number; // Altura de la ventana
	let top: boolean; // Si el dropdown esta arriba o abajo

	let ddActive: boolean = false; // Si el dropdown esta activo
	
	let ddHeight: number = 158; // Altura del dropdown
	
	let inputHeight: number; // Altura del input

	async function toggleDropdown(e: MouseEvent) {
    // Si el dropdown no cabe arriba, lo ponemos abajo
		if (
			(e.clientY + inputHeight) < ddHeight
			||
			(windowHeight - ddHeight - inputHeight - e.clientY) > 0
		) {
			top = false;
		} else {
			top = true;
		}
		
		ddActive = !ddActive;

		await tick();
		if (ddActive) {
			//document.querySelector('.color-block.active').focus();
		}
	}
	

  // TODO: Cuando se arregle el error de `click_outside` se podra usar
	// function clickOutsideDropdown() { // Si se hace click afuera del dropdown
	// 	ddActive = false;
  //  en html se usara asi:
  //  <div class:top={top} bind:clientHeight={ddHeight} class="values-dropdown" use:clickOutside on:click_outside={clickOutsideDropdown}>
	// }

	function changeValue(innerValue: string) { // Cambiar el valor del color
		value = innerValue;
		ddActive = false;
	}
	
  // Navegacion por el grid con el teclado
	function keyboardGridNav(e: KeyboardEvent, _index: number) {
		const focussedElement = document.activeElement?.id || '';
	 
		let myRow = parseInt(focussedElement.charAt(focussedElement.length-3));
		let myIndex = parseInt(focussedElement.charAt(focussedElement.length-1));
		let nextRow: number;
		let prevRow: number;
		let nextIndex: number;
		let prevIndex: number;

		switch(e.key) {
      // Flecha izquierda
			case "ArrowLeft":
				prevIndex = myIndex - 1;
				if (prevIndex > -1) {
					document.getElementById(id + '-' + myRow + '-' + prevIndex)?.focus();
				}
				break;
      // Flecha arriba
			case "ArrowUp":
				prevRow = myRow - 1;
				if (prevRow > -1) {
					document.getElementById(id + '-' + prevRow + '-' + myIndex)?.focus();
				}
				break;
      // Flecha derecha
			case "ArrowRight":
				nextIndex = myIndex + 1;
				if (nextIndex < values[0].length) {
					document.getElementById(id + '-' + myRow + '-' + nextIndex)?.focus();
				}
				break;
      // Flecha abajo
			case "ArrowDown":
				nextRow = myRow + 1;
				if (nextRow < values.length) {
					document.getElementById(id + '-' + nextRow + '-' + myIndex)?.focus();
				}
				break;          
		}
	}
</script>

<svelte:window bind:innerHeight={windowHeight} on:keydown={handleKeydown} />

<div class="color-picker-holder">
	<div class="color-picker-inner">
    <label>
      <button bind:clientHeight={inputHeight} class="select-color" on:click={(e) => toggleDropdown(e)} class:fake-focus={ddActive}>
          <div style="background: {value};" class="color-block"></div>
      </button>
    </label>
		<input type="text" bind:value />
	</div>

	{#if ddActive}
    <!-- TODO: Arreglar `click_outside` error -->
		<div class:top={top} bind:clientHeight={ddHeight} class="values-dropdown" use:clickOutside>
			<div class="values-dropdown-grid">
				{#each values as val, index}
					{#each val as innerValue, innerIndex}
						<button
							id="{id}-{index}-{innerIndex}"
							class:active={innerValue === value}
							on:keydown={(e) => keyboardGridNav(e, innerIndex)}
							style="background: {innerValue};"
							on:click={() => { changeValue(innerValue) }}
							class="color-block">
						</button>
					{/each}
				{/each}
			</div>
		</div>
	{/if}
</div>
