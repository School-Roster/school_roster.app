<script lang="ts">
  import "$styles/nav.scss";
  import { itemData } from "./itemData";
  export let isCollapsed: boolean;
  export let createWindow: (windowName: string) => void;
</script>

{#each itemData as item}
  <!-- representación condicional de los elementos del menú -->
  {#if (item.it === 1 || item.it === 3)}
    <span class="menu">{item.it === 1 ? "Menu" : "Utilidad"}</span>
  {/if}
  {#if (item.it === 2 || item.it === 4 || item.it === 1 || item.it === 3)}
    <div class="separator"></div>
  {/if}

  <!-- elementos del menú -->
  <div class="menu-item">
    <button
      class="btn"
      data-menu={item.menu}
      data-icon={item.icon}
      data-name={item.name}
      on:click={() => createWindow(item.menu)}
    >
      <img src={item.icon} alt={item.name} />
      {#if !isCollapsed}
        <span>{item.name}</span>
      {/if}
      {#if item.submenu.length > 0}
        <span class="arrow-container">
          <img src="/icons/right-arrow.svg" alt="Arrow" />
        </span>
      {/if}
    </button>

    {#if isCollapsed && item.submenu.length == 0}
      <div class="tooltip">{item.name}</div>
    {/if}

    {#if item.submenu.length > 0}
      <ul class="submenu"> 
        {#each item.submenu as subitem}
          <li>
            <img src={subitem.icon} alt={subitem.name} />
            <span>{subitem.name}</span>
          </li>
        {/each}
      </ul>
    {/if}

  </div>
{/each}

