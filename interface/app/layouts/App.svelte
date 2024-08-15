<script>
  import Navbar from "../components/Navbar.svelte";
  import Grid from "./Grid.svelte";
  import SubjectList from "../components/SubjectList.svelte";
</script>

<main class="antialiased bg-[#fffffe]">
  <!-- Sidebar -->
  <Navbar />
  <!-- Main content -->
  <div class="main-container">
    <Grid />
    <SubjectList />
  </div>
</main>

<style>
  :global(body) {
    font-family: "Inria Sans";
  }

  main {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .main-container {
    display: flex;
    flex-direction: column;
    padding: 10px;
    flex-grow: 1;
    overflow: auto;
  }

</style>
