<script>
  // @ts-nocheck

  import {
    Dropdown,
    DropdownItem,
    Checkbox,
    Button,
    Search,
  } from "flowbite-svelte";
  import { ChevronDownOutline, UserRemoveSolid } from "flowbite-svelte-icons";
</script>

<Button
  >Materias<ChevronDownOutline
    class="w-6 h-6 ms-2 text-white dark:text-white"
  /></Button
>
<Dropdown class="overflow-y-auto px-3 pb-3 text-sm h-44">
  <div slot="header" class="p-3">
    <Search size="md" />
  </div>
  <li class="rounded p-2 hover:bg-gray-100 dark:hover:bg-gray-600">
    <Checkbox>Matematicas</Checkbox>
  </li>
  <li class="rounded p-2 hover:bg-gray-100 dark:hover:bg-gray-600">
    <Checkbox>Programacion</Checkbox>
  </li>
  <li class="rounded p-2 hover:bg-gray-100 dark:hover:bg-gray-600">
    <Checkbox checked>Historia</Checkbox>
  </li>
  <li class="rounded p-2 hover:bg-gray-100 dark:hover:bg-gray-600">
    <Checkbox>Fisica</Checkbox>
  </li>
  <li class="rounded p-2 hover:bg-gray-100 dark:hover:bg-gray-600">
    <Checkbox>Arte</Checkbox>
  </li>
  <li class="rounded p-2 hover:bg-gray-100 dark:hover:bg-gray-600">
    <Checkbox>Ingles</Checkbox>
  </li>
  <a
    slot="footer"
    href="/"
    class="flex items-center p-3 -mb-1 text-sm font-medium text-red-600 bg-gray-50 hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-red-500 hover:underline"
  >
    <UserRemoveSolid class="w-5 h-5 me-1" />Borrar materia
  </a>
</Dropdown>
