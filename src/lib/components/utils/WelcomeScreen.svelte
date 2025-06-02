<script lang="ts">
  import { onMount } from "svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";
  import "$styles/welcome.scss";
  import { saveConfig } from '$lib/modules/config/configStore';

  // State variables
  let currentStep = 1;
  let selectedOption = "";

  // Configuration options
  let days = ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sabado"];
  let selectedDays = [...days];

  // Module configuration variables
  let moduleCount = 6; // Default number of modules
  let moduleDuration = 60; // Default duration in minutes
  let durationUnit = "minutes"; // "minutes" or "hours"
  let hasBreaks = false; // Whether to include breaks
  let breakCount = 1; // Number of breaks
  let breakDuration = 30; // Break duration in minutes
  let breakPositions = [2]; // After which modules to place breaks (0-indexed)
  let generatedSchedule = []; // Holds the generated schedule preview

  // Registration data progress tracking
  let dataCompletionStatus = {
    subjects: false,
    teachers: false,
    groups: false,
    classrooms: false,
  };

  // TODO: Abrir archivo
  let fileInput: HTMLInputElement;


  // Watch for changes in break count to update positions
  $: if (breakCount !== breakPositions.length) {
    // Adjust break positions array
    if (breakCount > breakPositions.length) {
      // Add new positions
      while (breakPositions.length < breakCount) {
        // Find first unused position
        for (let i = 0; i < moduleCount - 1; i++) {
          if (!breakPositions.includes(i)) {
            breakPositions.push(i);
            break;
          }
        }
      }
    } else {
      // Remove excess positions
      breakPositions = breakPositions.slice(0, breakCount);
    }
  }

  // Watch for changes in module count to update max breaks
  $: if (moduleCount < breakCount + 1) {
    breakCount = Math.max(0, moduleCount - 1);
  }

  function handleOptionSelect(option: string): void {
    selectedOption = option;
  }

  function handleFileSelect(event: Event): void {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file) {
      // TODO: Process the file and load the schedule
      console.log("File selected:", file.name);
    }
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

  function nextStep(): void {
    if (currentStep === 1 && !selectedOption) {
      return; // Don't proceed if no option is selected
    }

    if (currentStep === 1) {
      currentStep = 2;
    } else if (currentStep === 2) {
      currentStep = 3;
    } else if (currentStep === 3) {
      // Final step, proceed to the main application
      continueToSchedule();
    }
  }

  function previousStep(): void {
    if (currentStep > 1) {
      currentStep--;
    }
  }

  async function startNewSchedule(): void {
    // Save configuration before proceeding
    await saveConfig({
      days: selectedDays,
      modulesPerDay: moduleCount,
      moduleDuration,
      durationUnit,
      hasBreaks,
      breakCount,
      breakDuration,
      breakPositions
    });
    
    continueToSchedule();
  }

  async function generateSchedule(): void {
    // Save configuration before proceeding
    await saveConfig({
      days: selectedDays,
      modulesPerDay: moduleCount,
      moduleDuration,
      durationUnit,
      hasBreaks,
      breakCount,
      breakDuration,
      breakPositions
    });
    
    continueToSchedule();
  }

  function continueToSchedule(): void {
    // TODO: Navigate to the main schedule view
    window.dispatchEvent(new CustomEvent("closeWelcomeScreen"));
  }

  // Abre una nueva ventana con el nombre especificado (groups, subjects, teachers, classrooms)
  async function createWindow(windowName: string) {
    // Store current progress in localStorage or a store
    localStorage.setItem("currentSetupStep", "2");
    localStorage.setItem("selectedOption", selectedOption);
    localStorage.setItem(
      "dataCompletionStatus",
      JSON.stringify(dataCompletionStatus),
    );
    dataCompletionStatus[windowName] = true;
    if (windowName === "classrooms") windowName = "classroom";
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
</script>

<section class="welcome-container">
  <div class="step-indicator">
    <div
      class="step {currentStep === 1
        ? 'active'
        : currentStep > 1
          ? 'completed'
          : ''}"
    >
      1
    </div>
    <div
      class="step {currentStep === 2
        ? 'active'
        : currentStep > 2
          ? 'completed'
          : ''}"
    >
      2
    </div>
    <div class="step {currentStep === 3 ? 'active' : ''}">3</div>
  </div>

  {#if currentStep === 1}
    <div class="welcome-header">
      <img src="/icons/logo_transparent.png" alt="School Roster Logo" />
      <h1>Bienvenido a School Roster</h1>
      <p>
        Con este programa podrás realizar tu horario escolar de manera fácil y
        rápida. Queremos ayudarte para que no tengas que complicarte tu trabajo.
        Organiza tus clases, profesores y grupos de manera eficiente.
      </p>
    </div>

    <div class="step-container">
      <h2 class="step-title">¿Qué deseas hacer?</h2>
      <div class="options-grid">
        <div
          class="option-card {selectedOption === 'new' ? 'selected' : ''}"
          on:click={() => handleOptionSelect("new")}
        >
          <img src="/icons/new.svg" alt="Crear nuevo horario" />
          <h3>Crear nuevo horario</h3>
          <p>
            Crea un horario desde cero, configurando todos los elementos
            manualmente.
          </p>
        </div>

        <div
          class="option-card {selectedOption === 'existing' ? 'selected' : ''}"
          on:click={() => handleOptionSelect("existing")}
        >
          <img src="/icons/open.svg" alt="Abrir horario existente" />
          <h3>Abrir horario existente</h3>
          <p>
            Carga un horario previamente guardado para continuar trabajando en
            él.
          </p>
        </div>

        <div
          class="option-card {selectedOption === 'generate' ? 'selected' : ''}"
          on:click={() => handleOptionSelect("generate")}
        >
          <img src="/icons/robot.svg" alt="Generar horario automáticamente" />
          <h3>Generar horario</h3>
          <p>
            Utiliza algoritmos para generar automáticamente un horario
            optimizado.
          </p>
        </div>
      </div>
    </div>
  {:else if currentStep === 2}
    <div class="step-container">
      <h2 class="step-title">Registro de datos</h2>
      <p style="margin-top: 0;">
        Haz clic en cada categoría para registrar la información
        correspondiente.
      </p>

      {#if selectedOption === "existing"}
        <div class="registration-form">
          <h2>Selecciona un archivo de horario</h2>
          <div class="form-grid">
            <div class="form-group">
              <label for="file-input">Archivo de horario (.json)</label>
              <input
                type="file"
                id="file-input"
                accept=".json"
                on:change={handleFileSelect}
                bind:this={fileInput}
              />
            </div>
          </div>
        </div>
      {/if}

      <div class="registration-form">
        <div class="data-registration-container">
          <div class="data-cards-grid">
            <div
              class="data-card {dataCompletionStatus.subjects
                ? 'completed'
                : ''}"
              on:click={() => createWindow("subjects")}
            >
              <div class="card-content">
                <img src="/icons/books.svg" alt="Materias" />
                <h3>Materias</h3>
                <p>Registra las materias que se impartirán</p>
              </div>
              {#if dataCompletionStatus.subjects}
                <div class="completion-badge">
                  <span class="checkmark">✓</span>
                </div>
              {/if}
            </div>

            <div
              class="data-card {dataCompletionStatus.teachers
                ? 'completed'
                : ''}"
              on:click={() => createWindow("teachers")}
            >
              <div class="card-content">
                <img src="/icons/teacher.svg" alt="Profesores" />
                <h3>Profesores</h3>
                <p>Agrega a los profesores y sus datos</p>
              </div>
              {#if dataCompletionStatus.teachers}
                <div class="completion-badge">
                  <span class="checkmark">✓</span>
                </div>
              {/if}
            </div>

            <div
              class="data-card {dataCompletionStatus.groups ? 'completed' : ''}"
              on:click={() => createWindow("groups")}
            >
              <div class="card-content">
                <img src="/icons/group.svg" alt="Grupos" />
                <h3>Grupos</h3>
                <p>Configura los grupos escolares</p>
              </div>
              {#if dataCompletionStatus.groups}
                <div class="completion-badge">
                  <span class="checkmark">✓</span>
                </div>
              {/if}
            </div>

            <div
              class="data-card {dataCompletionStatus.classrooms
                ? 'completed'
                : ''}"
              on:click={() => createWindow("classrooms")}
            >
              <div class="card-content">
                <img src="/icons/door.svg" alt="Aulas" />
                <h3>Aulas</h3>
                <p>Registra las aulas disponibles</p>
              </div>
              {#if dataCompletionStatus.classrooms}
                <div class="completion-badge">
                  <span class="checkmark">✓</span>
                </div>
              {/if}
            </div>
          </div>

          <div class="registration-progress">
            <p>
              Progreso: {Object.values(dataCompletionStatus).filter(Boolean)
                .length}/4 categorías completadas
            </p>
            <div class="progress-bar">
              <div
                class="progress-fill"
                style="width: {(Object.values(dataCompletionStatus).filter(
                  Boolean,
                ).length /
                  4) *
                  100}%"
              ></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  {:else if currentStep === 3}
    <div class="step-container">
      <h2 class="step-title">Configuración del horario</h2>

      <div class="configuration-form">
        <div class="config-section">
          <h3>Días de clase</h3>
          <div class="options-grid">
            {#each days as day}
              <div
                class="option-item {selectedDays.includes(day)
                  ? 'selected'
                  : ''}"
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

        <!-- Configuration section for modules -->
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
                              <option value={j}
                                >Después del módulo {j + 1}</option
                              >
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
      </div>
    </div>
  {/if}

  <div class="navigation-buttons">
    {#if currentStep > 1}
      <button class="secondary" on:click={previousStep}>Anterior</button>
    {:else}
      <div></div>
      <!-- Empty div to maintain layout -->
    {/if}

    {#if currentStep === 1}
      <button class="primary" on:click={nextStep} disabled={!selectedOption}>
        Siguiente
      </button>
    {:else if currentStep === 2}
      <button class="primary" on:click={nextStep}>Siguiente</button>
    {:else if currentStep === 3}
      {#if selectedOption === "new"}
        <button class="primary" on:click={startNewSchedule}
          >Crear horario</button
        >
      {:else if selectedOption === "existing"}
        <button class="primary" on:click={continueToSchedule}>Continuar</button>
      {:else if selectedOption === "generate"}
        <button class="primary" on:click={generateSchedule}
          >Generar horario</button
        >
      {/if}
    {/if}
  </div>
</section>
