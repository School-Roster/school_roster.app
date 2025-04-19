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
  import { selectedGroup } from "$lib/modules/entities/selectedGroup";

  export let days: string[] = [
    "Lunes",
    "Martes",
    "Miércoles",
    "Jueves",
    "Viernes",
  ];
  export let modulesPerDay: number = 9;

  // Cargar datos iniciales cuando el componente se monta
  onMount(async (): Promise<void> => {
    await loadGroups();
    await loadAssignments();
    listen("groups_updated", async () => {
      await loadGroups();
    });
    listen("teachers_updated", async () => {
      await loadSubjectsWithTeachers();
      await loadAssignments();
    });
  });

  // Al hacer clic en un grupo, actualizamos el estado seleccionado
  function handleGroupClick(groupId: number): void {
    selectedGroup.set(groupId);
  }

  // Manejo de eventos personalizados de drop
  function handleCustomDrop(e: CustomEvent): void {
    const { subject, groupId, day, moduleIndex } = e.detail;
    handleAssignDrop(
      {
        preventDefault: () => {},
        subject,
        data: subject,
      },
      groupId,
      day,
      moduleIndex
    );
  }

  function handleDragOver(target: HTMLElement): void {
    target.classList.add("drag-over");
  }

  function handleDragLeave(target: HTMLElement): void {
    target.classList.remove("drag-over");
  }

  // Se asegura de agregar y eliminar el listener para el evento 'custom:drop'
  onMount(() => {
    const handleDrop = (e: Event) => handleCustomDrop(e as CustomEvent);
    document.addEventListener("custom:drop", handleDrop);

    return () => {
      document.removeEventListener("custom:drop", handleDrop);
    };
  });
</script>

<section class="schedule-grid">
  <!-- Header con los días y los módulos -->
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

  <!-- Grupos y módulos -->
  <div class="grid-content">
    {#each $groups as group}
      <div class="group-row">
        <div
          class="group-cell"
          style="cursor: pointer"
          on:click={() => handleGroupClick(group.id)}
          class:selected={group.id === $selectedGroup}
        >
          {group.grade}{group.group}
        </div>

        {#each days as day}
          <div class="day-modules">
            {#each Array(modulesPerDay) as _, moduleIndex}
              {#key $assignmentsStore}
                {#if true}
                  {@const assignment = getLocalAssignment(
                    group.id,
                    day,
                    moduleIndex
                  )}
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
                        style="background-color: {assignment.color ||
                          'black'}; color: {getContrastColor(
                          assignment.color || 'black'
                        )}"
                        on:mousedown={(e) =>
                          handleAssignClick(e, assignment.id)}
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
