<!-- Vista de la pizarra para los horarios -->

<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import Navbar from "$lib/components/navbar/Navbar.svelte";
  import GridView from "$lib/components/grid/GridView.svelte";
  import SubjectsPanel from "$lib/components/grid/SubjectsPanel.svelte";
  import WelcomeScreen from "$lib/components/utils/WelcomeScreen.svelte";

  let showWelcomeScreen: boolean = false;
  let checkedSchedule: boolean = false;

  const applySystemTheme = () => {
    const darkModeMediaQuery = window.matchMedia(
      "(prefers-color-scheme: dark)",
    );
    if (darkModeMediaQuery.matches) {
      document.body.classList.add("dark");
    } else {
      document.body.classList.remove("dark");
    }
  };

  const applyTheme = () => {
    const theme = localStorage.getItem("theme");
    if (theme === "dark") {
      document.body.classList.add("dark");
    } else if (theme === "light") {
      document.body.classList.remove("dark");
    } else {
      applySystemTheme();
    }
  };

  onMount(async () => {
  applyTheme();

  // Consultar si hay profesores (como proxy para "hay horario")
  try {
    const hasTeachers = await invoke<boolean>("has_teachers");
    showWelcomeScreen = !hasTeachers; // si no hay profesores, mostrar WelcomeScreen
  } catch (error) {
    console.error("Error al consultar profesores:", error);
    showWelcomeScreen = true; // en caso de error, mostrar pantalla bienvenida
  }

  checkedSchedule = true;

  const handleCloseWelcomeScreen = () => {
    showWelcomeScreen = false;
  };

  const handleShowWelcomeAgain = () => {
    showWelcomeScreen = true;
  };

  window.addEventListener("closeWelcomeScreen", handleCloseWelcomeScreen);
  window.addEventListener("showWelcomeScreenAgain", handleShowWelcomeAgain);

  return () => {
    window.removeEventListener("closeWelcomeScreen", handleCloseWelcomeScreen);
    window.removeEventListener("showWelcomeScreenAgain", handleShowWelcomeAgain);
  };
});

</script>

<main>
  {#if !checkedSchedule}
    <!-- Opcional: mostrar loading mientras se consulta -->
    <p>Cargando...</p>
  {:else}
    {#if showWelcomeScreen}
      <WelcomeScreen />
    {:else}
      <Navbar />
      <div class="content">
        <GridView />
        <SubjectsPanel />
      </div>
    {/if}
  {/if}
</main>
