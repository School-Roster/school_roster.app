<!-- Vista de la pizarra para los horarios -->

<script lang="ts">
  import Navbar from "$lib/components/navbar/Navbar.svelte";
  import GridView from "$lib/components/grid/GridView.svelte";
  import SubjectsPanel from "$lib/components/grid/SubjectsPanel.svelte";
  import { onMount } from "svelte";

  import WelcomeScreen from "$lib/components/utils/WelcomeScreen.svelte";

  let showWelcomeScreen: boolean = true;

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

  onMount(() => {
    applyTheme();
    
    // Listen for the custom event to close the welcome screen
    const handleCloseWelcomeScreen = () => {
      showWelcomeScreen = false;
    };
    
    window.addEventListener('closeWelcomeScreen', handleCloseWelcomeScreen);
    
    return () => {
      window.removeEventListener('closeWelcomeScreen', handleCloseWelcomeScreen);
    };
  });
</script>

<main>
  <!-- TODO: Checar horario guardado -->
  {#if showWelcomeScreen}
    <WelcomeScreen />
  {:else}
    <Navbar />
    <div class="content">
      <GridView />
      <SubjectsPanel />
    </div>
  {/if}
</main>
