<script>
  const Menus = {
    principal: "principal",

    grupos: "grupos",
  };

  import FormGrupos from "./FormGrupos.svelte";
  let selectecMenu = Menus.principal;
</script>

<div>
  <nav>
    <div class="navbar1">
      <button
        style="background-color:#094067; border:none;  font-size: 10px;
    color: aliceblue;
    padding: 0.2rem 0.2rem;
    display: inline-flex;
    flex-direction: column;
    align-items: center;"
        on:click={() => {
          selectecMenu = Menus.principal;
        }}
      >
        <img src="/new_file.svg" class="" alt="" />
        <span>Nuevo archivo</span>
      </button>

      <a href="/#" target="_blank" title="Nueva carpeta">
        <img src="/new_folder.svg" class="" alt="" />
        <span>Nueva carpeta</span>
      </a>

      <a href="/#" target="_blank" title="Guardar">
        <img src="/save.svg" class="" alt="" />
        <span>Guadar</span>
      </a>

      <a href="/#" target="_blank" title="preview">
        <img src="/preview.svg" class="" alt="" />
        <span>Vista previa</span>
      </a>

      <a class="" href="/#" target="_blank" title="printer">
        <img src="/printer.svg" class="" alt="" />
        <span>Imprimir</span>
      </a>

      <button
        style="background-color:#094067; border:none;  font-size: 10px;
    color: aliceblue;
    padding: 0.2rem 0.2rem;
    display: inline-flex;
    flex-direction: column;
    align-items: center;"
        on:click={() => {
          selectecMenu = Menus.grupos;
        }}
      >
        <img src="/group.svg" class="" alt="" />
        <span>Grupos</span>
      </button>

      <a href="/#" target="_blank" title="materias">
        <img src="/materias.svg" class="" alt="" />
        <span>Materias</span>
      </a>

      <a href="/#" target="_blank" title="maestros">
        <img src="/maestro.svg" class="" alt="" />
        <span>Maestros</span>
      </a>

      <a href="/#" target="_blank" title="aulas">
        <img src="/aulas.svg" class="" alt="" />
        <span>Aulas</span>
      </a>

      <a href="/#" target="_blank" title="IA">
        <img src="/ia.svg" class="" alt="" />
        <span>IA</span>
      </a>

      <a href="/#" target="_blank" title="hv">
        <img src="/hv.svg" class="" alt="" />
        <span>H.Versiones</span>
      </a>

      <a href="/generate_form" target="_blank" title="generador">
        <img src="/generador.svg" class="" alt="" />
        <span>Generar horario</span>
      </a>

      <a href="/#" target="_blank" title="school">
        <img src="/school.svg" class="" alt="" />
        <span>Institucion</span>
      </a>

      <a href="/#" target="_blank" title="nosotros">
        <img src="/nosotros.svg" class="" alt="" />
        <span>Sobre nosotros</span>
      </a>

      <a href="/#" target="_blank" title="ayuda">
        <img src="/ayuda.svg" class="" alt="" />
        <span>Ayuda</span>
      </a>
    </div>
  </nav>
  <div style="width: 100%;">
    {#if selectecMenu === Menus.principal}
      <p>Condition is true</p>
    {:else if selectecMenu === Menus.grupos}
      <FormGrupos />
    {:else}
      <p></p>
    {/if}
  </div>
</div>

<style>
  nav {
    background-color: #094067;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-direction: row;
  }

  .logo img {
    height: 35px;
    width: 45px;
    border-radius: 50%;
    display: flex;
    flex-direction: column;
    align-items: center;
    display: inline;
  }

  a {
    font-size: 10px;
    color: aliceblue;
    padding: 0.2rem 0.2rem;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
  }

  .navbar1 {
    display: flex;
    flex-direction: column;
    align-items: center;
    display: inline;
  }

  .navbar2 {
    display: flex;
    flex-direction: column;
    align-items: center;
    display: inline;
    align-items: center;
    justify-content: space-between;
  }
</style>
