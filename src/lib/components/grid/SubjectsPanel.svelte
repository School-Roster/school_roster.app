<script lang="ts">
  import { onMount } from "svelte";
  import {
    subjectsWithTeachers,
    loadSubjectsWithTeachers,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";
  import { getContrastColor } from "$lib/utilities/helpers";
  import { listen } from "@tauri-apps/api/event";
  import { saveAssignment } from "$lib/modules/entities/assignments";

  let selectedSubject: SubjectItem | null = null;
  let cleanup: () => void;
  
  // Variables for custom drag and drop
  let isDragging = false;
  let draggedSubject: SubjectItem | null = null;
  let ghostElement: HTMLElement | null = null;

  onMount(() => {
    loadSubjectsWithTeachers();

    (async () => {
      const listenerCleanup = await listen(
        "subjects_with_teachers_updated",
        async () => {
          await loadSubjectsWithTeachers();
        },
      );
      cleanup = listenerCleanup;
    })();

    // Agrega el evento global para los handlers
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);

    return (): void => {
      cleanup?.();
      // Limpia las funciones
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      removeGhostElement();
    };
  });

  function handleMouseDown(e: MouseEvent, subject: SubjectItem) {
    if (e.button !== 0) return;
    
    e.preventDefault();
    
    isDragging = true;
    draggedSubject = subject;
    
    // Create ghost element
    createGhostElement(e, subject);
  }

  // Handle mouse movement during drag
  function handleMouseMove(e: MouseEvent) {
    if (!isDragging || !ghostElement) return;
    
    // Move ghost element with cursor
    ghostElement.style.left = `${e.clientX + 10}px`;
    ghostElement.style.top = `${e.clientY + 10}px`;
  }

  // End dragging and handle drop
  function handleMouseUp(e: MouseEvent) {
    if (!isDragging || !draggedSubject) return;
    
    // Find if we're over a valid drop target
    const dropTarget = findDropTarget(e);
    
    if (dropTarget && draggedSubject) {
      // Get drop target information
      const groupId = dropTarget.getAttribute('data-group-id');
      const day = dropTarget.getAttribute('data-day');
      const moduleIndex = dropTarget.getAttribute('data-module-index');
      
      // Call saveAssignment directly
      if (groupId && day && moduleIndex) {
        saveAssignment(
          parseInt(groupId, 10),
          day,
          parseInt(moduleIndex, 10),
          draggedSubject.id,
          draggedSubject.assigned_teacher?.id
        );
        
        // Provide visual feedback
        dropTarget.classList.add('flash-highlight');
        setTimeout(() => {
          dropTarget.classList.remove('flash-highlight');
        }, 300);
      }
    }
    
    // Reset drag state
    isDragging = false;
    draggedSubject = null;
    removeGhostElement();
  }

  // Find valid drop target under cursor
  function findDropTarget(e: MouseEvent): HTMLElement | null {
    // Get all elements at the current mouse position
    const elements = document.elementsFromPoint(e.clientX, e.clientY);
    
    // Find the first element with class 'module-cell'
    for (const el of elements) {
      if (el.classList.contains('module-cell')) {
        return el as HTMLElement;
      }
    }
    
    return null;
  }

  // Create visual ghost element
  function createGhostElement(e: MouseEvent, subject: SubjectItem) {
    removeGhostElement(); // Remove any existing ghost
    
    ghostElement = document.createElement('div');
    ghostElement.className = 'subject-ghost';
    ghostElement.textContent = subject.shorten;
    ghostElement.style.backgroundColor = subject.color;
    ghostElement.style.color = getContrastColor(subject.color);
    ghostElement.style.position = 'fixed';
    ghostElement.style.pointerEvents = 'none';
    ghostElement.style.padding = '8px';
    ghostElement.style.borderRadius = '4px';
    ghostElement.style.boxShadow = '0 2px 10px rgba(0,0,0,0.2)';
    ghostElement.style.zIndex = '9999';
    ghostElement.style.opacity = '0.8';
    
    // Position at cursor
    ghostElement.style.left = `${e.clientX + 10}px`;
    ghostElement.style.top = `${e.clientY + 10}px`;
    
    document.body.appendChild(ghostElement);
  }

  // Remove ghost element
  function removeGhostElement() {
    if (ghostElement && ghostElement.parentNode) {
      ghostElement.parentNode.removeChild(ghostElement);
      ghostElement = null;
    }
  }

  // Memoize filtered subjects
  $: assignedSubjects = $subjectsWithTeachers.filter(
    (item) => item.assigned_teacher,
  );
</script>

<div class="subjects-container">
  <section class="subjects-items">
    {#each assignedSubjects as item (item.id + "-" + item.assigned_teacher?.id)}
      <div
        class="subject"
        role="button"
        tabindex="0"
        style="background-color: {item.color}; color: {getContrastColor(
          item.color,
        )}"
        on:mousedown={(e) => handleMouseDown(e, item)}
        on:click={() => (selectedSubject = item)}
        on:keydown={(e) => e.key === "Enter" && (selectedSubject = item)}
        class:dragging={isDragging && draggedSubject?.id === item.id}
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
