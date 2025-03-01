<script lang="ts">
  let showTooltip: boolean = false;

  function handleMouseEnter(): void {
    showTooltip = true;
  }

  function handleMouseLeave(): void {
    showTooltip = false;
  }

  export let description: string = "";
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="tooltip-container"
  on:mouseenter={handleMouseEnter}
  on:mouseleave={handleMouseLeave}
>
  <img
    class="information-icon"
    src="/icons/information.svg"
    alt="informacion-icon"
  />

  {#if showTooltip}
    <div class="tooltip">
      {description}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "../../../styles/variables";

  .information-icon {
    width: 1.5rem;
    filter: variables.$filter-black;

    body.dark & {
      filter: variables.$filter-other;
    }
  }
  .tooltip-container {
    position: relative;
    display: inline-block;
    cursor: pointer;

    .tooltip {
      position: absolute;
      z-index: 1;
      top: -5px;
      right: 125%;
      transform: none;
      background-color: variables.$black-overlay;
      color: variables.$white-hard;
      padding: 8px;
      border-radius: 5px;
      font-size: 14px;
      white-space: nowrap;
      opacity: 0;
      transition: opacity 0.2s ease-in-out;
      box-shadow: 0px 4px 6px rgba(0, 0, 0, 0.1);

      body.dark & {
        background-color: variables.$white-overlay;
        color: variables.$black-hard;
      }
    }

    &:hover .tooltip {
      opacity: 1;
    }
  }
</style>
