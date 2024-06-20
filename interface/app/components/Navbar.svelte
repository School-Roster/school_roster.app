<script lang="ts">
  import { MenuData } from "../data/MenuData";

  let selectecMenu = "principal";

  const changeMenu = (menu: string) => {
    selectecMenu = menu;
  };

  import RegistroGrupos from "../forms/RegistroGrupos.svelte";
  import RegistroMaterias from "../forms/RegistroMaterias.svelte";
  import RegistroAulas from "../forms/RegistroAulas.svelte";
  import RegistroMaestros from "../forms/RegistroMaestros.svelte";
  import FormAulas from "../forms/FormAulas.svelte";
  import FormMaterias from "../forms/FormMaterias.svelte";
  import FormMaestros from "../forms/FormMaestros.svelte";
  import Nosotros from "../pages/Nosotros.svelte";
  import Login from "../pages/Login.svelte";
  import FormGrupos from "../forms/FormGrupos.svelte";
  import NuevoUsuario from "../pages/NuevoUsuario.svelte";
</script>

<div>
  <nav>
    <div class="navbar">
      {#each MenuData as item}
        <button class="btn" on:click={() => changeMenu(item.menu)}>
          <img src={item.icon} alt={item.name} />
          <span>{item.name}</span>
        </button>
      {/each}
    </div>
  </nav>

  <div style="width: 100%;">
    {#if selectecMenu === "principal"}
      <p></p>
    {:else if selectecMenu === "RegistroGrupos"}
      <RegistroGrupos />
    {:else if selectecMenu === "RegistroAulas"}
      <RegistroAulas />
    {:else if selectecMenu === "RegistroMaterias"}
      <RegistroMaterias />
    {:else if selectecMenu === "RegistroMaestros"}
      <RegistroMaestros />
    {:else if selectecMenu === "Login"}
      <Login />
    {:else if selectecMenu === "FormMaterias"}
      <FormMaterias />
    {:else if selectecMenu === "FormAulas"}
      <FormAulas />
    {:else if selectecMenu === "FormGrupos"}
      <FormGrupos />
    {:else if selectecMenu === "FormMaestros"}
      <FormMaestros />
    {:else if selectecMenu === "Nosotros"}
      <Nosotros />
    {:else if selectecMenu === "NuevoUsuario"}
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
    justify-content: center;
    align-items: center;
    top: 0;
    left: 0;
    right: 0;
    position: absolute;
    padding: 0.5rem 1rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    z-index: 1000;
  }

  .navbar {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-around;
    align-items: center;
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
    /* background-color: var(--background); */
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

  @media (max-width: 768px) {
    .btn {
      padding: 0.25rem;
    }

    .btn img {
      width: 1.5rem;
      height: 1.5rem;
    }

    .btn span {
      font-size: 0.625rem;
    }
  }

  @media (max-width: 40px) {
    .navbar {
      flex-direction: column;
      align-items: stretch;
    }

    .btn {
      width: 100%;
      justify-content: flex-start;
      padding: 0.5rem 1rem;
      margin: 0.25rem 0;
    }

    .btn img {
      margin-right: 1rem;
    }

    .btn span {
      font-size: 1rem;
      margin-top: 0;
    }
  }
</style>
