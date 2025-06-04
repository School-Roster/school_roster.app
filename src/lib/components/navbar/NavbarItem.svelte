<script lang="ts">
  import "$styles/nav.scss";
  import { itemData } from "./itemData";
  import { generateSchedule } from "$lib/utilities/generateAlgorithm";
  import {
    deleteAll,
    exportFile,
    importFile,
  } from "$lib/utilities/fileHandler";
  import ConfirmModal from "$lib/components/buttons/ConfirmModal.svelte";

  export let isCollapsed: boolean;
  export let createWindow: (windowName: string) => void;

  let showModal = false;

  async function handleConfirm() {
    showModal = false;
    await exportFile();
    await deleteAll();
    window.location.reload();
    window.dispatchEvent(new CustomEvent("showWelcomeScreenAgain"));
  }

  async function handleCancel() {
    showModal = false;
    await deleteAll();
    window.location.reload();
    window.dispatchEvent(new CustomEvent("showWelcomeScreenAgain"));
  }
</script>

{#each itemData as item}
  <!-- representación condicional de los elementos del menú -->
  {#if item.it === 1 || item.it === 3}
    <span class="menu">{item.it === 1 ? "Menu" : "Utilidad"}</span>
  {/if}
  {#if item.it === 2 || item.it === 4 || item.it === 1 || item.it === 3}
    <div class="separator"></div>
  {/if}

  <!-- elementos del menú -->
  <div class="menu-item">
    <button
      class="btn"
      data-menu={item.menu}
      data-icon={item.icon}
      data-name={item.name}
      on:click={() => {
        if (item.menu == "generate") {
          generateSchedule();
        } else if (item.menu != "submenu") {
          createWindow(item.menu);
        }
      }}
    >
      <img src={item.icon} alt={item.name} />
      {#if !isCollapsed}
        <span>{item.name}</span>
      {/if}
      {#if item.submenu.length > 0}
        <span class="arrow-container">
          <img src="/icons/right-arrow.svg" alt="Arrow" />
        </span>
      {/if}
    </button>

    {#if isCollapsed && item.submenu.length == 0}
      <div class="tooltip">{item.name}</div>
    {/if}

    {#if item.submenu.length > 0}
      <ul class="submenu">
        {#each item.submenu as subitem}
          <li>
            <button
              class="submenu-item"
              on:click={async () => {
                if (subitem.menu === "export") {
                  await exportFile();
                } else if (subitem.menu === "import") {
                  await importFile();
                } else if (subitem.menu === "deleteAll") {
                  await deleteAll();
                } else if (subitem.menu === "newSchedule") {
                  showModal = true; //
                } else {
                  createWindow(subitem.menu);
                }
              }}
            >
              <img src={subitem.icon} alt={subitem.name} />
              <span>{subitem.name}</span>
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
{/each}

<ConfirmModal
  isOpen={showModal}
  onConfirm={handleConfirm}
  onCancel={handleCancel}
  title="¿Deseas guardar?"
  message="¿Deseas guardar el horario antes de continuar?"
  confirmText="Guardar"
  cancelText="No guardar"
/>

<style>
  .submenu-item {
    background: none;
    border: none;
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    text-align: left;
    color: white;
  }
</style>
