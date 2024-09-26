<script lang="ts">
  import { onMount } from "svelte";
  import { dndzone } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  import { subjectsWithTeachers, loadSubjectsWithTeachers, type SubjectItem } from "$lib/modules/entities/subjectsStore";
  import { getContrastColor } from "$lib/utilities/helpers";
  import { listen } from "@tauri-apps/api/event";

  let selectedSubject: SubjectItem | null = null;

  onMount(() => {
    loadSubjectsWithTeachers(); // Carga las materias desde la base de datos en rust

    // Escucha el evento para actualizar la vista de materias
    listen("subjects_with_teachers_updated", async () => {
      await loadSubjectsWithTeachers();
    });
  });

  /* Funcionalidad Drag and Drop */
  const flipDurationMs = 150;
  
  const handleConsider = (e: any) => {
    const updatedItems = e.detail.items;
    // Only update the subjects store if necessary
    subjectsWithTeachers.update((currentSubjects: SubjectItem[]) => {
      return currentSubjects.map((subject, index) => updatedItems[index] || subject);
    });
  };

  const handleFinalize = (e: any) => {
    const updatedItems = e.detail.items;
    subjectsWithTeachers.update((currentSubjects: SubjectItem[]) => {
      return currentSubjects.map((subject, index) => updatedItems[index] || subject);
    });
  };

  // Funcion para manejar el evento de click en una materia
  function handleClick(item: SubjectItem) {
    console.log("Clicked", item);
    selectedSubject = item;
  }
</script>

<div class="subjects-container">
  <section
    use:dndzone={{ items: $subjectsWithTeachers, flipDurationMs }}
    on:consider={handleConsider}
    on:finalize={handleFinalize}
    class="items"
  >
    {#each $subjectsWithTeachers as item (item.id)}
      <div
        class="subject"
        role="button"
        tabindex="0"
        style="background-color: {item.color}; color: {getContrastColor(item.color)}"
        animate:flip="{{duration: flipDurationMs}}"
        on:click={() => handleClick(item)}
        on:keydown={(e) => e.key === "Enter" && handleClick(item)}
      >
        {item.shorten}
      </div>
    {/each}
  </section>

  {#if selectedSubject}
    <div class="subjects-details">
      <!-- Show the color of the subject -->
      <div class="color" style="background-color: {selectedSubject.color}; color: {getContrastColor(selectedSubject.color)}">
        {selectedSubject.shorten}
      </div>
      <div class="details">
        <span>Nombre de la materia: {selectedSubject.name}</span>
        <span>Tipo: {selectedSubject.spec}</span>
        <!-- TODO -->
        <!-- <span>Profesor asignado: {selectedSubject.teacher}</span> -->
      </div>
    </div>
  {/if}
</div>
