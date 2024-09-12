<script lang="ts">
  import "$styles/table.scss";
  import { getContrastColor } from "$lib/utilities/helpers";

  export let data: any[] = [];
  export let columns: any[] = [];
  export let actions: any[] = [];
</script>

<section class="table-container">
  <table>
    <thead>
      <tr>
        {#each columns as column (column.name)}
          <th>{column.name}</th>
        {/each}
        <th>Acciones</th>
      </tr>
    </thead>
    <tbody>
      {#each data as item (item.id)}
        <tr>
          {#each columns as column (column.key)}
            {#if column.key === "color"}
              <td class="table-color"
                style="background-color: {item[column.key]}; color: {getContrastColor(item[column.key])}"
              >
                {item[column.key]}
              </td>
            {:else}
              <td>{item[column.key]}</td>
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
