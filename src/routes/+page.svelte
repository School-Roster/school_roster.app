<!-- Vista de la pizarra para los horarios -->

<script lang="ts">
  import Navbar from "$lib/components/navbar/Navbar.svelte";
  import GridView from "$lib/components/grid/GridView.svelte";
  import SubjectsPanel from "$lib/components/grid/SubjectsPanel.svelte";
  import { onMount } from "svelte";

  const applySystemTheme = () => {
    const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
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
  }

  onMount(() => {
    applyTheme();
  });
</script>

<main>
  <Navbar />
  <div class="content">
    <GridView />
    <SubjectsPanel />
  </div>
</main>
