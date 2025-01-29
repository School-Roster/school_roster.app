<script lang="ts">
  import "$styles/grid.scss";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
  import { getContrastColor } from "$lib/utilities/helpers";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { writable, type Writable } from "svelte/store";

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

  // Utilizamos Map() para mantener O(1)
  // let assignments = new Map();
  const assignmentsStore: Writable<Map<any, any>> = writable(new Map());

  onMount(() => {
    loadGroups();
    // Carga los grupos de nuevo en caso de actualizados
    listen("groups_updated", async () => {
      await loadGroups();
    });
  });

  // Funcion para cuando una materia entra en un modulo
  function handleDragOver(e: DragEvent): void {
    e.preventDefault();
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

  // Funcion para cuando se suelta una materia en el modulo
  function handleDrop(
    e: DragEvent,
    groupId: number,
    day: string,
    moduleIndex: number,
  ) {
    e.preventDefault();
    const target = e.target as HTMLElement;
    target.classList.remove("drag-over");

    const subjectData = e.dataTransfer?.getData("application/json");
    if (!subjectData) return;

    const subject = JSON.parse(subjectData);
    // Creamos una llave unica para el assign
    const key = `${groupId}-${day}-${moduleIndex}`;

    // assignments.set(key, subject);
    // assignments = assignments; // Maneja reactividad
    assignmentsStore.update((currentMap: any) => {
      const newMap = new Map(currentMap);
      newMap.set(key, subject);
      return newMap;
    });
    // console.log("Updated assignments:", Array.from(assignments.entries())); // Debug log
    // getAssignment(groupId, day, moduleIndex);
  }

  // Manera eficiente de conseguir los 'assigns'
  function getAssignment(groupId: number, day: string, moduleIndex: number) {
    const key = `${groupId}-${day}-${moduleIndex}`;
    let assignment;
    assignmentsStore.subscribe((map: any) => {
      assignment = map.get(key);
    })();
    return assignment;
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
                  {@const assignment = getAssignment(
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
                    on:drop={(e) => handleDrop(e, group.id, day, moduleIndex)}
                  >
                    {#if assignment}
                      <div
                        class="subject-pill"
                        style="background-color: {assignment.color}; color: {getContrastColor(
                          assignment.color,
                        )}"
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
