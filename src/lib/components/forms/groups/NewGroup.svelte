<script lang="ts">
  import "$styles/form/editor.scss";

  import {
    addGroup,
    editGroup,
    type GroupItem,
  } from "$lib/modules/entities/groupsStore";
  import {
    loadSubjectsWithTeachers,
    subjects,
    subjectsWithTeachers,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TooltipIcon from "$lib/components/buttons/TooltipIcon.svelte";
  import ImportExcel from "$lib/components/utils/excel/ImportExcel.svelte";
  import { importStudentsFromXlsx } from "$lib/modules/entities/groupsStore";
  import { ClassType } from "$lib/utilities/helpers";
  import { addNotification } from "$lib/stores/notificationsStore";

  let g: GroupItem = {
    grade: null,
    group: "",
    career: "",
    students: null,
    max_modules_per_day: null,
  };

  let selectedSubjects: SubjectItem[] = [];
  let showSubjects: boolean = false;

  // Para editar se le pasa el item
  export let item: GroupItem | null = null;

  let showStudentImport = false;
  let lastNamesCombined = false;

  $: if (item) {
    initForm(item);
  }

  function initForm(item: any | null): void {
    if (item) {
      g.id = item.id;
      g.grade = item.grade;
      g.group = item.group;
      g.career = item.career;
      g.students = item.students;
      g.max_modules_per_day = item.max_modules_per_day;
      selectedSubjects = item.required_subjects;
    } else {
      selectedSubjects = [];
    }
  }

  onMount((): void => {
    loadSubjectsWithTeachers();
    listen("subjects_updated", async () => {
      await loadSubjectsWithTeachers();
    });
  });

  const handleSubmit = (): void => {
    if (item) {
      editGroup(g, selectedSubjects);
      addNotification({
        message: "Edicion del grupo exitoso: El grupo ha sido modificado en el sistema.",
        type: "success",
        timeout: 1500,
      });
    } else {
      addGroup(g, selectedSubjects);
      addNotification({
        message: "Registro exitoso: Grupo agregado al sistema.",
        type: "success",
        timeout: 1500,
      });
      g.grade = null;
      g.group = "";
      g.career = "";
      g.students = null;
    }
  };

  function toggleSelection(subject: SubjectItem): void {
    const index: number = selectedSubjects.findIndex(
      (s) => s.id === subject.id,
    );
    if (index >= 0) {
      // Si la materia ya esta seleccionada la quitamos
      selectedSubjects = selectedSubjects.filter((s) => s.id !== subject.id);
    } else {
      // Si no esta seleccionada la agregamos
      selectedSubjects = [...selectedSubjects, subject];
    }
  }
  const showSelectedSubjects = () => (showSubjects = !showSubjects);

  const handleStudentImport = async (event: CustomEvent) => {
    const { headerMappings, data } = event.detail;
    if (item?.id) {
      await importStudentsFromXlsx(
        headerMappings,
        data,
        item.id,
        lastNamesCombined,
      );
      showStudentImport = false;
    }
  };
</script>

<section class="form-editor">
  <h1>{item ? "Editar grupo existente" : "Generar nuevo grupo"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="grade">
        <img src="/icons/group.svg" alt="Icon" />
      </label>

      <input
        type="number"
        placeholder="* Grado"
        id="grade"
        bind:value={g.grade}
      />
      <div class="form-information-icon">
        <TooltipIcon description="Grado del grupo (ejemplo: 2)" />
      </div>
    </div>

    <div class="form-field">
      <label for="group"><img src="/icons/group.svg" alt="Icon" /></label>
      <input
        type="text"
        placeholder="* Grupo"
        id="group"
        bind:value={g.group}
      />
    </div>
    <div class="form-field">
      <label for="career"><img src="/icons/books.svg" alt="Icon" /></label>
      <input
        type="text"
        placeholder="Especialidad o carrera"
        id="career"
        bind:value={g.career}
      />
    </div>
    <div class="form-field">
      <label for="students"><img src="/icons/group.svg" alt="Icon" /></label>
      <input
        type="number"
        placeholder="Cantidad de alumnos (opcional)"
        id="career"
        bind:value={g.students}
      />
    </div>
    <!-- Aqui iran las materias que pre-asignadas a los grupos -->
    <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
    <div
      class="form-field"
      style="cursor: pointer;"
      on:click={showSelectedSubjects}
    >
      <label for="name"><img src="/icons/books.svg" alt="Materias" /></label>
      <!-- Muestra las materias seleccionadas -->
      {#if selectedSubjects.length > 0}
        {#each selectedSubjects as subject}
          <span class="form-name">{subject.name}</span>
        {/each}
      {:else}
        <span>Seleccione materias</span>
      {/if}
    </div>
    <!-- Lista de materias -->
    {#if showSubjects}
      <div class="subject-list">
        {#each $subjectsWithTeachers as subject}
          <div class="subject-item">
            <input
              type="checkbox"
              class="form-checkbox"
              id={subject.id.toString()}
              checked={selectedSubjects.some((s) => s.id === subject.id)}
              on:change={() => toggleSelection(subject)}
            />
            <label for={subject.id.toString()} class="tooltip">
              {subject.name}
              <span class="tooltiptext">
                {subject.assigned_teacher
                  ? subject.assigned_teacher.name +
                    " " +
                    subject.assigned_teacher.father_lastname
                  : "Sin maestro asignado"}
              </span>
            </label>
          </div>
        {/each}
      </div>
    {/if}

    <button class="form-button" on:click={handleSubmit}>
      {item ? "Editar grupo" : "Agregar grupo"}
    </button>
  </div>

  {#if item?.id}
    <div class="form-field">
      <button
        class="import-students-btn"
        on:click={() => {
          showStudentImport = true;
        }}
      >
        Importar estudiantes
      </button>
    </div>

    {#if showStudentImport}
      <div class="student-import-section">
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={lastNamesCombined} />
          Los apellidos están en la misma columna
        </label>

        <ImportExcel
          defaultClass={ClassType.Students}
          groupId={item?.id}
          availableData={[
            { name: "Nombre", key: "name", required: true },
            {
              name: "Apellido paterno",
              key: "father_lastname",
              required: true,
            },
            {
              name: !lastNamesCombined ? "Apellido materno" : "Apellidos",
              key: "mother_lastname",
            },
          ]}
        />
      </div>
    {/if}
  {/if}
</section>

<style lang="scss">
  .import-students-btn {
    background-color: #4caf50;
    color: white;
    padding: 8px 16px;
    margin-left: 4px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;

    &:hover {
      background-color: darken(#4caf50, 10%);
    }
  }

  .student-import-section {
    margin-top: 15px;
    padding: 15px;
    background-color: #f5f5f5;
    border-radius: 4px;
    color: black;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 15px;
    font-size: 14px;

    input[type="checkbox"] {
      width: 16px;
      height: 16px;
    }
  }
  @import "/src/styles/tooltip.scss";
</style>
