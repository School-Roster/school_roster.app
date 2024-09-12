<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import NewSubject from "./NewSubject.svelte";
  import { subjects, loadSubjects, type SubjectItem } from "$lib/modules/entities/subjectsStore";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";

  // Carga las materias desde la base de datos en rust
  onMount(loadSubjects);

  // Columnas de la tabla (key es el nombre de la propiedad en la interfaz)
  const columns = [
    { name: "ID", key: "id" },
    { name: "Nombre", key: "name" },
    { name: "Abreviatura", key: "shorten" },
    { name: "Color", key: "color" },
    { name: "Tipo", key: "spec" },
  ];

  let editShown = false;
  let editItem: SubjectItem | null = null;
  const handleEdit = (item: SubjectItem) => {
    editShown = !editShown;
    editItem = item;
    if (newShown) newShown = false;
  };

  const actions = [
    { name: "Editar", action: (item: SubjectItem) => {
      handleEdit(item);
    }},
    { name: "Eliminar", action: (item: SubjectItem) => {
      // TODO: Implementar confirmación desde el componente en vez de un alert (como si fuera un tooltip)
      let confirm = window.confirm("¿Estás seguro de que quieres eliminar esta materia?");
      if (!confirm) return;
      invoke("delete_subject", { id: item.id }).then(loadSubjects);
      emit("subjects_updated");
    }},
  ];

  let newShown = false;
  const handleNew = () => {
    newShown = !newShown;
    if (editShown) editShown = false;
  };
</script>

<section class="form-container">
  <div class="title">
    <img src="/icons/books.svg" alt="Materias" />
    <span>Materias</span>
  </div>
  <div class="divider"></div>
  <div class="controls">
    <div class="controls-left">
      <button class="new-button" on:click={handleNew}>
        <img src="/icons/plus.svg" alt="Agregar materia" />
        Agregar nueva materia
      </button>

      <button class={newShown || editShown ? "cancel-button show" : "cancel-button"} 
              on:click={() => { newShown = false; editShown = false; }}>
        Cancelar
      </button>
    </div>
    <div class="controls-right">
      <SearchAnimation />
    </div>
  </div>
  <!-- Muestra el formulario de nueva materia si se presiona el botón -->
  {#if newShown}
    <NewSubject item={null} />
  {/if}
  {#if editShown}
    <NewSubject item={editItem} />
  {/if}
  <TableComponent data={$subjects} {columns} {actions} />
</section>
