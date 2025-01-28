<script lang="ts">
  import {
    addClassroom,
    editClassroom,
    // addClassroom,
    // editClassroom,
    loadClassrooms,
    type ClassroomItem,
  } from "$lib/modules/entities/classroomStore";

  // Variables
  let building_id: string;
  let building_number: number;
  let building_type: string;
  let capacity: number;

  // Para editar se le pasa el item
  export let item: ClassroomItem | null = null;

  const handleSubmit = (): void => {
    if (item) {
      editClassroom(item);
    } else {
      addClassroom(building_id, building_number, building_type, capacity);
    }
  };
</script>

<section class="form-editor">
  <h1>{item ? "Editar aula existente" : "Generar nueva aula"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="building_id"><img src="/icons/door.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="text"
          placeholder="* ID del edificio"
          id="building_id"
          bind:value={item.building_id}
        />
      {:else}
        <input
          type="text"
          placeholder="* ID del edificio"
          id="building_id"
          bind:value={building_id}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="building_number"
        ><img src="/icons/door.svg" alt="Icon" /></label
      >
      {#if item}
        <input
          type="number"
          placeholder="* Numero de aula"
          id="building_number"
          bind:value={item.building_number}
        />
      {:else}
        <input
          type="number"
          placeholder="* Numero de aula"
          id="building_number"
          bind:value={building_number}
        />
      {/if}
    </div>

    <div class="form-field">
      <label for="building_type"><img src="/icons/door.svg" alt="Icon" /></label
      >
      {#if item}
        <input
          type="text"
          placeholder="Tipo de edificio"
          id="building_type"
          bind:value={item.building_type}
        />
      {:else}
        <input
          type="text"
          placeholder="Tipo de edificio"
          id="building_type"
          bind:value={building_type}
        />
      {/if}
    </div>

    <div class="form-field">
      <label for="capacity"><img src="/icons/group.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="number"
          placeholder="Capacidad del aula"
          id="building_type"
          bind:value={item.capacity}
        />
      {:else}
        <input
          type="number"
          placeholder="Capacidad del aula"
          id="building_type"
          bind:value={capacity}
        />
      {/if}
    </div>
  </div>
  <button class="form-button" on:click={handleSubmit}>
    {item ? "Editar" : "Agregar"}
  </button>
</section>
