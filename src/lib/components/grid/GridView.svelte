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
  import NavbarTutorial from "../utils/tutorials/NavbarTutorial.svelte";

  // Tutorial menu state
  let showTutorialMenu = false;
  let showNavbarTutorial = false;

  // Close tutorial menu when clicking outside
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.tutorial-menu-container')) {
      showTutorialMenu = false;
    }
  }

  // Handle tutorial completion
  function handleTutorialComplete() {
    showNavbarTutorial = false;
  }

  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });

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
</script>

<div class="tutorial-menu-container">
  <button 
    class="tutorial-menu-button" 
    on:click={() => showTutorialMenu = !showTutorialMenu}
    aria-label="Tutoriales disponibles"
  >
    <img src="/icons/circle-question-solid.svg" alt="Tutoriales">
  </button>
  
  {#if showTutorialMenu}
    <div class="tutorial-menu-dropdown">
      <div class="tutorial-menu-header">
        <h3>Tutoriales disponibles</h3>
      </div>
      <div class="tutorial-menu-items">
        <button 
          class="tutorial-menu-item" 
          on:click={() => {
            showNavbarTutorial = true;
            showTutorialMenu = false;
          }}
        >
          <img src="/icons/navbar.svg" alt="Navbar">
          <span>Tutorial de navegación</span>
        </button>
        <!-- Add more tutorial options here as they become available -->
      </div>
    </div>
  {/if}
</div>

{#if showNavbarTutorial}
  <NavbarTutorial on:complete={handleTutorialComplete} />
{/if}

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

<style>
  .tutorial-menu-container {
    position: relative;
    display: flex;
    flex-direction: row-reverse;
    margin-bottom: 1rem;
    margin-right: 1rem;
  }
  
  .tutorial-menu-button {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.65rem;
    height: 1.65rem;
    transition: transform 0.2s ease;
  }
  
  .tutorial-menu-button:hover {
    transform: scale(1.1);
  }
  
  .tutorial-menu-button img {
    width: 100%;
    height: 100%;
  }
  
  .tutorial-menu-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    width: 250px;
    z-index: 1000;
    overflow: hidden;
  }
  
  .tutorial-menu-header {
    padding: 12px 16px;
    border-bottom: 1px solid #eee;
  }
  
  .tutorial-menu-header h3 {
    margin: 0;
    font-size: 16px;
    color: #333;
  }
  
  .tutorial-menu-items {
    padding: 8px 0;
  }
  
  .tutorial-menu-item {
    display: flex;
    align-items: center;
    padding: 10px 16px;
    width: 100%;
    border: none;
    background: none;
    cursor: pointer;
    text-align: left;
    transition: background-color 0.2s ease;
  }
  
  .tutorial-menu-item:hover {
    background-color: #f5f5f5;
  }
  
  .tutorial-menu-item img {
    width: 20px;
    height: 20px;
    margin-right: 12px;
  }
  
  .tutorial-menu-item span {
    font-size: 14px;
    color: #333;
  }
</style>
