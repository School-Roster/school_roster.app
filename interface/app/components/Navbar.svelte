<script lang="ts">
  import { MenuData } from "../data/MenuData";
  import { WebviewWindow } from "@tauri-apps/api/window";
  import "../../styles/nav.css";

  let isCollapsed = false;
  function colapseSidebar() {
    isCollapsed = !isCollapsed;
  }

  // Open a new window if user selects (grupos | materias | maestros | salones)
  async function createWindow(windowName: string) {
    const win = new WebviewWindow("ss_window", {
      // NOTE: Testing with VistaMaterias
      url: "../../../public/window/" + windowName + ".html",
      title: "School Schedule",
      width: 1000,
      height: 800,
      resizable: true
    });
    await win.show();
  }

</script> <div>
  <nav class="antialiased">
    <div class="sidebar" class:collapsed={isCollapsed}>

      <div class="logo">
        <img src="/logicirculo.png" alt="Logo" />
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
            on:click={() => createWindow(item.menu)}
          >
            <img src={item.icon} alt={item.name} />
            {#if !isCollapsed}
              <span>{item.name}</span>
            {/if}
            {#if item.submenu.length > 0}
              <span class="arrow-container absolute right-0 mr-2 w-2 h-2">
                <img src="/app_icons/right-arrow.svg" alt="Arrow" />
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
          src={isCollapsed ? "/app_icons/caret-right.svg" : "/app_icons/caret-left.svg"}
          alt="Toggle Sidebar"
        />
      </button>
    </div>
  </nav>
</div>

<style>
</style>
