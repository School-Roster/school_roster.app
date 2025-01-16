<script lang="ts">
  import { onMount } from "svelte";
  import {
    subjectsWithTeachers,
    loadSubjectsWithTeachers,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";
  import { getContrastColor } from "$lib/utilities/helpers";
  import { listen } from "@tauri-apps/api/event";

  let selectedSubject: SubjectItem | null = null;
  let cleanup: () => void;

  onMount(() => {
    loadSubjectsWithTeachers(); // Carga las materias desde la base de datos en rust

    // Escucha el evento para actualizar la vista de materias
    (async () => {
      const listenerCleanup = await listen(
        "subjects_with_teachers_updated",
        async () => {
          await loadSubjectsWithTeachers();
        },
      );
      cleanup = listenerCleanup;
    })();

    return () => {
      cleanup?.();
    };
  });

  // Manejamos cuando el usuario agarra la materia
  function handleDragStart(e: DragEvent, subject: SubjectItem) {
    // Only send necessary data
    const dragData = {
      id: subject.id,
      shorten: subject.shorten,
      color: subject.color,
      teacherId: subject.assigned_teacher?.id,
    };

    e.dataTransfer?.setData("application/json", JSON.stringify(dragData));
  }

  // Memoize filtered subjects
  $: assignedSubjects = $subjectsWithTeachers.filter(
    (item) => item.assigned_teacher,
  );
</script>

<div class="subjects-container">
  <section class="items">
    {#each assignedSubjects as item (item.id + "-" + item.assigned_teacher?.id)}
      <div
        class="subject"
        role="button"
        tabindex="0"
        draggable="true"
        style="background-color: {item.color}; color: {getContrastColor(
          item.color,
        )}"
        on:dragstart={(e) => handleDragStart(e, item)}
        on:click={() => (selectedSubject = item)}
        on:keydown={(e) => e.key === "Enter" && (selectedSubject = item)}
      >
        {item.shorten}
      </div>
    {/each}
  </section>

  {#if selectedSubject}
    <div class="subjects-details">
      <div
        class="color"
        style="background-color: {selectedSubject.color}; color: {getContrastColor(
          selectedSubject.color,
        )}"
      >
        {selectedSubject.shorten}
      </div>
      <div class="details">
        <span>Nombre de la materia: {selectedSubject.name}</span>
        <span>Tipo: {selectedSubject.spec}</span>
        {#if selectedSubject.assigned_teacher}
          <span
            >Profesor asignado: {selectedSubject.assigned_teacher.name}
            {selectedSubject.assigned_teacher.father_lastname}</span
          >
        {/if}
      </div>
    </div>
  {/if}
</div>
