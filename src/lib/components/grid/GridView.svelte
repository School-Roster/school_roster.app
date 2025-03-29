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
    import { loadSubjectsWithTeachers } from "$lib/modules/entities/subjectsStore";

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

  // Funcion para cuando una materia entra en un modulo
  function handleDragOver(e: DragEvent): void {
    e.preventDefault();
    // Explicitly set dropEffect to show valid drop target
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'copy';
    }
    
    const target = e.target as HTMLElement;
    if (target.classList.contains("module-cell")) {
      target.classList.add("drag-over");
    }
  }

  // Funcion para cuando una materia abandona el modulo
  function handleDragLeave(e: DragEvent): void {
    e.preventDefault();
    const target = e.target as HTMLElement;
    if (target.classList.contains("module-cell")) {
      target.classList.remove("drag-over");
    }
  }
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
              {#key $assignmentsStore}
                {#if true}
                  {@const assignment = getLocalAssignment(
                    group.id,
                    day,
                    moduleIndex,
                  )}
                  <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
                  <div
                    class="module-cell"
                    class:has-subject={assignment}
                    on:dragover={handleDragOver}
                    on:dragleave={handleDragLeave}
                    on:drop={(e) =>
                      handleAssignDrop(e, group.id, day, moduleIndex)}
                  >
                    {#if assignment}
                      <div
                        class="subject-pill"
                        style="background-color: {assignment.color || 'black'}; color: {getContrastColor(
                          assignment.color || 'black',
                        )}"
                        on:mousedown={(e) => handleAssignClick(e, assignment.id)}
                      >
                        {assignment.shorten}
                      </div>
                    {/if}
                  </div>
                {/if}
              {/key}
            {/each}
          </div>
        {/each}
      </div>
    {/each}
  </div>
</section>
