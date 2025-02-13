<script lang="ts">
  import "$styles/table.scss";
  import { getContrastColor } from "$lib/utilities/helpers";

  export let data: any[] = [];
  export let columns: any[] = [];
  export let actions: any[] = [];

  let selectedItems: Set<number> = new Set();

  const toggleSelection = (id: number): void => {
    selectedItems.has(id) ? selectedItems.delete(id) : selectedItems.add(id);
    // Fuerza reactividad
    selectedItems = new Set(selectedItems);
  };

  function toggleAll(event: Event): void {
    const target = event.target as HTMLInputElement;
    if (!target.checked) {
      selectedItems.clear();
    } else {
      selectedItems = new Set(data.map((item) => Number(item.id)));
    }
    selectedItems = new Set(selectedItems);
  }
  $: allSelected = data.length > 0 && selectedItems.size === data.length;

  // TODO
  function handleAction(action: any, item: any): void {
    if (action.name === "Eliminar" && selectedItems.size > 0) {
      // Si hay item y hay seleccion borra los items seleccionados
      action.action(Array.from(selectedItems));
      selectedItems.clear();
      selectedItems = new Set();
    } else {
      // Acciones para items sencillos (editar/eliminar)
      action.action(item);
    }
  }
</script>

<section class="table-container">
  <table>
    <thead>
      <tr>
        <th>
          <input type="checkbox" checked={allSelected} on:change={toggleAll} />
        </th>
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
              checked={selectedItems.has(Number(item.id))}
              on:change={() => toggleSelection(Number(item.id))}
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
              {:else if column.key === "preAssignedSubjects"}
                <td>
                  {#if item[column.key] && Array.isArray(item[column.key])}
                    {#each item[column.key] as subj, i}
                      {subj.name}{#if i < item[column.key].length - 1},
                      {/if}
                    {/each}
                  {:else}
                    N/A
                  {/if}
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
                on:click={() => handleAction(action, item)}
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
