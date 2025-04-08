<script lang="ts">
  import ZoomBlur from "./ZoomBlur.svelte";
  import "$styles/tutorial.scss";
  import { onMount, createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  
  let showTutorial = false;
  let currentStep = 0;

  type Position = "top" | "right" | "bottom" | "left";

  const tutorialSteps: {
    selector: string;
    description: string;
    position: Position;
  }[] = [
    {
      selector: ".schedule-grid",
      description:
        "Esta es tu maqueta principal, en este podras asignar materias a un dia y modulo",
      position: "bottom",
    },
    {
      selector: ".corner-cell",
      description:
        "Aqui abajo saldran todos los grupos conforme los vayas registrando.",
      position: "right",
    },
    {
      selector: '.day-column .modules-header',
      description:
        "Cada seccion representa un modulo, puedes cambiar la cantidad de modulos en configuracion",
      position: "bottom",
    },
    {
      selector: '.subjects-items',
      description:
        "En esta seccion apareceran tus materias cuando las asignes a un profesor.",
      position: "top",
    },
    {
      selector: '.day-modules',
      description:
        "Este esta es tu maqueta principal, aqui arrastraras las materias para asignarlas a un grupo, modulo y dia",
      position: "top",
    },
  ];

  function startTutorial() {
    showTutorial = false; // First hide it
    setTimeout(() => {
      currentStep = 0;
      showTutorial = true; // Then show it again
    }, 50);
  }

  function nextStep() {
    if (currentStep < tutorialSteps.length - 1) {
      currentStep++;
    } else {
      endTutorial();
    }
  }

  function prevStep() {
    if (currentStep > 0) {
      currentStep--;
    }
  }


  function endTutorial() {
    showTutorial = false;
    // Dispatch an event to notify the parent component
    dispatch('complete');
  }

  onMount(() => { 
    startTutorial();
  });
</script>

<div class="tutorial-container">
  {#if showTutorial}
    <ZoomBlur
      targetSelector={tutorialSteps[currentStep].selector}
      description={tutorialSteps[currentStep].description}
      position={tutorialSteps[currentStep].position}
      onClose={endTutorial}
    />

    <div class="tutorial-controls">
      <button on:click={prevStep} disabled={currentStep === 0}>
        Anterior
      </button>
      <span>{currentStep + 1} / {tutorialSteps.length}</span>
      <button on:click={nextStep}>
        {currentStep === tutorialSteps.length - 1 ? "Finalizar" : "Siguiente"}
      </button>
      <button class="skip-btn" on:click={endTutorial}>Omitir tutorial</button>
    </div>
  {/if}
</div>

