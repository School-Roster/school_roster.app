<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";
  import ConfirmModal from "$lib/components/buttons/ConfirmModal.svelte";

  import NoResults from "$lib/components/utils/NoResults.svelte";
  import ImportExcel from "$lib/components/utils/excel/ImportExcel.svelte";
  import { ClassType } from "$lib/utilities/helpers";

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
    { name: "Modulos a la semana", key: "max_modules_per_day" },
    { name: "Materias pre-asignadas", key: "required_subjects" },
  ];

  let importShown: boolean = false;

  let editShown: boolean = false;
  let editItem: GroupItem | null = null;

  let showModal: boolean = false;
  // Manejamos eliminar por medio de un diccionario si hay varios seleccionados
  let groupToDelete: { single?: GroupItem; multiple?: number[] } | null = null;

  const actions = [
    {
      name: "Editar",
      action: (item: GroupItem) => {
        handleEdit(item);
      },
    },
    {
      name: "Eliminar",
      action: (itemOrItems: GroupItem | number[]) => {
        if (Array.isArray(itemOrItems)) {
          groupToDelete = { multiple: itemOrItems };
        } else {
          groupToDelete = { single: itemOrItems };
        }
        showModal = true;
      },
    },
  ];

  const handleDelete = async (): Promise<void> => {
    if (!groupToDelete) return;

    try {
      if (groupToDelete.multiple) {
        console.log(groupToDelete.multiple);
        await invoke("delete_groups", { ids: groupToDelete.multiple });
      } else if (groupToDelete.single) {
        await invoke("delete_group", { id: groupToDelete.single.id });
      }
      loadGroups();
      emit("groups_updated");
    } catch (error) {
      console.error("Error deleting groups:", error);
    }
    showModal = false;
  };
  const handleCancel = (): void => {
    showModal = false;
  };

  let newShown = false;
  const handleNew = () => {
    newShown = !newShown;
    if (editShown) editShown = false;
  };
  const handleEdit: (item: GroupItem) => void = (item: GroupItem) => {
    editShown = !editShown;
    editItem = item;
    if (newShown) newShown = false;
  };
  const importToggle = (): void => {
    importShown = !importShown;
    if (editShown) editShown = false;
    if (newShown) newShown = false;
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
      <button
        class="new-button"
        on:click={handleNew}
        disabled={newShown || editShown}
      >
        <img src="/icons/plus.svg" alt="Agregar" />
        Agregar nuevo grupo
      </button>

      <!-- Boton para importar de excel -->
      <button class="import-button" on:click={importToggle}>
        Importar desde Excel
      </button>

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
  {#if importShown}
    <ImportExcel defaultClass={ClassType.Groups} availableData={columns} />
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
      message={groupToDelete?.multiple
        ? `Seguro que deseas eliminar ${groupToDelete.multiple.length} elementos?`
        : `Estas seguro de eliminar este elemento?`}
    />
  {/if}
</section>
