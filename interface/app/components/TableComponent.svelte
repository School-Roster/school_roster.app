<script lang="ts">
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  export let data: any[] = [];
  export let columns: string[] = [];
  export let columnKeys: string[] = [];
</script>

<div class="custom-table-container">
  <Table class="custom-table" hoverable={true} striped={true}>
    <TableHead>
      {#each columns as column}
        <TableHeadCell>{column}</TableHeadCell>
      {/each}
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each data as item}
        <TableBodyRow>
          {#each columnKeys as key}
            <TableBodyCell>
              {item[key] === "" || item[key] === undefined ? 'No especifica' : item[key]}
            </TableBodyCell>
          {/each}
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</div>

<style>
  .custom-table-container {
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }
</style>
