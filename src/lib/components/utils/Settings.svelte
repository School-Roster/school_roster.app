<script lang="ts">
  import "$styles/config.scss";

  import ToggleDarkTheme from "../buttons/ToggleDarkTheme.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { getCurrent } from "@tauri-apps/api/window";
  import {
    configStore,
    loadConfig,
    saveConfig,
  } from "$lib/modules/config/configStore";

  // Configuration variables (loaded from store)
  let days = ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sabado"];
  let selectedDays = [];
  let moduleCount = 6;
  let moduleDuration = 60;
  let durationUnit = "minutes";
  let hasBreaks = false;
  let breakCount = 1;
  let breakDuration = 30;
  let breakPositions = [2];

  // Status variables
  let isLoading = true;
  let isSaving = false;
  let showSuccessMessage = false;

  // Load configuration on component mount
  onMount(async () => {
    await loadConfig();
    configStore.subscribe((config) => {
      selectedDays = [...config.days];
      moduleCount = config.modulesPerDay;
      moduleDuration = config.moduleDuration;
      durationUnit = config.durationUnit;
      hasBreaks = config.hasBreaks;
      breakCount = config.breakCount;
      breakDuration = config.breakDuration;
      breakPositions = [...config.breakPositions];
      isLoading = false;
    });
  });

  // Watch for changes in break count to update positions
  $: if (breakCount !== breakPositions.length) {
    if (breakCount > breakPositions.length) {
      while (breakPositions.length < breakCount) {
        for (let i = 0; i < moduleCount - 1; i++) {
          if (!breakPositions.includes(i)) {
            breakPositions.push(i);
            break;
          }
        }
      }
    } else {
      breakPositions = breakPositions.slice(0, breakCount);
    }
  }

  // Watch for changes in module count to update max breaks
  $: if (moduleCount < breakCount + 1) {
    breakCount = Math.max(0, moduleCount - 1);
  }

  function handleDayToggle(day: string): void {
    if (selectedDays.includes(day)) {
      selectedDays = selectedDays.filter((d) => d !== day);
    } else {
      selectedDays = [...selectedDays, day].sort(
        (a, b) => days.indexOf(a) - days.indexOf(b),
      );
    }
  }

  async function saveConfiguration() {
    isSaving = true;
    try {
      await saveConfig({
        days: selectedDays,
        modulesPerDay: moduleCount,
        moduleDuration,
        durationUnit,
        hasBreaks,
        breakCount,
        breakDuration,
        breakPositions,
      });

      showSuccessMessage = true;
      setTimeout(() => {
        showSuccessMessage = false;
      }, 3000);
    } catch (error) {
      console.error("Error saving configuration:", error);
    } finally {
      isSaving = false;
    }
  }

  async function createWindow(windowName: string) {
    const win = new WebviewWindow(`${windowName}`, {
      url: `/window/${windowName}`,
      title: "School Roster",
      width: 1000,
      height: 800,
      resizable: true,
      focus: true,
      visible: true,
    });
    await win.show();
  }
  $: if (hasBreaks && breakCount > breakPositions.length) {
    for (let i = breakPositions.length; i < breakCount; i++) {
      breakPositions.push(i); // Default: break after module i
    }
  }
</script>

<section class="config-card">
  <h2>Configuración</h2>

  {#if isLoading}
    <div class="loading">Cargando configuración...</div>
  {:else}
    <!-- Days Configuration -->
    <div class="config-section">
      <h3>Días de clase</h3>
      <div class="days-grid">
        {#each days as day}
          <div
            class="day-option {selectedDays.includes(day) ? 'selected' : ''}"
            on:click={() => handleDayToggle(day)}
          >
            <input
              type="checkbox"
              checked={selectedDays.includes(day)}
              on:change={() => {}}
            />
            {day}
          </div>
        {/each}
      </div>
    </div>

    <!-- Modules Configuration -->
    <div class="config-section">
      <h3>Configuración de módulos</h3>

      <div class="module-config">
        <div class="config-row">
          <div class="form-group">
            <label for="module-count">Número de módulos</label>
            <select id="module-count" bind:value={moduleCount}>
              {#each Array(12) as _, i}
                <option value={i + 1}>{i + 1}</option>
              {/each}
            </select>
          </div>

          <div class="form-group">
            <label for="module-duration">Duración de cada módulo</label>
            <div class="duration-input">
              <input
                type="number"
                id="module-duration"
                bind:value={moduleDuration}
                min="30"
                max="180"
                step="5"
              />
              <select bind:value={durationUnit}>
                <option value="minutes">min</option>
                <option value="hours">hr</option>
              </select>
            </div>
          </div>
        </div>

        <div class="config-row">
          <div class="form-group checkbox-group">
            <label class="checkbox-label">
              <input type="checkbox" bind:checked={hasBreaks} />
              <span>Incluir descansos entre módulos</span>
            </label>
          </div>
        </div>

        {#if hasBreaks}
          <div class="breaks-config">
            <div class="form-group">
              <label for="break-count">Número de descansos</label>
              <select id="break-count" bind:value={breakCount}>
                {#each Array(Math.max(1, moduleCount - 1)) as _, i}
                  <option value={i + 1}>{i + 1}</option>
                {/each}
              </select>
            </div>

            <div class="form-group">
              <label for="break-duration"
                >Duración de cada descanso (minutos)</label
              >
              <input
                type="number"
                id="break-duration"
                bind:value={breakDuration}
                min="5"
                max="60"
                step="5"
              />
            </div>

            {#if breakCount > 0 && moduleCount > 0}
              <div class="break-positions">
                <label>Posición de los descansos</label>
                <div class="break-slots">
                  {#each Array(Math.min(breakCount, moduleCount - 1)) as _, i}
                    <div class="break-slot-row">
                      <span>Descanso {i + 1}:</span>
                      <select bind:value={breakPositions[i]}>
                        {#each Array(moduleCount - 1) as _, j}
                          <option value={j}>Después del módulo {j + 1}</option>
                        {/each}
                      </select>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>

    <!-- Save Configuration Button -->
    <div class="config-section">
      <button
        class="btn-save {isSaving ? 'saving' : ''}"
        on:click={saveConfiguration}
        disabled={isSaving}
      >
        {isSaving ? "Guardando..." : "Guardar configuración"}
      </button>

      {#if showSuccessMessage}
        <div class="success-message">Configuración guardada correctamente</div>
      {/if}
    </div>

    <!-- Theme Configuration -->
    <div class="config-section theme-toggle">
      <h3>Tema</h3>
      <p>Cambia el tema entre claro y oscuro.</p>
      <div class="toggle-container">
        <ToggleDarkTheme />
      </div>
    </div>

    <!-- School Mapping -->
    <div class="config-section">
      <h3>Mapear Escuela</h3>
      <button on:click={() => createWindow("mapping")} class="btn-mapping">
        Mapear escuela
      </button>
    </div>
  {/if}
</section>
