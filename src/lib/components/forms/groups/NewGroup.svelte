<script lang="ts">
  import {
    addGroup,
    editGroup,
    type GroupItem,
  } from "$lib/modules/entities/groupsStore";
  import {
    loadSubjects,
    subjects,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  // Variables locales
  let grade: number;
  let group: string;
  let career: string;
  let students: number;

  let selectedSubjects: SubjectItem[] = [];
  let showSubjects: boolean = false;

  // Para editar se le pasa el item
  export let item: GroupItem | null = null;

  onMount(() => {
    loadSubjects();
    listen("subjects_updated", async () => {
      await loadSubjects();
    });
  });

  const handleSubmit = (): void => {
    if (item) {
      editGroup(item, selectedSubjects);
    } else {
      addGroup(grade, group, career, students, selectedSubjects);
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
</script>

<section class="form-editor">
  <h1>{item ? "Editar grupo existente" : "Generar nuevo grupo"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="grade"><img src="/icons/group.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="text"
          placeholder="* Grado"
          id="grade"
          bind:value={item.grade}
        />
      {:else}
        <input
          type="number"
          placeholder="* Grado"
          id="grade"
          bind:value={grade}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="group"><img src="/icons/group.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="text"
          placeholder="* Grupo"
          id="group"
          bind:value={item.group}
        />
      {:else}
        <input
          type="text"
          placeholder="* Grupo"
          id="group"
          bind:value={group}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="career"><img src="/icons/books.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="text"
          placeholder="Especialidad o carrera"
          id="career"
          bind:value={item.career}
        />
      {:else}
        <input
          type="text"
          placeholder="Especialidad o carrera"
          id="career"
          bind:value={career}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="students"><img src="/icons/group.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="number"
          placeholder="Cantidad de alumnos (opcional)"
          id="career"
          bind:value={item.students}
        />
      {:else}
        <input
          type="number"
          placeholder="Cantidad de alumnos (opcional)"
          id="career"
          bind:value={students}
        />
      {/if}
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
        {#each $subjects as subject}
          <div class="subject-item">
            <input
              type="checkbox"
              class="form-checkbox"
              id={subject.id.toString()}
              checked={selectedSubjects.some((s) => s.id === subject.id)}
              on:change={() => toggleSelection(subject)}
            />
            <label for={subject.id.toString()}>{subject.name}</label>
          </div>
        {/each}
      </div>
    {/if}

    <button class="form-button" on:click={handleSubmit}>
      {item ? "Editar grupo" : "Agregar grupo"}
    </button>
  </div>
</section>
