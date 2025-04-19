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
  import { selectedGroup } from "$lib/modules/entities/selectedGroup";
  import { groups } from "$lib/modules/entities/groupsStore";

  let selectedSubject: SubjectItem | null = null;
  let cleanup: () => void;

  // Variables para el drag and drop personalizado
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
        }
      );
      cleanup = listenerCleanup;
    })();

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);

    return (): void => {
      cleanup?.();
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
      removeGhostElement();
    };
  });

  function handleMouseDown(e: MouseEvent, subject: SubjectItem) {
    if (e.button !== 0) return;
    e.preventDefault();
    isDragging = true;
    draggedSubject = subject;
    createGhostElement(e, subject);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging || !ghostElement) return;
    ghostElement.style.left = `${e.clientX + 10}px`;
    ghostElement.style.top = `${e.clientY + 10}px`;
  }

  function handleMouseUp(e: MouseEvent) {
    if (!isDragging || !draggedSubject) return;
    const dropTarget = findDropTarget(e);
    if (dropTarget && draggedSubject) {
      const groupId = dropTarget.getAttribute("data-group-id");
      const day = dropTarget.getAttribute("data-day");
      const moduleIndex = dropTarget.getAttribute("data-module-index");
      if (groupId && day && moduleIndex) {
        saveAssignment(
          parseInt(groupId, 10),
          day,
          parseInt(moduleIndex, 10),
          draggedSubject.id,
          draggedSubject.assigned_teacher?.id
        );
        dropTarget.classList.add("flash-highlight");
        setTimeout(() => {
          dropTarget.classList.remove("flash-highlight");
        }, 300);
      }
    }
    isDragging = false;
    draggedSubject = null;
    removeGhostElement();
  }

  function findDropTarget(e: MouseEvent): HTMLElement | null {
    const elements = document.elementsFromPoint(e.clientX, e.clientY);
    for (const el of elements) {
      if (el.classList.contains("module-cell")) {
        return el as HTMLElement;
      }
    }
    return null;
  }

  function createGhostElement(e: MouseEvent, subject: SubjectItem) {
    removeGhostElement();
    ghostElement = document.createElement("div");
    ghostElement.className = "subject-ghost";
    ghostElement.textContent = subject.shorten;
    ghostElement.style.backgroundColor = subject.color;
    ghostElement.style.color = getContrastColor(subject.color);
    ghostElement.style.position = "fixed";
    ghostElement.style.pointerEvents = "none";
    ghostElement.style.padding = "8px";
    ghostElement.style.borderRadius = "4px";
    ghostElement.style.boxShadow = "0 2px 10px rgba(0,0,0,0.2)";
    ghostElement.style.zIndex = "9999";
    ghostElement.style.opacity = "0.8";
    ghostElement.style.left = `${e.clientX + 10}px`;
    ghostElement.style.top = `${e.clientY + 10}px`;
    document.body.appendChild(ghostElement);
  }

  function removeGhostElement() {
    if (ghostElement && ghostElement.parentNode) {
      ghostElement.parentNode.removeChild(ghostElement);
      ghostElement = null;
    }
  }

  // Filtrado de materias:
  // Si hay un grupo seleccionado, buscamos ese grupo en $groups.
  // Si el grupo tiene materias pre-asignadas (required_subjects con elementos),
  // filtramos las materias disponibles para mostrar solo las que coinciden.
  // De lo contrario (sin preasignación), se muestran todas las materias disponibles.
  $: assignedSubjects = $selectedGroup
    ? (() => {
        const group = $groups.find((g) => g.id === $selectedGroup);
        if (group) {
          if (group.required_subjects && group.required_subjects.length > 0) {
            const requiredIds = group.required_subjects.map((subj) => subj.id);
            return $subjectsWithTeachers.filter((item) =>
              requiredIds.includes(item.id)
            );
          } else {
            return $subjectsWithTeachers.filter(
              (item) => !!item.assigned_teacher
            );
          }
        }
        return [];
      })()
    : $subjectsWithTeachers.filter((item) => !!item.assigned_teacher);
</script>

<div class="subjects-container">
  <section class="items">
    {#each assignedSubjects as item (item.id + "-" + item.assigned_teacher?.id)}
      <div
        class="subject"
        role="button"
        tabindex="0"
        style="background-color: {item.color}; color: {getContrastColor(
          item.color
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
          selectedSubject.color
        )}"
      >
        {selectedSubject.shorten}
      </div>
      <div class="details">
        <span>Nombre de la materia: {selectedSubject.name}</span>
        <span>Tipo: {selectedSubject.spec}</span>
        {#if selectedSubject.assigned_teacher}
          <span>
            Profesor asignado: {selectedSubject.assigned_teacher.name}
            {selectedSubject.assigned_teacher.father_lastname}
          </span>
        {/if}
      </div>
    </div>
  {/if}
</div>
