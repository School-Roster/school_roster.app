<!-- Vista de la pizarra para los horarios -->

<script lang="ts">
  import Navbar from "$lib/components/navbar/Navbar.svelte";
  import GridView from "$lib/components/grid/GridView.svelte";
  import SubjectsPanel from "$lib/components/grid/SubjectsPanel.svelte";
</script>

<main>
  <Navbar />
  <div class="content">
    <GridView />
    <SubjectsPanel />
  </div>
</main>
