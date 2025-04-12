<script lang="ts">
  import { onMount, onDestroy, afterUpdate } from "svelte";

  // Props
  export let targetSelector: string; // CSS selector for the element to highlight
  export let description: string; // Description text to show next to the highlighted element
  export let position: "top" | "right" | "bottom" | "left" = "right"; // Position of the description
  export let zoomLevel = 1.01; // How much to zoom the target element (1.05 = 5% zoom)
  export let blurAmount = 5; // Blur amount in pixels for non-target elements
  export let onClose: () => void; // Callback function when tutorial is closed

  let targetElement: HTMLElement | null = null;
  let descriptionElement: HTMLElement | null = null;
  let overlayElement: HTMLElement | null = null;
  let isInitialized = false;

  // Handle ESC key press
  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      closeTutorial();
    }
  }

  // Handle click outside the highlighted area
  function handleOverlayClick(event: MouseEvent) {
    if (overlayElement && event.target === overlayElement) {
      closeTutorial();
    }
  }

  function closeTutorial() {
    if (onClose) {
      onClose();
    }
  }

  function initializeTutorial() {
    // Clean up any existing elements first
    cleanup();

    // Find the target element
    targetElement = document.querySelector(targetSelector);
    if (!targetElement) {
      console.error(`Tutorial target element not found: ${targetSelector}`);
      return;
    }

    // Create overlay for blur effect
    overlayElement = document.createElement("div");
    overlayElement.classList.add("zoon-blur-overlay");
    document.body.appendChild(overlayElement);

    // Create description element
    descriptionElement = document.createElement("div");
    descriptionElement.classList.add("zoon-blur-description");
    descriptionElement.classList.add(`position-${position}`);

    // Create description content
    const descriptionContent = document.createElement("div");
    descriptionContent.classList.add("description-content");
    descriptionContent.textContent = description;
    descriptionElement.appendChild(descriptionContent);

    // Boton para cerrar el highlight (depende de una funcion externa)
    const closeButton = document.createElement("button");
    closeButton.classList.add("close-button");
    closeButton.innerHTML = "&times;";
    closeButton.setAttribute("aria-label", "Close tutorial");
    closeButton.addEventListener("click", closeTutorial);
    descriptionElement.appendChild(closeButton);

    document.body.appendChild(descriptionElement);

    // Aplicamos un pequeno zoom para que destaque mas
    targetElement.style.zIndex = "1000";
    targetElement.style.transform = `scale(${zoomLevel})`;
    targetElement.style.transition = "transform 0.3s ease";

    // Manejamos algunos casos bugeados aqui
    if (
      targetSelector.includes("subjects") ||
      targetSelector.includes("modules-header") ||
      targetSelector.includes("table-container") ||
      targetSelector.includes("btn")
    ) {
      // Ensure the element is visible and above the overlay
      targetElement.style.position = "relative";
      targetElement.style.zIndex = "1001";

      // If the element is inside a submenu, make sure the submenu is visible
      // Si el elemento se encuentra dentro de un submenu, hacemos ese submenu visible
      const submenu = targetElement.closest(".submenu");
      if (submenu) {
        (submenu as HTMLElement).style.display = "block";
        (submenu as HTMLElement).style.zIndex = "1001";
      }
      const header= targetElement.closest(".day-header");
      if (header) {
        (header as HTMLElement).style.display = "block";
        (header as HTMLElement).style.zIndex = "1001";
      }
    }

    // Use setTimeout to ensure the DOM is fully rendered before positioning
    setTimeout(() => {
      // Descripcion (posicion)
      updateDescriptionPosition();

      // Add event listeners
      window.addEventListener("resize", updateDescriptionPosition);
      window.addEventListener("keydown", handleKeyDown);
      if (overlayElement) {
        overlayElement.addEventListener("click", handleOverlayClick);
      }

      isInitialized = true;
    }, 100); // Small delay to ensure DOM is ready
  }

  function updateDescriptionPosition() {
    if (!targetElement || !descriptionElement) return;

    const targetRect = targetElement.getBoundingClientRect();
    const scrollX = window.scrollX;
    const scrollY = window.scrollY;

    // Basamos la posicion de la descripcion en la variable
    switch (position) {
      case "right":
        descriptionElement.style.left = `${targetRect.right + scrollX + 10}px`;
        descriptionElement.style.top = `${targetRect.top + scrollY + targetRect.height / 2 - descriptionElement.offsetHeight / 2}px`;
        break;
      case "left":
        descriptionElement.style.left = `${targetRect.left + scrollX - descriptionElement.offsetWidth - 10}px`;
        descriptionElement.style.top = `${targetRect.top + scrollY + targetRect.height / 2 - descriptionElement.offsetHeight / 2}px`;
        break;
      case "top":
        descriptionElement.style.left = `${targetRect.left + scrollX + targetRect.width / 2 - descriptionElement.offsetWidth / 2}px`;
        descriptionElement.style.top = `${targetRect.top + scrollY - descriptionElement.offsetHeight - 10}px`;
        break;
      case "bottom":
        descriptionElement.style.left = `${targetRect.left + scrollX + targetRect.width / 2 - descriptionElement.offsetWidth / 2}px`;
        descriptionElement.style.top = `${targetRect.bottom + scrollY + 10}px`;
        break;
    }
  }

  function cleanup() {
    // Clean up
    if (targetElement) {
      targetElement.style.zIndex = "";
      targetElement.style.transform = "";
      targetElement.style.transition = "";
      targetElement.style.position = "";
      targetElement = null;
    }

    if (overlayElement && overlayElement.parentNode) {
      overlayElement.parentNode.removeChild(overlayElement);
      overlayElement = null;
    }

    if (descriptionElement && descriptionElement.parentNode) {
      descriptionElement.parentNode.removeChild(descriptionElement);
      descriptionElement = null;
    }

    window.removeEventListener("resize", updateDescriptionPosition);
    window.removeEventListener("keydown", handleKeyDown);

    isInitialized = false;
  }

  onMount(() => {
    initializeTutorial();
  });

  // Reinitialize when props change
  afterUpdate(() => {
    if (isInitialized) {
      initializeTutorial();
    }
  });

  onDestroy(() => {
    cleanup();
  });
</script>

<style>
  :global(.zoon-blur-overlay) {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(5px);
    z-index: 999;
    pointer-events: auto;
  }

  :global(.zoon-blur-description) {
    position: absolute;
    background-color: white;
    color: #333;
    padding: 12px 16px;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    max-width: 300px;
    z-index: 1001;
    font-size: 14px;
    line-height: 1.5;
  }

  :global(.zoon-blur-description .description-content) {
    margin-right: 20px;
  }

  :global(.zoon-blur-description .close-button) {
    position: absolute;
    top: 8px;
    right: 8px;
    background: none;
    border: none;
    font-size: 18px;
    cursor: pointer;
    color: #666;
    padding: 0;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
  }

  :global(.zoon-blur-description .close-button:hover) {
    background-color: #f0f0f0;
    color: #333;
  }

  :global(.zoon-blur-description::before) {
    content: "";
    position: absolute;
    width: 0;
    height: 0;
    border-style: solid;
  }

  :global(.zoon-blur-description.position-right::before) {
    left: -10px;
    top: 50%;
    transform: translateY(-50%);
    border-width: 10px 10px 10px 0;
    border-color: transparent white transparent transparent;
  }

  :global(.zoon-blur-description.position-left::before) {
    right: -10px;
    top: 50%;
    transform: translateY(-50%);
    border-width: 10px 0 10px 10px;
    border-color: transparent transparent transparent white;
  }

  :global(.zoon-blur-description.position-top::before) {
    bottom: -10px;
    left: 50%;
    transform: translateX(-50%);
    border-width: 10px 10px 0 10px;
    border-color: white transparent transparent transparent;
  }

  :global(.zoon-blur-description.position-bottom::before) {
    top: -10px;
    left: 50%;
    transform: translateX(-50%);
    border-width: 0 10px 10px 10px;
    border-color: transparent transparent white transparent;
  }
</style>
