<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";

  import NoResults from "$lib/components/utils/NoResults.svelte";
  import ImportExcel from "$lib/components/utils/excel/ImportExcel.svelte";
  import ConfirmModal from "$lib/components/buttons/ConfirmModal.svelte";
  import { ClassType } from "$lib/utilities/helpers";

  import NewSubject from "./NewSubject.svelte";
  import {
    subjects,
    loadSubjects,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";


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
    { name: "Modulos por semana", key: "required_modules" },
    { name: "Prioridad (1-5)", key: "priority" },
  ];

  let importShown: boolean = false;

  let editShown: boolean = false;
  let editItem: SubjectItem | null = null;
  const handleEdit = (item: SubjectItem): void => {
    editShown = true;
    editItem = item;
    if (newShown) newShown = false;
  };

  let showModal = false;
  // Manejamos eliminar por medio de un diccionario si hay varios seleccionados
  let subjectToDelete: { single?: SubjectItem; multiple?: number[] } | null = null;

  const actions = [
    {
      name: "Editar",
      action: (item: SubjectItem) => {
        handleEdit(item);
      },
    },
    {
      name: "Eliminar",
      action: (itemOrItems: SubjectItem| number[]): void => {
        if (Array.isArray(itemOrItems)) {
          subjectToDelete = { multiple: itemOrItems };
        } else {
          subjectToDelete = { single: itemOrItems };
        }
        showModal = true;
      },
    },
  ];

  const handleDelete = async (): Promise<void> => {
    if (!subjectToDelete) return;

    try {
      if (subjectToDelete.multiple) {
        console.log(subjectToDelete.multiple);
        await invoke("delete_subjects", { ids: subjectToDelete.multiple });
      } else if (subjectToDelete.single) {
        await invoke("delete_subject", { id: subjectToDelete.single.id });
      }
      loadSubjects();
      emit("subjects_updated");
    } catch (error) {
      console.error("Error deleting subjects:", error);
    }

    showModal = false;
  };
  const handleCancel = (): void => {
    showModal = false;
  };

  let newShown: boolean = false;
  const handleNew = (): void => {
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
    <NewSubject item={null} />
  {/if}
  {#if editShown}
    <NewSubject item={editItem} />
  {/if}
  {#if importShown}
    <ImportExcel defaultClass={ClassType.Subjects} availableData={columns} />
  {/if}
  <!-- Muestra la tabla de materias -->
  {#if $subjects.length === 0 && !newShown && !editShown}
    <NoResults />
  {:else}
    {#if search}
      <div class="search-results">
        Mostrando resultados de búsqueda para "{search}"
      </div>
      <TableComponent
        data={$subjects.filter((s) =>
          s.name.toLowerCase().includes(search.toLowerCase()),
        )}
        {columns}
        {actions}
      />
    {:else}
      <TableComponent data={$subjects} {columns} {actions} />
    {/if}

    <!-- Modal de confirmación para eliminar una materia -->
    <ConfirmModal
      isOpen={showModal}
      onConfirm={handleDelete}
      onCancel={handleCancel}
      message={subjectToDelete?.multiple
        ? `Seguro que deseas eliminar ${subjectToDelete.multiple.length} elementos?`
        : `Estas seguro de eliminar este elemento?`}
    />
  {/if}
</section>
