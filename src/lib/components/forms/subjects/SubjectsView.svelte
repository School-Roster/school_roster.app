<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import NewSubject from "./NewSubject.svelte";
  import { subjects, loadSubjects, type SubjectItem } from "$lib/modules/entities/subjectsStore";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";
  import ConfirmModal from "$lib/components/buttons/ConfirmModal.svelte";

  let search = "";

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
    editShown = true;
    editItem = item;
    if (newShown) newShown = false;
  };

  let showModal = false;
  let subjectToDelete: SubjectItem | null = null;

  const actions = [
    { name: "Editar", action: (item: SubjectItem) => {
      handleEdit(item);
    }},
    { name: "Eliminar", action: (item: SubjectItem) => {
      subjectToDelete = item;
      showModal = true;
    }},
  ];

  const handleDelete = async () => {
    if (!subjectToDelete) return;
    invoke("delete_subject", { id: subjectToDelete.id })
      .then(() => {
        loadSubjects();
        emit("subjects_updated");
      });
    showModal = false;
  };
  const handleCancel = () => { showModal = false; };

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
      <!-- Botón para agregar una nueva materia -->
      <button class="new-button" on:click={handleNew}>
        <img src="/icons/plus.svg" alt="Agregar materia" />
        Agregar nueva materia
      </button>

      <!-- Botón para cancelar la edición o creación de una materia -->
      <button class={newShown || editShown ? "cancel-button show" : "cancel-button"} 
              on:click={() => { newShown = false; editShown = false; }}>
        Cancelar
      </button>
    </div>
    <div class="controls-right">
      <SearchAnimation bind:search />
    </div>
  </div>
  <!-- Muestra el formulario de nueva materia si se presiona el botón -->
  {#if newShown}
    <NewSubject item={null} />
  {/if}
  {#if editShown}
    <NewSubject item={editItem} />
  {/if}
  <!-- Muestra la tabla de materias -->
  {#if $subjects.length === 0 && !newShown && !editShown}
    <div class="empty">No hay materias registradas</div>
  {:else}
    {#if search}
      <div class="search-results">Mostrando resultados de búsqueda para "{search}"</div>
      <TableComponent data={$subjects.filter(s => s.name.toLowerCase().includes(search.toLowerCase()))} {columns} {actions} />
    {:else}
      <TableComponent data={$subjects} {columns} {actions} />
    {/if}

    <!-- Modal de confirmación para eliminar una materia -->
    <ConfirmModal
      isOpen={showModal}
      onConfirm={handleDelete}
      onCancel={handleCancel}
    />
  {/if}
</section>
