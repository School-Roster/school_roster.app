<script lang="ts">
  import { MenuData } from "../data/MenuData";

  let selectecMenu = "principal";

  const changeMenu = (menu: string) => {
    selectecMenu = menu;
  };

  import RegistroGrupos from "../forms/RegistroGrupos.svelte";
  import FormAulas from "../forms/FormAulas.svelte";
  import FormMaterias from "../forms/FormMaterias.svelte";
  import FormMaestros from "../forms/FormMaestros.svelte";
  import RegistroAulas from "../forms/RegistroAulas.svelte";
  import Login from "../pages/Login.svelte";
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
    {:else if selectecMenu === "materias"}
      <FormMaterias />
    {:else if selectecMenu === "maestros"}
      <FormMaestros />
    {:else if selectecMenu === "Login"}
      <Login />
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
  }

  .navbar {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-around;
    align-items: center;
    width: 100%;
    padding: 0.5rem 0.5rem;
  }

  .btn {
    background-color: var(--headline);
    border: none;
    font-weight: bold;
    color: var(--background);
    cursor: pointer;
    padding: 0.5rem 1rem;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    margin: 0.5rem;
  }

  .btn img {
    width: 2rem;
    height: 2rem;
  }

  .btn span {
    font-family: "Inria Sans";
    font-size: 0.725rem;
    margin-top: 0.5rem;
  }
</style>
