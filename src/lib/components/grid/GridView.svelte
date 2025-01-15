<script lang="ts">
  import "$styles/grid.scss";
  import {
    groups,
    loadGroups,
    type GroupItem,
  } from "$lib/modules/entities/groupsStore";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  export let days: string[] = [
    "Lunes",
    "Martes",
    "Miercoles",
    "Jueves",
    "Viernes",
  ];

  // TODO: Por ahora los modulos viven aqui, despues los sacamos de la informacion
  //       registrada en ~School
  export let modulesPerDay: number = 6;

  function handleModuleAssignment(
    groupId: string,
    day: string,
    moduleIndex: number,
  ) {
    console.log(
      `Assigning module for group ${groupId} on ${day} at index ${moduleIndex}`,
    );
  }

  onMount(() => {
    loadGroups();
    // Carga los grupos de nuevo en caso de actualizados
    listen("groups_updated", async () => {
      await loadGroups();
    });
  });
</script>

<section class="grid">
  <!-- Header row with days and their modules -->
  <div class="grid__header">
    <div class="header-group">Grupos</div>
    {#each days as day}
      <div class="header-day">
        <div class="day-name">{day}</div>
        <div class="modules-row">
          {#each Array(modulesPerDay) as _, index}
            <div class="module-header">{index + 1}</div>
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <!-- Groups and their modules -->
  <div class="grid__content">
    {#each $groups as group}
      <div class="grid__row">
        <div class="grid__row__group">{group.grade}{group.group}</div>
        {#each days as day}
          <div class="day-slot">
            {#each Array(modulesPerDay) as _, moduleIndex}
              <div 
                class="module-slot"
                on:click={() => handleModuleAssignment(group.id, day, moduleIndex)}
              >
                <!-- Module content will go here -->
              </div>
            {/each}
          </div>
        {/each}
      </div>
    {/each}
  </div>
</section>

<style lang="scss">
  .grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    
    &__header {
      display: grid;
      grid-template-columns: auto repeat(5, 1fr);
      gap: 1rem;
      align-items: start;

      .header-group {
        min-width: 100px;
        padding: 0.5rem;
        background-color: #f0f0f0;
        border-radius: 4px;
        text-align: center;
      }

      .header-day {
        .day-name {
          font-weight: bold;
          margin-bottom: 0.5rem;
          text-align: center;
          padding: 0.5rem;
          background-color: #f0f0f0;
          border-radius: 4px;
        }

        .modules-row {
          display: flex;
          gap: 0.5rem;

          .module-header {
            flex: 1;
            text-align: center;
            padding: 0.25rem;
            background-color: #e0e0e0;
            border-radius: 2px;
            font-size: 0.8rem;
          }
        }
      }
    }
    
    &__content {
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }
    
    &__row {
      display: grid;
      grid-template-columns: auto repeat(5, 1fr);
      gap: 1rem;
      align-items: center;
      
      &__group {
        padding: 0.5rem;
        background-color: #f0f0f0;
        border-radius: 4px;
        min-width: 100px;
        text-align: center;
      }

      .day-slot {
        display: flex;
        gap: 0.5rem;
        
        .module-slot {
          flex: 1;
          background-color: #ffffff;
          border: 1px solid #ddd;
          border-radius: 4px;
          padding: 0.5rem;
          min-height: 40px;
          cursor: pointer;
          transition: background-color 0.2s;
          
          &:hover {
            background-color: #f5f5f5;
          }
        }
      }
    }
  }
</style>
