<script lang="ts">
  import ToggleDarkTheme from "../buttons/ToggleDarkTheme.svelte";

  let dias = ["..."];
  let modulos = ["..."];
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
      <span>🌞</span>
      <ToggleDarkTheme />
      <span>🌙</span>
    </div>
  </div>
</section>
