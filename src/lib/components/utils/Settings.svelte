<script lang="ts">
  import ToggleDarkTheme from "../buttons/ToggleDarkTheme.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";

  let dias = ["..."];
  let modulos = ["..."];

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

<section class="config-card">
  <h2>Configuración</h2>

  <div class="config-section">
    <h3>Nombre</h3>
    <ul>
      {#each dias as dia}
        <li>{dia}</li>
      {/each}
    </ul>
  </div>

  <div class="config-section">
    <h3>Escuela</h3>
    <ul>
      {#each modulos as modulo}
        <li>{modulo}</li>
      {/each}
    </ul>
  </div>

  <div class="config-section theme-toggle">
    <h3>Tema</h3>
    <p>Cambia el tema entre claro y oscuro.</p>
    <div class="toggle-container">
      <ToggleDarkTheme />
    </div>
  </div>

  <div class="config-section">
    <h3>Mapear Escuela</h3>
    <button on:click={() => createWindow('mapping')} class="btn-mapping">Mapear escuela</button>
  </div>
</section>


<style lang="scss">
  /* Estilos para la vista de configuración */
  .config-card {
    background: var(--background);
    color: var(--text-color);
    padding: 20px;
    border-radius: 12px;
    box-shadow: 0px 4px 10px rgba(55, 129, 154, 0.999);
    max-width: 400px;
    margin: auto;
    text-align: center;
    margin-top: 45px;

    

    h2 {
      font-size: 1.5rem;
      margin-bottom: 15px;
    }

    .config-section {
      margin-bottom: 15px;
      padding: 10px;
      border-radius: 8px;
      background: var(--section-background);

      h3 {
        font-size: 1.2rem;
        margin-bottom: 8px;
      }

      ul {
        list-style: none;
        padding: 0;

        li {
          background: var(--item-background);
          padding: 8px;
          margin: 5px 0;
          border-radius: 6px;
        }
      }

      .btn-mapping {
        background: var(--item-background);
        padding: 8px;
        margin: 5px 0;
        border-radius: 6px;
        color: white;
      }
    }

    /* Estilos del toggle de tema */
    .theme-toggle {
      text-align: center;

      p {
        font-size: 0.9rem;
        color: var(--text-secondary);
      }

      .toggle-container {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 10px;
        font-size: 1.5rem;
      }
    }
  }

</style>

