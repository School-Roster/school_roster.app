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

  type Teacher = {
    id: number;
    shorten: string;
    name: string;
    first_last_name: string;
    second_last_name: string;
    email: string;
    phone: string;
    degree: string;
    commissioned_hours: number;
    active_hours: number;
    stars: number;
  };

  let teachers: Teacher[] = [];

  async function getTeachers() {
    teachers = await invoke('list_teachers');
  }

  onMount(() => {
    getTeachers();
  });
</script>

<div class="custom-table-container">
  <Table class="custom-table" color="blue" hoverable={true}>
    <TableHead>
      <TableHeadCell>Shorten</TableHeadCell>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>First Last Name</TableHeadCell>
      <TableHeadCell>Second Last Name</TableHeadCell>
      <TableHeadCell>Email</TableHeadCell>
      <TableHeadCell>Commissioned Hours</TableHeadCell>
      <TableHeadCell>Active Hours</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each teachers as teacher}
        <TableBodyRow>
          <TableBodyCell>{teacher.shorten}</TableBodyCell>
          <TableBodyCell>{teacher.name}</TableBodyCell>
          <TableBodyCell>{teacher.first_last_name}</TableBodyCell>
          <TableBodyCell>{teacher.second_last_name}</TableBodyCell>
          <TableBodyCell>{teacher.email}</TableBodyCell>
          <TableBodyCell>{teacher.commissioned_hours}</TableBodyCell>
          <TableBodyCell>{teacher.active_hours}</TableBodyCell>
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
