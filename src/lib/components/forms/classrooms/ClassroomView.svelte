<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";
  import ConfirmModal from "$lib/components/buttons/ConfirmModal.svelte";

  import NoResults from "$lib/components/utils/NoResults.svelte";
  import ImportExcel from "$lib/components/utils/ImportExcel.svelte";
  import { ClassType } from "$lib/utilities/helpers";

  import NewClassroom from "./NewClassroom.svelte";
  import {
    classrooms,
    loadClassrooms,
    type ClassroomItem,
  } from "$lib/modules/entities/classroomStore";

  let search = "";
  let filter: string = "";

  // Carga las aulas desde la base de datos en rust
  onMount(loadClassrooms);

  // Columnas de la tabla (key es el nombre de la propiedad en la interfaz)
  const columns = [
    { name: "ID", key: "id" },
    { name: "Edificio", key: "building_id" },
    { name: "Aula", key: "building_number" },
    { name: "Tipo de aula", key: "building_type" },
    { name: "Capacidad del aula", key: "capacity" },
  ];

  let editShown: boolean = false;
  let importShown: boolean = false;
  let editItem: ClassroomItem | null = null;
  const handleEdit = (item: ClassroomItem) => {
    editShown = true;
    editItem = item;
    if (newShown) newShown = false;
  };

  let showModal: boolean = false;
  let classroomToDelete: {
    single?: ClassroomItem;
    multiple?: number[];
  } | null = null;

  const actions = [
    {
      name: "Editar",
      action: (item: ClassroomItem): void => {
        handleEdit(item);
      },
    },
    {
      name: "Eliminar",
      action: (itemOrItems: ClassroomItem | number[]) => {
        if (Array.isArray(itemOrItems)) {
          classroomToDelete = { multiple: itemOrItems };
        } else {
          classroomToDelete = { single: itemOrItems };
        }
        showModal = true;
      },
    },
  ];

  const handleDelete = async (): Promise<void> => {
    if (!classroomToDelete) return;

    try {
      if (classroomToDelete.multiple) {
        console.log(classroomToDelete.multiple);
        await invoke("delete_classrooms", { ids: classroomToDelete.multiple });
      } else if (classroomToDelete.single) {
        await invoke("delete_classroom", { id: classroomToDelete.single.id });
      }
      loadClassrooms();
      emit("classrooms_updated");
    } catch (error) {
      console.error("Error deleting classroom:", error);
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
  const importToggle = (): void => {
    importShown = !importShown;
    if (editShown) editShown = false;
    if (newShown) newShown = false;
  };
</script>

<section class="form-container">
  <div class="title">
    <img src="/icons/door.svg" alt="Aulas" />
    <span>Aulas</span>
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
        Agregar nueva aula
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
    <NewClassroom item={null} />
  {/if}
  {#if editShown}
    <NewClassroom item={editItem} />
  {/if}
  {#if importShown}
    <ImportExcel defaultClass={ClassType.Classrooms} availableData={columns} />
  {/if}
  <!-- Muestra la tabla -->
  {#if $classrooms.length === 0 && !newShown && !editShown}
    <NoResults />
  {:else}
    {#if search}
      <div class="search-results">
        Mostrando resultados de búsqueda para "{search}"
      </div>
      <TableComponent
        data={$classrooms.filter((s) => {
          switch (filter) {
            case "ID":
              return s.id.toString().includes(search);
            case "Edificio":
              return s.building_id.toLowerCase().includes(search.toLowerCase());
            case "Aula":
              return s.building_number.toString().includes(search);
            case "Tipo de aula":
              return s.building_type
                .toLowerCase()
                .includes(search.toLowerCase());
            case "Capacidad":
              return s.capacity.toString().includes(search);
            default:
              return (
                s.building_id.toLowerCase().includes(search.toLowerCase()) ||
                s.building_number.toString().includes(search) ||
                s.building_type.toLowerCase().includes(search.toLowerCase()) ||
                s.capacity.toString().includes(search)
              );
          }
        })}
        {columns}
        {actions}
      />
    {:else}
      <TableComponent data={$classrooms} {columns} {actions} />
    {/if}

    <!-- Modal de confirmación para eliminar una materia -->
    <ConfirmModal
      isOpen={showModal}
      onConfirm={handleDelete}
      onCancel={handleCancel}
      message={classroomToDelete?.multiple
        ? `Seguro que deseas eliminar ${classroomToDelete.multiple.length} elementos?`
        : `Estas seguro de eliminar este elemento?`}
    />
  {/if}
</section>
