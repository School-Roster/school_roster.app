<script lang="ts">
  import "$styles/form/editor.scss";

  import { invoke } from "@tauri-apps/api";
  import {
    loadClassrooms,
    type ClassroomItem,
  } from "$lib/modules/entities/classroomStore";
  import { emit } from "@tauri-apps/api/event";

  let building_id: string;
  let building_number: number;
  let building_type: string;
  let capacity: number;

  // Para editar se le pasa el item
  export let item: ClassroomItem | null = null;
  async function editClassroom() {
    if (!item) return;
    if (!item.building_id || !item.building_number) {
      alert("Por favor, rellene todos los campos");
      return;
    }
    // TODO: Pasar el item directamente en vez de sus propiedades (mas limpio)
    await invoke("update_classroom", {
      id: item.id,
      building_id: item.building_id,
      building_number: item.building_number,
      building_type: item.building_type,
      capacity: item.capacity,
    });
    await loadClassrooms();
    await emit("classrooms_updated");
  }

  async function addClassroom() {
    if (!building_number || !building_id) {
      alert("Por favor, rellene todos los campos");
      return;
    }

    await invoke("create_classroom", {
      building_id,
      building_number,
      building_type: building_type || null,
      capacity: capacity || null,
    });
    await loadClassrooms(); // Recarga las vistas
    await emit("classrooms_updated"); // Emite un evento para actualizar la vista de materias

    // Limpiamos los campos
    building_number = 0;
    building_id = "";
    building_type = "";
    capacity = 0;
  }
</script>

<section class="form-editor">
  {#if item}
    <h1>Editar aula</h1>
    <div class="form-group">
      <div class="form-field">
        <label for="building_id"><img src="/icons/door.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="* ID del edificio"
          id="building_id"
          bind:value={item.building_id}
        />
      </div>
      <div class="form-field">
        <label for="building_number"><img src="/icons/door.svg" alt="Icon" /></label>
        <input
          type="number"
          placeholder="* Numero de aula"
          id="building_number"
          bind:value={item.building_number}
        />
      </div>

      <div class="form-field">
        <label for="building_type"><img src="/icons/door.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="Tipo de edificio"
          id="building_type"
          bind:value={item.building_type}
        />
      </div>

      <div class="form-field">
        <label for="capacity"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="number"
          placeholder="Capacidad del aula"
          id="building_type"
          bind:value={item.capacity}
        />
      </div>

      <button class="form-button" on:click={editClassroom}>Editar</button>
    </div>
  {:else}
    <h1>Generar nueva aula</h1>
    <div class="form-group">
      <div class="form-field">
        <label for="building_id"><img src="/icons/door.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="* ID del edificio"
          id="building_id"
          bind:value={building_id}
        />
      </div>
      <div class="form-field">
        <label for="building_number"><img src="/icons/door.svg" alt="Icon" /></label>
        <input
          type="number"
          placeholder="* Numero de aula"
          id="building_number"
          bind:value={building_number}
        />
      </div>

      <div class="form-field">
        <label for="building_type"><img src="/icons/door.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="Tipo de edificio"
          id="building_type"
          bind:value={building_type}
        />
      </div>

      <div class="form-field">
        <label for="capacity"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="number"
          placeholder="Capacidad del aula"
          id="building_type"
          bind:value={capacity}
        />
      </div>
      <button class="form-button" on:click={addClassroom}>Agregar</button>
    </div>
  {/if}
</section>
