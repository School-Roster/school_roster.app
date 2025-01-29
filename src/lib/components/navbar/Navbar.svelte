<script lang="ts">
  import "$styles/nav.scss";
    import ToggleDarkTheme from "../buttons/ToggleDarkTheme.svelte";
  import NavbarItem from "./NavbarItem.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";

  let isCollapsed = false;
  const collapseSidebar = () => {
    isCollapsed = !isCollapsed;
  };

  // Abre una nueva ventana con el nombre especificado (groups, subjects, teachers, classrooms)
  async function createWindow(windowName: string) {
    const win = new WebviewWindow(`${windowName}`, {
      url: `/window/${windowName}`,
      title: "School Roster",
      width: 1000,
      height: 800,
      resizable: true,
      focus: true,
      visible: true,
    });
    await win.show();
  }
</script>

<nav class="sidebar" class:collapsed={isCollapsed}>
  <div class="logo">
    <img src="/icons/logo_transparent.png" alt="Logo" />
    {#if !isCollapsed}
      <h1>School Roster</h1>
    {/if}
  </div>

  <!-- Navbar Items -->
  <NavbarItem {isCollapsed} {createWindow} />

  <button class="toggle-btn" on:click={collapseSidebar}>
    <img
      src={isCollapsed ? "/icons/caret-right.svg" : "/icons/caret-left.svg"}
      alt="Toggle Sidebar"
    />
  </button>
</nav>
