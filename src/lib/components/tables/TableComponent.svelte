<script lang="ts">
  import "$styles/table.scss";
  import { getContrastColor } from "$lib/utilities/helpers";

  export let data: any[] = [];
  export let columns: any[] = [];
  export let actions: any[] = [];

  let selectedItems: Set<string> = new Set();
  const toggleSelection = (id: string): void => {
    selectedItems.has(id) ? selectedItems.delete(id) : selectedItems.add(id);
    // Force reactivity by reassigning the Set
    selectedItems = new Set(selectedItems);
  };

  function isSelected(id: string): boolean {
    return selectedItems.has(id);
  }

  function deleteSelected(): void {
    const selectedIds = Array.from(selectedItems);
    // Assuming you have an action for deleting multiple items
    const deleteAction = actions.find((action) => action.name === "Eliminar");
    if (deleteAction) {
      deleteAction.action(selectedIds);
    }
    // Clear the selection after deletion
    selectedItems.clear();
    // Force reactivity by reassigning the Set
    selectedItems = new Set(selectedItems);
  }
</script>

<section class="table-container">
  <table>
    <thead>
      <tr>
        <th><!-- Checkbox para seleccionar todos --></th>
        {#each columns as column (column.name)}
          {#if column.name !== "ID"}
            <th>{column.name}</th>
          {/if}
        {/each}
        <th>Acciones</th>
      </tr>
    </thead>
    <tbody>
      {#each data as item (item.id)}
        <tr>
          <td>
            <input
              type="checkbox"
              checked={isSelected(item.id)}
              on:change={() => toggleSelection(item.id)}
            />
          </td>
          {#each columns as column (column.key)}
            {#if column.key !== "id"}
              {#if column.key === "color"}
                <td
                  class="table-color"
                  style="
                    width: 10px;
                    background-color: {item[column.key]};
                    color: {getContrastColor(item[column.key])};
                  "
                >
                  {item[column.key]}
                </td>
              {:else if !item[column.key]}
                <td>N/A</td>
              {:else}
                <td>{item[column.key]}</td>
              {/if}
            {/if}
          {/each}
          <td>
            {#each actions as action (action.name)}
              <button
                class={action.name === "Eliminar" ? "btn btn-danger" : "btn"}
                on:click={() => action.action(item)}
              >
                {#if action.name === "Eliminar"}
                  <img src="/icons/trash.svg" alt="Eliminar" />
                {:else if action.name === "Editar"}
                  <img src="/icons/edit.svg" alt="Editar" />
                {:else}
                  {action.name}
                {/if}
              </button>
            {/each}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</section>
