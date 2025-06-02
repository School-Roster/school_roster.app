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
      selector: ".sidebar",
      description:
        "Esta es tu barra de navegación principal. Puedes colapsarla para aprovechar mejor el espacio de trabajo.",
      position: "right",
    },
    {
      selector: ".toggle-btn",
      description:
        "Haz clic en este botón para expandir o contraer la barra de navegación según lo necesites.",
      position: "right",
    },
    {
      selector: '.menu-item button[data-menu="subjects"]',
      description:
        "Haz clic aquí para acceder a la gestión y registro de materias.",
      position: "right",
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
      zoomLevel={currentStep === 2 ? 1.05 : 1.01}
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
