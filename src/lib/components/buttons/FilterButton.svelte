<script lang="ts">
  import "$styles/buttons/search.scss";
  export let columns: any[] = [];

  let show_options = false;
</script>

<!-- TODO: Las opciones deben aparecer abajo del botón y con un efecto de despliegue -->
<!-- TODO: Las opciones deben ser botones que al hacer click quiten o agreguen columnas a la tabla -->
<!-- Desarrollador asignado: @moye30 -->

<!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
<div on:click={() => show_options = !show_options} class="search" style="position: relative; width: 40px; margin-right: 12px;">
  <div class="symbol">
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" style="width: 16px; height: 16px;">
      <!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
      <path d="M3.9 54.9C10.5 40.9 24.5 32 40 32l432 0c15.5 0 29.5 8.9 36.1 22.9s4.6 30.5-5.2 42.5L320 320.9 320 448c0 12.1-6.8 23.2-17.7 28.6s-23.8 4.3-33.5-3l-64-48c-8.1-6-12.8-15.5-12.8-25.6l0-79.1L9 97.3C-.7 85.4-2.8 68.8 3.9 54.9z"/>
    </svg>
  </div>
</div>

{#if show_options}
  <div class="options">
    {#each columns as column (column.name)}
      <button>{column.name}</button>
    {/each}
  </div>
{/if}
