<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { emit, listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TableComponent from "$lib/components/tables/TableComponent.svelte";
  import SearchAnimation from "$lib/components/buttons/SearchAnimation.svelte";
  import FilterAnimation from "$lib/components/buttons/FilterButton.svelte";
  import ConfirmModal from "$lib/components/buttons/ConfirmModal.svelte";
  import { ClassType } from "$lib/utilities/helpers";

  import {
    teachers,
    loadTeachers,
    type TeacherItem,
  } from "$lib/modules/entities/teachersStore";
  import NewTeacher from "./NewTeacher.svelte";
  import { loadSubjects } from "$lib/modules/entities/subjectsStore";

  import NoResults from "$lib/components/utils/NoResults.svelte";
  import ImportExcel from "$lib/components/utils/excel/ImportExcel.svelte";
  import { loadAssignments } from "$lib/modules/entities/assignments";

  let search: string = "";
  let filter: string = "";

  let importShown: boolean = false;
  let editShown: boolean = false;
  let editItem: any | null = null;

  let newShown: boolean = false;

  let showModal: boolean = false;
  let teacherToDelete: {
    single?: TeacherItem;
    multiple?: number[];
  } | null = null;

  // Carga las materias desde la base de datos en rust
  onMount((): void => {
    loadTeachers();
    loadSubjects();

    listen("teachers_updated", async () => {
      await loadTeachers();
    });
  });

  // Columnas de la tabla (key es el nombre de la propiedad en la interfaz)
  const columns = [
    { name: "ID", key: "id" },
    { name: "Nombre", key: "name" },
    { name: "Apellido paterno", key: "father_lastname" },
    { name: "Apellido materno", key: "mother_lastname" },
    { name: "Correo", key: "email" },
    { name: "Teléfono", key: "phone" },
    { name: "Titulo", key: "degree" },
    { name: "Horas (comision)", key: "commisioned_hours" },
    { name: "Horas (activas)", key: "active_hours" },
    { name: "Rendimiento", key: "performance" },
    { name: "Materias", key: "assigned_subjects" },
  ];

  const handleChange = (): void => {
    newShown = !newShown;
    if (editShown) editShown = false;
  };

  const actions = [
    {
      name: "Editar",
      action: (item: TeacherItem): void => {
        editShown = true;
        editItem = item;
        if (newShown) newShown = false;
      },
    },
    {
      name: "Eliminar",
      action: (itemOrItems: TeacherItem | number[]): void => {
        if (Array.isArray(itemOrItems)) {
          teacherToDelete = { multiple: itemOrItems };
        } else {
          teacherToDelete = { single: itemOrItems };
        }
        showModal = true;
      },
    },
  ];

  const handleDelete = async (): Promise<void> => {
    if (!teacherToDelete) return;
    try {
      if (teacherToDelete.multiple) {
        await invoke("delete_teachers", { ids: teacherToDelete.multiple });
      } else if (teacherToDelete.single) {
        await invoke("delete_teacher", {
          teacher_id: teacherToDelete.single.id,
        });
      }
      loadTeachers();
      loadAssignments();
      await emit("teachers_updated");
    } catch (err) {
      console.log(err);
    }
    showModal = false;
  };
  const handleCancel = (): void => {
    showModal = false;
  };
  const importToggle = (): void => {
    importShown = !importShown;
    if (editShown) editShown = false;
    if (newShown) newShown = false;
  };
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
      <button class="new-button" on:click={handleChange}>
        <img src="/icons/plus.svg" alt="Agregar materia" />
        Agregar un nuevo profesor
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
      <!-- Botón para filtrar la tabla por opciones -->
      <FilterAnimation {columns} bind:filter />
      <!-- Filtro de búsqueda -->
      <SearchAnimation bind:search />
    </div>
  </div>
  <!-- Muestra el formulario de nueva materia si se presiona el botón -->
  {#if newShown}
    <NewTeacher />
  {/if}
  {#if editShown}
    <NewTeacher item={editItem} />
  {/if}
  {#if importShown}
    <ImportExcel defaultClass={ClassType.Teachers} availableData={columns} />
  {/if}
  <!-- Muestra la tabla de profesores -->
  {#if $teachers.length === 0 && !newShown && !editShown}
    <NoResults />
  {:else if search}
    <div class="search-results">
      <span>Mostrando resultados de búsqueda "{search}""</span>
    </div>
    <TableComponent
      data={$teachers.filter((s) => {
        switch (filter) {
          case "ID":
            return s.id.toString().includes(search);
          case "Nombre":
            return s.name.toLowerCase().includes(search.toLowerCase());
          case "Apellido paterno":
            return s.father_lastname
              .toLowerCase()
              .includes(search.toLowerCase());
          case "Apellido materno":
            return s.mother_lastname
              .toLowerCase()
              .includes(search.toLowerCase());
          case "Correo":
            return s.email.toLowerCase().includes(search.toLowerCase());
          case "Teléfono":
            return s.phone.toLowerCase().includes(search.toLowerCase());
          case "Titulo":
            return s.degree.toLowerCase().includes(search.toLowerCase());
          case "Horas (comosion)":
            return s.commisioned_hours.toString().includes(search);
          case "Horas (activas)":
            return s.active_hours.toString().includes(search);
          case "Rendimiento":
            return s.performance.toString().includes(search);
          default:
            return (
              s.id.toString().includes(search) ||
              s.name.toLowerCase().includes(search.toLowerCase()) ||
              s.father_lastname.toLowerCase().includes(search.toLowerCase()) ||
              s.mother_lastname.toLowerCase().includes(search.toLowerCase())
            );
        }
      })}
      {columns}
      {actions}
    />
  {:else}
    <TableComponent data={$teachers} {columns} {actions} />
  {/if}

  <!-- Modal para eliminar profesor -->
  <ConfirmModal
    isOpen={showModal}
    onConfirm={handleDelete}
    onCancel={handleCancel}
    message={teacherToDelete?.multiple
      ? `Seguro que deseas eliminar ${teacherToDelete.multiple.length} elementos?`
      : `Estas seguro de eliminar este elemento?`}
  />
</section>
