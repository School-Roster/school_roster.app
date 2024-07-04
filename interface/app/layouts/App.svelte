<script lang="ts">
  import Navbar from "../components/Navbar.svelte";
  import Grid from "./Grid.svelte";
  import SubjectList from "../components/SubjectList.svelte";
</script>

<main>
  <Navbar />
  <div class="grid-container">
    <Grid />
    <SubjectList />
  </div>
</main>

<style>
  * {
    overflow-y: auto;
  }
  :global(body) {
    font-family: "Inria Sans";
  }
  .grid-container {
    margin-top: 12rem;
  }

  /* Grid */
  @media (min-width: 1366px) {
    .grid-container {
      margin-top: 6.5rem;
    }
  }
</style>
