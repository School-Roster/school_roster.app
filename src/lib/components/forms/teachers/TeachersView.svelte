<script lang="ts">
  // import { invoke } from "@tauri-apps/api";
  // import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";
  import FilterAnimation from "$lib/components/buttons/FilterButton.svelte";

  import { teachers, loadTeachers, type TeacherItem } from "$lib/modules/entities/teachersStore";
  import NewTeacher from "./NewTeacher.svelte";

  let search = "";

  // Carga las materias desde la base de datos en rust
  onMount(loadTeachers);

  // Columnas de la tabla (key es el nombre de la propiedad en la interfaz)
  const columns = [
    { name: "ID", key: "id" },
    { name: "Nombre", key: "name" },
    { name: "Apellido paterno", key: "father_lastname" },
    { name: "Apellido materno", key: "mother_lastname" },
    { name: "Correo", key: "email" },
    { name: "Teléfono", key: "phone" },
    { name: "Titulo", key: "degree" },
    { name: "Horas (comosion)", key: "commissioned_hours" },
    { name: "Horas (activas)", key: "active_hours" },
    { name: "Rendimiento", key: "performance" },
    { name: "Materias", key: "mat" }, // TODO
  ];

  let editShown = false;
  let editItem: TeacherItem | null = null;
  let newShown = false;
  const handleNew = () => {
    newShown = !newShown;
    if (editShown) editShown = false;
  };

  const actions = [
    { name: "Editar", action: (item: TeacherItem) => {
      editShown = true;
      editItem = item;
      if (newShown) newShown = false;
    }},
    { name: "Eliminar", action: (_item: TeacherItem) => {
      // TODO: Implementar confirmación desde el componente en vez de un alert (como si fuera un tooltip)
      let confirm = window.confirm("¿Estás seguro de que quieres eliminar a este profesor?");
      if (!confirm) return;
      // invoke("delete_subject", { id: item.id }).then(loadSubjects);
      // emit("subjects_updated");
    }},
  ];
</script>

<section class="form-container">
  <div class="title">
    <img src="/icons/teacher.svg" alt="Profesores" />
    <span>Profesores</span>
  </div>
  <div class="divider"></div>
  <div class="controls">
    <div class="controls-left">
      <!-- Botón para agregar una nueva materia -->
      <button class="new-button" on:click={handleNew}>
        <img src="/icons/plus.svg" alt="Agregar materia" />
        Agregar nuevo profesor
      </button>

      <!-- Botón para cancelar la edición o creación de una materia -->
      <button class={newShown || editShown ? "cancel-button show" : "cancel-button"} 
              on:click={() => { newShown = false; editShown = false; }}>
        Cancelar
      </button>
    </div>
    <div class="controls-right">
      <!-- Botón para filtrar la tabla por opciones -->
      <FilterAnimation {columns} />
      <!-- Filtro de búsqueda -->
      <SearchAnimation bind:search />
    </div>
  </div>
  <!-- Muestra el formulario de nueva materia si se presiona el botón -->
  {#if newShown}
    <NewTeacher />
  {/if}
  <!-- {#if editShown} -->
  <!--   <NewTeacher item={editItem} /> -->
  <!-- {/if} -->
  <!-- Muestra la tabla de profesores -->
  {#if $teachers.length === 0 && !newShown && !editShown}
    <div class="empty">Agregar un nuevo profesor para comenzar</div>
  {:else}
    {#if search}
      <div class="search-results">Mostrando resultados de búsqueda para "{search}"</div>
      <TableComponent data={$teachers.filter(s => s.name.toLowerCase().includes(search.toLowerCase()))} {columns} {actions} />
    {:else}
      <TableComponent data={$teachers} {columns} {actions} />
    {/if}
  {/if}
</section>
