<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  type Subject = {
    id: number;
    name: string;
    shorten: string;
    color: string;
    stype: string;
  };

  let subject: Subject[] = [];

  async function getSubject() {
    subject = await invoke('list_subjects');
  }

  onMount(() => {
    getSubject();
  });
</script>

<div class="custom-table-container">
  <Table class="custom-table" hoverable={true} striped={true}>
    <TableHead>
      <TableHeadCell>Abreviacion</TableHeadCell>
      <TableHeadCell>Nombre</TableHeadCell>
      <TableHeadCell>Color</TableHeadCell>
      <TableHeadCell>Tipo</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each subject as item}
        <TableBodyRow>
          <TableBodyCell>{item.shorten}</TableBodyCell>
          <TableBodyCell>{item.name}</TableBodyCell>
          <TableBodyCell>{item.color}</TableBodyCell>
          <TableBodyCell>
            { item.stype === "" || item.stype === undefined ? 'No especifica' : item.stype }
          </TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</div>

<style>
  .custom-table-container {
    margin: 20px;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }
</style>
