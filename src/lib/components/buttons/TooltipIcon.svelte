<script>
  
    let showTooltip = false;
  
    function handleMouseEnter() {
      showTooltip = true;
    }
  
    function handleMouseLeave() {
      showTooltip = false;
    }
  </script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="tooltip-container" on:mouseenter={handleMouseEnter} on:mouseleave={handleMouseLeave}>
    <img class="information-icon" src="/icons/informacion.png" alt="informacion-icon" />
    
    {#if showTooltip}
      <div class="tooltip">
        15 champions
      </div>
    {/if}
  </div>
  
  <style lang="scss">
  .tooltip-container {
    position: relative;
    display: inline-block;
    cursor: pointer;
  
    .tooltip {
      position: absolute;
      bottom: 130%;
      left: 50%;
      transform: translateX(-50%);
      background-color: #333;
      color: #fff;
      padding: 8px;
      border-radius: 5px;
      font-size: 14px;
      white-space: nowrap;
      opacity: 0;
      visibility: hidden;
      transition: opacity 0.2s ease-in-out;
      box-shadow: 0px 4px 6px rgba(0, 0, 0, 0.1);
  
      &::after {
        content: "";
        position: absolute;
        top: 100%;
        left: 50%;
        transform: translateX(-50%);
        border-width: 5px;
        border-style: solid;
        border-color: #333 transparent transparent transparent;
      }
    }
  
    &:hover .tooltip {
      opacity: 1;
      visibility: visible;
    }
  }
  </style>
  
