<script lang="ts">
  import "../../styles/crud_view.css";

  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import TableComponent from '../components/TableComponent.svelte';

  type Subject = {
    id: number;
    name: string;
    shorten: string;
    color: string;
    stype: string;
  };

  let subjects: Subject[] = [];

  async function getSubjects() {
    subjects = await invoke('list_subjects');
  }

  onMount(() => {
    getSubjects();
  });

  const columns = ["Abreviacion", "Nombre", "Color", "Tipo"];
  const columnKeys = ["shorten", "name", "color", "stype"];
</script>

<main class="page">
  <div class="container">
    <div class="header">
      <div class="header-title">
        <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-books" viewBox="0 0 24 24" stroke-width="1" fill="none" stroke-linecap="round" stroke-linejoin="round">
          <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
          <path d="M5 4m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z" />
          <path d="M9 4m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z" />
          <path d="M5 8h4" />
          <path d="M9 16h4" />
          <path d="M13.803 4.56l2.184 -.53c.562 -.135 1.133 .19 1.282 .732l3.695 13.418a1.02 1.02 0 0 1 -.634 1.219l-.133 .041l-2.184 .53c-.562 .135 -1.133 -.19 -1.282 -.732l-3.695 -13.418a1.02 1.02 0 0 1 .634 -1.219l.133 -.041z" />
          <path d="M14 9l4 -1" />
          <path d="M16 16l3.923 -.98" />
        </svg>
        <h1>Materias</h1>
      </div>
      <div class="header-info">
        <span>Materias registradas: {subjects.length}</span>
      </div>
    </div>

    <div class="divider"></div>

    <div class="controls">
      <button>
        <img src="/nuevo.svg" alt="Nuevo" />
        <p>Agregar nueva materia</p>
      </button>
      <div class="search">
        <input type="text" placeholder="Buscar materia" />
        <img src="/search.svg" alt="Buscar" />
      </div>
    </div>

    <div class="table-container">
      <TableComponent data={subjects} columns={columns} columnKeys={columnKeys} />
    </div>

  </div>
</main>
