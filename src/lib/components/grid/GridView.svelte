<script lang="ts">
  import "$styles/grid.scss";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
  import { getContrastColor } from "$lib/utilities/helpers";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import {
    assignmentsStore,
    loadAssignments,
    getLocalAssignment,
    handleAssignDrop,
    handleAssignClick,
  } from "$lib/modules/entities/assignments";
  import { subjectsWithTeachers, loadSubjectsWithTeachers, type SubjectItem } from "$lib/modules/entities/subjectsStore";
  import { commitChange, findDropTarget } from "$lib/stores/AssignmentUndoRedo";

  // TODO: Los dias se registraran en la ventana de configuracion
  export let days: string[] = [
    "Lunes",
    "Martes",
    "Miercoles",
    "Jueves",
    "Viernes",
  ];

  // TODO: Por ahora los modulos viven aqui, despues los sacamos de la informacion
  //       registrada en configuracion
  export let modulesPerDay: number = 9;

  onMount(async (): Promise<void> => {
    await loadGroups();
    await loadAssignments(); // Llama a base de datos cuando se inicia el programa
    // Carga los grupos de nuevo en caso de actualizados
    listen("groups_updated", async () => {
      await loadGroups();
    });
    // Carga si las materias son actualizadas (con profesores)
    listen("teachers_updated", async () => {
      await loadSubjectsWithTeachers();
      await loadAssignments(); // Llama a base de datos cuando se inicia el programa
    });
  });

  // Maneja el evento fuera de HTML5 como custom event
  function handleCustomDrop(e: CustomEvent): void {
    const { subject, groupId, day, moduleIndex } = e.detail;
    
    // Llama el handler existente con los datos necesarios
    handleAssignDrop(
      { 
        preventDefault: () => {}, 
        subject: subject,   // Pasa la materia directamente
        data: subject       // Pasamos 'data' para mayor flexibilidad en el codigo
      }, 
      groupId, 
      day, 
      moduleIndex
    );
  }

  function handleMiddleClick(e: MouseEvent, assignment: undefined, subject: SubjectItem): void {
   
    const dropTarget = findDropTarget(e)
    const groupId = dropTarget?.getAttribute("data-group-id");
    const day = dropTarget?.getAttribute("data-day");
    const moduleIndex = dropTarget?.getAttribute("data-module-index")

    if (!groupId || !day || !moduleIndex) return

    if (groupId && day && moduleIndex) {
      handleAssignClick(e, assignment);
      commitChange(
        {
          action: "delete",
          day,
          groupId: parseInt(groupId, 10),
          moduleIndex: parseInt(moduleIndex, 10),
          subjectId: subject.id!,
          teacherId: subject.assigned_teacher?.id!
        }
      )
    }
  }

  function handleDragOver(target: HTMLElement): void{
    target.classList.add("drag-over");
  }

  function handleDragLeave(target: HTMLElement): void {
    target.classList.remove("drag-over");
  }

  onMount(() => {
    document.addEventListener('custom:drop', handleCustomDrop as EventListener);
    
    return () => {
      document.removeEventListener('custom:drop', handleCustomDrop as EventListener);
    };
  });

  $: assignedSubjects = $subjectsWithTeachers.filter(
    (item) => item.assigned_teacher,
  );

</script>

<section class="schedule-grid">
  <!-- Header con los dias y los modulos -->
  <div class="header-row">
    <div class="corner-cell">Grupos</div>
    {#each days as day}
      <div class="day-column">
        <div class="day-header">{day}</div>
        <div class="modules-header">
          {#each Array(modulesPerDay) as _, index}
            <div class="module-label">{index + 1}</div>
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <!-- Grupos y los modulos -->
  <div class="grid-content">
    {#each $groups as group}
      <div class="group-row">
        <div class="group-cell">{group.grade}{group.group}</div>
        {#each days as day}
          <div class="day-modules">
            {#each Array(modulesPerDay) as _, moduleIndex}
              {#each assignedSubjects as subject}
                {#key $assignmentsStore}
                  {#if true}
                    {@const assignment = getLocalAssignment(
                      group.id,
                      day,
                      moduleIndex,
                    )}
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <div
                      class="module-cell"
                      class:has-subject={assignment}
                      data-group-id={group.id}
                      data-day={day}
                      data-module-index={moduleIndex}
                      on:mouseenter={(e) => handleDragOver(e.currentTarget)}
                      on:mouseleave={(e) => handleDragLeave(e.currentTarget)}
                    >
                      {#if assignment}
                        <div
                          class="subject-pill"
                          style="background-color: {assignment.color || 'black'}; color: {getContrastColor(
                            assignment.color || 'black',
                          )}"
                          on:mousedown={(e) => handleMiddleClick(e, assignment.id, subject)}
                        >
                          {assignment.shorten}
                        </div>
                      {/if}
                    </div>
                  {/if}
                {/key}
              {/each}
            {/each}
          </div>
        {/each}
      </div>
    {/each}
  </div>
</section>