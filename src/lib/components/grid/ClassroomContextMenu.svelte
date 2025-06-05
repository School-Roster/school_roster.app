<script lang="ts">
  import {
    classrooms,
    loadClassrooms,
    type ClassroomItem,
  } from "$lib/modules/entities/classroomStore";
  import {
    assignClassroom,
    checkClassroomAvailability,
  } from "$lib/modules/entities/assignments";
  import { addNotification } from "$lib/stores/notificationsStore";
  import { onMount } from "svelte";
  import "$styles/context_menu.scss";

  export let assignment: any;
  export let day: string;
  export let moduleIndex: number;

  let searchTerm = "";
  let isLoading = false;

  $: filteredClassrooms = $classrooms.filter((c) =>
    `${c.building_id}${c.building_number}${c.building_type}`
      .toLowerCase()
      .includes(searchTerm.toLowerCase()),
  );

  async function handleAssign(classroom: ClassroomItem) {
    if (!classroom.id) return;
    isLoading = true;

    try {
      const isAvailable = await checkClassroomAvailability(
        classroom.id,
        day,
        moduleIndex,
      );

      if (!isAvailable) {
        addNotification({
          message: `El aula ${classroom.building_id}-${classroom.building_number} ya está ocupada en este horario`,
          type: "warning",
          timeout: 3000,
        });
        return;
      }

      await assignClassroom(assignment.id, classroom.id);
    } finally {
      isLoading = false;
    }
  }

  onMount(async () => {
    await loadClassrooms();
  });
</script>

<!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
<div class="context-menu">
  <div class="search-box">
    <input
      type="text"
      bind:value={searchTerm}
      placeholder="Buscar aula..."
      disabled={isLoading}
    />
  </div>

  <div class="classroom-list">
    {#if isLoading}
      <div class="loading">Cargando...</div>
    {:else if filteredClassrooms.length === 0}
      <div class="empty">No se encontraron aulas</div>
    {:else}
      {#each filteredClassrooms as classroom (classroom.id)}
        <div
          class="classroom-item"
          on:click|stopPropagation={() => handleAssign(classroom)}
          on:keydown|stopPropagation={(e) =>
            e.key === "Enter" && handleAssign(classroom)}
        >
          <span class="building"
            >{classroom.building_id}-{classroom.building_number}</span
          >
          {#if classroom.building_type}
            <span class="type">({classroom.building_type})</span>
          {/if}
          {#if classroom.capacity}
            <span class="capacity">Capacidad: {classroom.capacity}</span>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</div>
