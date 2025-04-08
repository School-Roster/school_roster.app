<script lang="ts">
  import ZoomBlur from "./ZoomBlur.svelte";
  import "$styles/tutorial.scss";
  import { onMount, createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  
  let showTutorial = false;
  let currentStep = 0;

  export let formName: string = "";

  type Position = "top" | "right" | "bottom" | "left";

  const tutorialSteps: {
    selector: string;
    description: string;
    position: Position;
  }[] = [
    {
      selector: ".new-button",
      description:
        "Para poder comenzar registraremos un nuevo elemento haciendo click en este boton.",
      position: "bottom",
    },
    {
      selector: ".import-button",
      description:
        "Tambien podemos importar todos los datos de " + formName ? formName : "" + " desde un archivo xlsx (Microsoft Excel).",
      position: "top",
    },
    {
      selector: '.search',
      description:
        "Aqui podemos buscar cualquier " + formName ? formName : "" + " en cualquiera de sus secciones registradas.",
      position: "bottom",
    },
    {
      selector: '.table-container',
      description:
        "Aqui se guardaran todos los datos importados o registrados.",
      position: "top",
    },
    {
      selector: '.btn',
      description:
        "Estos son tus botones para editar o eliminar un elemento.",
      position: "left",
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


