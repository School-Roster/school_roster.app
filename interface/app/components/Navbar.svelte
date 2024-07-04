<script lang="ts">
  import { MenuData } from "../data/MenuData";
  import { onMount, afterUpdate } from "svelte";

  import VistaGrupos from "../forms/VistaGrupos.svelte";
  import VistaMaterias from "../forms/VistaMaterias.svelte";
  import VistaAulas from "../forms/VistaAulas.svelte";
  import VistaMaestros from "../forms/VistaMaestros.svelte";
  import FormAulas from "../forms/FormAulas.svelte";
  import FormMaterias from "../forms/FormMaterias.svelte";
  import FormMaestros from "../forms/FormMaestros.svelte";
  import Nosotros from "../pages/Nosotros.svelte";
  import Login from "../pages/Login.svelte";
  import FormGrupos from "../forms/FormGrupos.svelte";
  import NuevoUsuario from "../pages/NuevoUsuario.svelte";

  let selectedMenu = "principal";
  function changeMenu(menu: string) {
    selectedMenu = menu;
  }

  let visibleItems: any[] = []; // Items that fit the screen
  let hiddenItems: any[] = []; // Items that don't fit the screen

  function updateItems() {
    const navbar = document.querySelector(".navbar"); // Navbar container
    const buttons = Array.from(navbar.querySelectorAll(".btn")); // Get all buttons within the navbar

    const navbarWidth = navbar.clientWidth; // Get the navbar width
    let accWidth = 0; // Accumulator for the items' width

    visibleItems = [];
    hiddenItems = [];

    for (const button of buttons) {
      const buttonWidth = button.clientWidth;
      accWidth += buttonWidth;

      if (accWidth <= navbarWidth) {
        visibleItems.push(button.dataset); // Add to visible items
      } else {
        hiddenItems.push(button.dataset); // Add to hidden items
      }
    }
  }

  onMount(() => {
    updateItems();
    window.addEventListener("resize", updateItems); // Listen for resize events
  });

  afterUpdate(() => {
    updateItems(); // Ensure items are updated after each DOM update
  });
</script>

<div>
  <nav>
    <div class="navbar">
      {#each MenuData as item}
        <button class="btn" data-menu={item.menu} data-icon={item.icon} data-name={item.name} on:click={() => changeMenu(item.menu)}>
          <img src={item.icon} alt={item.name} />
          <span>{item.name}</span>
        </button>
      {/each}

      {#if hiddenItems.length > 0}
        <div class="dropdown">
          <button class="btn">...</button>
          <div class="dropdown-menu">
            {#each hiddenItems as item}
              <li>
                <button on:click={() => changeMenu(item.menu)}>
                  <img src={item.icon} alt={item.name} style="width: 1.5rem; height: 1.5rem;" />
                  <!-- <span>{item.name}</span> -->
                </button>
              </li>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </nav>

  <div style="width: 100%;">
    {#if selectedMenu === "principal"}
      <p></p>
    {:else if selectedMenu === "VistaGrupos"}
      <VistaGrupos />
    {:else if selectedMenu === "VistaAulas"}
      <VistaAulas />
    {:else if selectedMenu === "VistaMaterias"}
      <VistaMaterias />
    {:else if selectedMenu === "VistaMaestros"}
      <VistaMaestros />
    {:else if selectedMenu === "Login"}
      <Login />
    {:else if selectedMenu === "FormMaterias"}
      <FormMaterias />
    {:else if selectedMenu === "FormAulas"}
      <FormAulas />
    {:else if selectedMenu === "FormGrupos"}
      <FormGrupos />
    {:else if selectedMenu === "FormMaestros"}
      <FormMaestros />
    {:else if selectedMenu === "Nosotros"}
      <Nosotros />
    {:else if selectedMenu === "NuevoUsuario"}
      <NuevoUsuario />
    {:else}
      <p></p>
    {/if}
  </div>
</div>

<style>
  nav {
    background-color: var(--headline);
    color: var(--text);
    display: flex;
    top: 0;
    left: 0;
    right: 0;
    position: absolute;
    padding: 0.5rem 1rem;
    box-shadow: 0 2
    4px rgba(0, 0, 0, 0.1);
    z-index: 1000;
  }

  .navbar {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-around;
    width: 100%;
    max-width: 1200px;
  }

  .btn {
    background-color: var(--headline);
    border: none;
    font-weight: bold;
    color: var(--background);
    cursor: pointer;
    padding: 0.5rem;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    margin: 0.5rem;
    transition: background-color 0.3s;
  }

  .btn:hover {
    color: var(--blue);
  }

  .btn img {
    width: 2rem;
    height: 2rem;
  }

  .btn span {
    font-family: "Inria Sans", sans-serif;
    font-size: 0.725rem;
    margin-top: 0.5rem;
  }

  .dropdown-menu {
    display: none;
    position: absolute;
    background-color: var(--headline);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    z-index: 1000;
  }

  .dropdown:hover .dropdown-menu {
    display: block;
  }

  @media (max-width: 768px) {
    .dropdown-menu {
      width: 90%;
    }
  }
  
  .dropdown-menu > li > a {
    white-space: normal;
    display: flex;
    align-items: center;
    padding: 0.5rem;
  }
</style>
