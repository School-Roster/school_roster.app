<script lang="ts">
  import { MenuData } from "../data/MenuData";

  import { WebviewWindow } from "@tauri-apps/api/window";

  import VistaMaterias from "../forms/VistaMaterias.svelte";

  const win = new WebviewWindow("ss_window", {
    url: "https://www.google.com",
    title: "School Schedule",
    width: 1000,
    height: 800,
    resizable: true,
    maximizable: true,
    transparent: false,
    fullscreen: false,
    alwaysOnTop: false,
    visible: true,
    x: 0,
    y: 0,
  });

  let isCollapsed = false;
  function colapseSidebar() {
    isCollapsed = !isCollapsed;
  }

  let selectedMenu = "principal";
  function changeMenu(menu: string) {
    selectedMenu = menu;
    console.log(selectedMenu);
  }
</script>

<div>
  <nav class="antialiased">
    <div class="sidebar" class:collapsed={isCollapsed}>

      <div class="logo">
        <img src="/logicirculo.svg" alt="Logo" />
        {#if !isCollapsed}
          <h1>School Schedule</h1>
        {/if}
      </div>

      {#each MenuData as item}
        <!-- Conditional rendering -->
        {#if (item.it === 1 || item.it === 3)}
          <span class="menu">{item.it === 1 ? "Menu" : "Utilidad"}</span>
        {/if}
        {#if (item.it === 2 || item.it === 4 || item.it === 1 || item.it === 3)}
          <div class="lg:mt-4"></div>
        {/if}
        <div class="menu-item">
          <button
            class="btn"
            data-menu={item.menu}
            data-icon={item.icon}
            data-name={item.name}
            on:click={() => changeMenu(item.menu)}
          >
            <img src={item.icon} alt={item.name} />
            {#if !isCollapsed}
              <span>{item.name}</span>
            {/if}
            {#if item.submenu.length > 0}
              <span class="arrow-container absolute right-0 mr-2 w-2 h-2">
                <img src="/bar_icons/right-arrow.svg" alt="Arrow" />
              </span>
            {/if}
          </button>

          {#if isCollapsed && item.submenu.length == 0}
            <div class="tooltip">{item.name}</div>
          {/if}

          {#if item.submenu.length > 0}
            <ul class="submenu"> 
              {#each item.submenu as subitem}
                <li class="flex flex-row space-x-2">
                  <img class="w-4 h-4" src={subitem.icon} alt={subitem.name} />
                  <span>{subitem.name}</span>
                </li>
              {/each}
            </ul>
          {/if}

        </div>
      {/each}

      <!-- Toggle Button -->
      <button class="toggle-btn" on:click={colapseSidebar}>
        <img
          class="w-6 h-6"
          src={isCollapsed ? "/bar_icons/caret-right.svg" : "/bar_icons/caret-left.svg"}
          alt="Toggle Sidebar"
        />
      </button>

    </div>
  </nav>
  {#if selectedMenu !== "principal"}
    <div class="materias-window">
      <VistaMaterias />
    </div>
  {/if}
</div>

<style>
  nav {
    background-color: var(--base-color);
    color: var(--fg-color);
    display: flex;

    height: 100vh;

    border-radius: 0 2px 2px 0;
    box-shadow: 0 20px 20px rgba(0, 0, 0, 0.3);
    border-right: 1px solid var(--gray);

    position: relative;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar {
    font-family: "Inria Serif";
    width: 100%;
    height: 100%;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    transition: width 0.3s ease, padding 0.3s ease;
  }

  .sidebar.collapsed {
    width: 80px;
    padding: 1rem 0.5rem;
  }
  .sidebar.collapsed .logo h1 {
    opacity: 0;
    transition: opacity 0.1s ease;
  }
  .sidebar.collapsed .btn {
    justify-content: center;
    padding: 0.5rem;
  }
  .sidebar.collapsed .btn img {
    margin-right: 0;
  }


  .sidebar .logo {
    display: flex;
    align-items: center;
    margin-bottom: 1.2rem;
    padding-top: 1rem;
    padding-left: 0.5rem;
    border-radius: 10px;
  }

  .sidebar .logo h1 {
    font-family: "Inria Serif";
    font-size: 18px;
    margin-left: 6px;
    transition: opacity 0.3s ease;
  }

  .sidebar .logo img {
    width: 50px;
    height: 50px;
  }

  .sidebar .btn img {
    width: 20px;
    height: 20px;
    margin-right: 1rem;
    filter: invert(43%) sepia(7%) saturate(968%) hue-rotate(172deg) brightness(94%) contrast(89%);
  }

  .sidebar .btn span {
    font-size: 12px;
  }

  .sidebar .menu {
    font-size: 14px;
    margin-top: 0.75rem;
    padding-left: 1rem;
    margin-bottom: 0.5rem;
    color: var(--gray);
    font-weight: 700;
  }

  .sidebar .btn {
    display: flex;
    width: 100%;
    align-items: center;
    padding: 0.5rem 1rem;
    border-radius: 10px;
    margin-bottom: 0.5rem;
    cursor: pointer;
    transition: padding 0.3s ease;
  }

  .sidebar .btn:hover {
    background-color: var(--blue-dark);
    color: var(--base-color);
    transition: background-color 0.3s ease;
  }

  .sidebar .btn:hover img {
    filter: invert(1);
  }

  .sidebar .btn:active {
    background-color: var(--gray-dark);
  }

  .sidebar.collapsed .menu {
    display: none;
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    margin-bottom: 1rem;
    cursor: pointer;
    border-radius: 10px;
    transition: background-color 0.3s ease;

    /* Position the button at the bottom of the sidebar */
    margin-top: auto;
  }

  .toggle-btn:active {
    background-color: var(--gray-dark);
  }

  .toggle-btn img {
    width: 20px;
    height: 20px;
    filter: invert(17%) sepia(54%) saturate(1537%) hue-rotate(174deg) brightness(101%) contrast(94%);
  }

  .menu-item {
    position: relative;
  }

  /* Submenu */
  .submenu {
    list-style: none;
    padding-left: 0;
    margin: 0;
    display: none;
    position: absolute;
    top: 0; /* Aligns the submenu at the top of the button */
    left: 100%; /* Places the submenu to the right of the button */
    margin-left: 0.05rem;
    background-color: var(--gray-dark);
    border-radius: 10px;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
    z-index: 1;
    width: 180px;
  }

  .submenu li {
    padding: 10px 15px;
    color: var(--base-color);
    font-size: 12px;
    cursor: pointer;
  }

  .submenu li:hover {
    background-color: var(--blue-dark);
  }

  .submenu img {
    filter: invert(1);
  }

  /* Show submenu on hover */
  .menu-item:hover .submenu {
    display: block;
  }

  .sidebar.collapsed .menu-item:hover .tooltip {
    display: block;
  }

  /* Tooltip Styles */
  .tooltip {
    display: none;
    position: absolute;
    left: 90px; /* Position the tooltip to the right of the menu item */
    top: 50%;
    transform: translateY(-50%);
    padding: 6px 12px;
    background-color: var(--blue-dark);
    color: var(--base-color);
    border-radius: 4px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    white-space: nowrap;
    z-index: 10;
    font-size: 12px;
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .menu-item:hover .tooltip {
    opacity: 1;
  }

  .arrow-container {
    display: flex;
    align-items: center; /* Centers the arrow vertically */
    justify-content: center; /* Centers the arrow horizontally */
    height: 100%; /* Ensure the container spans the full height of the button */
  }

  .materias-window {
    position: fixed;
    top: 0;
    right: 0;
    width: calc(100% - 240px);
    height: 100vh;
    padding: 1rem;
    transition: width 0.3s ease;
  }
</style>
