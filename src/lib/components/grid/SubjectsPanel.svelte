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

    // Windows compatibility: Add fallback manual drag handling
    const subjects = document.querySelectorAll('.subject');
    subjects.forEach(subject => {
      subject.addEventListener('mousedown', (e) => {
        if (!e.target.draggable) return;
        
        // Force draggable state
        const evt = document.createEvent("HTMLEvents");
        evt.initEvent("dragstart", true, true);
        e.target.dispatchEvent(evt);
      });
    });

    return () => {
      cleanup?.();
    };
  });

  // Manejamos cuando el usuario agarra la materia
  function handleDragStart(e: DragEvent, subject: SubjectItem) {
    // First, create the dragData object
    const dragData = {
      id: subject.id,
      shorten: subject.shorten,
      color: subject.color,
      teacherId: subject.assigned_teacher?.id,
    };
    
    // Force cursor style (helps on Windows)
    document.body.style.cursor = 'grabbing';
    
    // Use a more universally supported approach for drag effect
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = 'copy';
      
      // Windows compatibility: Try both MIME types
      e.dataTransfer.setData("text/plain", JSON.stringify(dragData));
      e.dataTransfer.setData("application/json", JSON.stringify(dragData));
      
      // For Windows desktop apps, setting a small image sometimes helps
      try {
        const ghostElement = document.createElement('div');
        ghostElement.innerHTML = subject.shorten;
        ghostElement.style.backgroundColor = subject.color;
        ghostElement.style.color = getContrastColor(subject.color);
        ghostElement.style.padding = '10px';
        ghostElement.style.borderRadius = '4px';
        ghostElement.style.position = 'absolute';
        ghostElement.style.top = '-1000px';
        document.body.appendChild(ghostElement);
        
        e.dataTransfer.setDragImage(ghostElement, 10, 10);
        
        // Clean up after the drag operation
        setTimeout(() => {
          document.body.removeChild(ghostElement);
        }, 0);
      } catch (err) {
        console.error('Error setting drag image:', err);
      }
    }
    
    // Add a class to indicate dragging state
    const elem = e.currentTarget as HTMLElement;
    if (elem) {
      elem.classList.add('dragging');
    }
  }

  function handleDragEnd(e: DragEvent) {
    // Reset cursor
    document.body.style.cursor = '';
    
    // Clean up any visual state
    const elem = e.currentTarget as HTMLElement;
    if (elem) {
      elem.classList.remove('dragging');
    }
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
        on:dragend={handleDragEnd}
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
