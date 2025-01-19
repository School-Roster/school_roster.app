<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";
  import ConfirmModal from "$lib/components/buttons/ConfirmModal.svelte";

  import NoResults from "$lib/components/utils/NoResults.svelte";
  import ImportExcel from "$lib/components/utils/ImportExcel.svelte";

  import NewGroup from "./NewGroup.svelte";
  import {
    groups,
    loadGroups,
    type GroupItem,
  } from "$lib/modules/entities/groupsStore";

  let search = "";
  let filter: string = "Grado";

  // Carga los grupos desde la base de datos en rust
  onMount(loadGroups);

  // Columnas de la tabla (key es el nombre de la propiedad en la interfaz)
  const columns = [
    { name: "ID", key: "id" },
    { name: "Grado", key: "grade" },
    { name: "Grupo", key: "group" },
    { name: "Carrera", key: "career" },
    { name: "Cantidad de estudiantes", key: "students" },
  ];

  let editShown = false;
  let editItem: GroupItem | null = null;
  const handleEdit = (item: GroupItem) => {
    editShown = true;
    editItem = item;
    if (newShown) newShown = false;
  };

  let showModal = false;
  let groupToDelete: GroupItem | null = null;

  const actions = [
    {
      name: "Editar",
      action: (item: GroupItem) => {
        handleEdit(item);
      },
    },
    {
      name: "Eliminar",
      action: (item: GroupItem) => {
        groupToDelete = item;
        showModal = true;
      },
    },
  ];

  const handleDelete = async () => {
    if (!groupToDelete) return;
    invoke("delete_group", { id: groupToDelete.id }).then(() => {
      loadGroups();
      emit("groups_updated");
    });
    showModal = false;
  };
  const handleCancel = () => {
    showModal = false;
  };

  let newShown = false;
  const handleNew = () => {
    newShown = !newShown;
    if (editShown) editShown = false;
  };
</script>

<section class="form-container">
  <div class="title">
    <img src="/icons/group.svg" alt="Grupos" />
    <span>Grupos</span>
  </div>
  <div class="divider"></div>
  <div class="controls">
    <div class="controls-left">
      <!-- Botón para agregar un nuevo elemento -->
      <button class="new-button" on:click={handleNew}>
        <img src="/icons/plus.svg" alt="Agregar" />
        Agregar nuevo grupo
      </button>
      <!-- Boton para importar de excel -->
      <ImportExcel />

      <!-- Botón para cancelar la edición o creación de una materia -->
      <button
        class={newShown || editShown ? "cancel-button show" : "cancel-button"}
        on:click={() => {
          newShown = false;
          editShown = false;
        }}
      >
        Cancelar
      </button>
    </div>
    <div class="controls-right">
      <SearchAnimation bind:search />
    </div>
  </div>
  <!-- Muestra el formulario de nueva materia si se presiona el botón -->
  {#if newShown}
    <NewGroup item={null} />
  {/if}
  {#if editShown}
    <NewGroup item={editItem} />
  {/if}
  <!-- Muestra la tabla -->
  {#if $groups.length === 0 && !newShown && !editShown}
    <NoResults />
  {:else}
    {#if search}
      <div class="search-results">
        Mostrando resultados de búsqueda para "{search}"
      </div>
      <TableComponent
        data={$groups.filter((s) => {
          switch (filter) {
            case "ID":
              return s.id.toString().includes(search);
            case "Grado":
              return s.grade.toString().includes(search);
            case "Grupo":
              return s.group.toLowerCase().includes(search.toLowerCase());
            case "Especialidad":
              return s.career.toLowerCase().includes(search.toLowerCase());
            case "Cantidad estudiantes":
              return s.students.toString().includes(search);
            default:
              // Busqueda por defecto permite al usuario buscar por ejemplo '2A'
              // Búsqueda genérica combinando campos
              return (
                s.id.toString().includes(search) ||
                s.grade.toString().includes(search) ||
                s.group.toLowerCase().includes(search.toLowerCase()) ||
                s.career.toLowerCase().includes(search.toLowerCase()) ||
                s.students.toString().includes(search)
              );
          }
        })}
        {columns}
        {actions}
      />
    {:else}
      <TableComponent data={$groups} {columns} {actions} />
    {/if}

    <!-- Modal de confirmación para eliminar una materia -->
    <ConfirmModal
      isOpen={showModal}
      onConfirm={handleDelete}
      onCancel={handleCancel}
    />
  {/if}
</section>
