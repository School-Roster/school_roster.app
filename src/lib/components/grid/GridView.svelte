<script lang="ts">
  import "$styles/grid.scss";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
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
  export let modulesPerDay: number = 9;

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
              <div
                class="module-cell"
                on:click={() =>
                  handleModuleAssignment(group.id, day, moduleIndex)}
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
  @use "../../../styles/variables";
  .schedule-grid {
    border: 1px solid variables.$white-overlay;
    border-radius: 8px;
    overflow: hidden;
    background: variables.$white-overlay;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

    .header-row {
      display: grid;
      grid-template-columns: auto repeat(5, 1fr);
      background-color: variables.$white-background;
      border-bottom: 1px solid variables.$black;

      .corner-cell {
        padding: 0.5rem;
        font-weight: 600;
        text-align: center;
        border-right: 1px solid variables.$black;
        background-color: variables.$white-overlay;
      }

      .day-column {
        border-right: 1px solid variables.$black;

        &:last-child {
          border-right: none;
        }

        .day-header {
          padding: 0.75rem;
          font-weight: 600;
          text-align: center;
          border-bottom: 1px solid variables.$black;
          background-color: variables.$white-overlay;
        }

        .modules-header {
          display: flex;

          .module-label {
            flex: 1;
            padding: 0.5rem;
            text-align: center;
            font-size: 0.875rem;
            color: variables.$black;
            border-right: 1px solid variables.$black;

            &:last-child {
              border-right: none;
            }
          }
        }
      }
    }

    .grid-content {
      .group-row {
        display: grid;
        height: 40px;
        grid-template-columns: auto repeat(5, 1fr);
        border-bottom: 1px solid variables.$black;

        &:last-child {
          border-bottom: none;
        }

        .group-cell {
          min-width: 42px;
          padding: 0.75rem;
          font-weight: 500;
          text-align: center;
          border-right: 1px solid variables.$black;
          background-color: variables.$white-background;
        }

        .day-modules {
          display: flex;
          // border-right: 1px solid #ddd;
          border-right: 1px solid variables.$black;

          &:last-child {
            border-right: none;
          }

          .module-cell {
            flex: 1;
            max-height: 92%;
            border-right: 1px solid #eee;
            background-color: variables.$white-hard;
            cursor: pointer;
            transition: background-color 0.2s;

            &:last-child {
              border-right: none;
            }

            &:hover {
              background-color: variables.$white-overlay;
            }
          }
        }
      }
    }
  }
</style>
