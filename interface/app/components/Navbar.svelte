<script lang="ts">
  import { MenuData } from "../data/MenuData";

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
    </div>
  </nav>

  <div class="new-page">
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
  
  .new-page {
    position: absolute;
    width: 100%;
    /* height: 100%; */
    background-color: var(--background);
  }
</style>
