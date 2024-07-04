<script>
  import "../../styles/crud_view.css";

  import SearchBar from "../components/SearchBar.svelte";
  import NuEdEl from "../components/NuEdEl.svelte";
  import TableForm from "../components/TableForm.svelte";
</script>

<main style="margin-top:180px;">
  <div class="container">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" alt="" />
        <h1>Grupos</h1>
      </div>
      <div
        style="display: block; height:1px; width:150%; background-color:black"
      ></div>
      <div>
        <SearchBar />
      </div>
      <div
        style="display: flex; width:90%; justify-content:flex-start; height:100% "
      >
        <img style="width:35px; align-self:end" src="/antes.svg" alt="" />
      </div>
    </div>

    <div style="margin-left: 10px;" class="card">
      <div>
        <TableForm />
      </div>
    </div>
    <div style="margin-left:10px">
      <NuEdEl />
    </div>
  </div>
</main>
