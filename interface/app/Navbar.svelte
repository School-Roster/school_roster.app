<script>
  const Menus = {
    principal: "principal",
    aulas: "aulas",
    materias: "materias",
    maestros: "maestros",
    Bienvenida: "bienvenida",
    RegistroGrupos: "registrogrupos",
  };

  import FormGrupos from "./FormGrupos.svelte";
  import FormAulas from "./FormAulas.svelte";
  import FormMaterias from "./FormMaterias.svelte";
  import FormMaestros from "./FormMaestros.svelte";
  import FormTopics from "./FormTopics.svelte";
  import Bienvenida from "./bienvenida.svelte";
  import RegistroGrupos from "./RegistroGrupos.svelte";
  let selectecMenu = Menus.principal;
</script>

<div>
  <nav>
    <div class="navbar">
      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/new_file.svg" class="" alt="" />
        <span>Nuevo archivo</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/new_folder.svg" class="" alt="" />
        <span>Nueva carpeta</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/save.svg" class="" alt="" />
        <span>Guadar</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/preview.svg" class="" alt="" />
        <span>Vista previa</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/printer.svg" class="" alt="" />
        <span>Imprimir</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.RegistroGrupos; }}>
        <img src="/group.svg" class="" alt="" />
        <span>Grupos</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.materias; }}>
        <img src="/materias.svg" class="" alt="" />
        <span>Materias</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.maestros; }}>
        <img src="/maestro.svg" class="" alt="" />
        <span>Mestros</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.aulas; }}>
        <img src="/aulas.svg" class="" alt="" />
        <span>Aulas</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/ia.svg" class="" alt="" />
        <span>IA</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/hv.svg" class="" alt="" />
        <span>H.Versiones</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.Bienvenida; }}>
        <img src="/generador.svg" class="" alt="" />
        <span>Generar</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/school.svg" class="" alt="" />
        <span>Institucion</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/nosotros.svg" class="" alt="" />
        <span>Sobre nosotros</span>
      </button>

      <button class="btn" on:click={() => { selectecMenu = Menus.principal; }}>
        <img src="/ayuda.svg" class="" alt="" />
        <span>Ayuda</span>
      </button>
    </div>
  </nav>

  <div style="width: 100%;">
    {#if selectecMenu === Menus.principal}
      <p></p>
    {:else if selectecMenu === Menus.RegistroGrupos}
      <RegistroGrupos />
    {:else}
      <p></p>
    {/if}
  </div>

  <div style="width: 100%;">
    {#if selectecMenu === Menus.principal}
      <p></p>
    {:else if selectecMenu === Menus.aulas}
      <FormAulas />
    {:else}
      <p></p>
    {/if}
  </div>

  <div style="width: 100%;">
    {#if selectecMenu === Menus.principal}
      <p></p>
    {:else if selectecMenu === Menus.materias}
      <!-- <FormMaterias /> -->
      <FormMaterias />
    {:else}
      <p></p>
    {/if}
  </div>

  <div style="width: 100%;">
    {#if selectecMenu === Menus.principal}
      <p></p>
    {:else if selectecMenu === Menus.maestros}
      <FormMaestros />
    {:else}
      <p></p>
    {/if}
  </div>
  <div style="width: 100%;">
    {#if selectecMenu === Menus.principal}
      <p></p>
    {:else if selectecMenu === Menus.Bienvenida}
      <Bienvenida />
    {:else}
      <p></p>
    {/if}
  </div>
</div>

<style>
  nav {
    /* use headline variable as background color */
    background-color: var(--headline);
    color: var(--text);
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-direction: row;
    top: 0;
    left: 0;
    right: 0;
    position: absolute;
  }

  .navbar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    display: inline;
    width: 100%;
    padding: 0.5rem 0.5rem;
  }

  .btn {
    background-color: var(--headline);
    border: none;
    font-size: 10px;
    font-weight: bold;
    color: var(--background);
    cursor: pointer;
    padding: 0.5rem 1rem;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
  }

  .btn img {
    width: 2rem;
    height: 2rem;
  }

  .btn span {
    margin-top: 0.5rem;
  }

</style>
