<script lang="ts">
  import {
    addClassroom,
    editClassroom,
    type ClassroomItem,
  } from "$lib/modules/entities/classroomStore";
  import { onMount } from "svelte";

  let cr: ClassroomItem = {
    building_id: "",
    building_number: null,
    building_type: "",
    capacity: null,
    available: false,
  };

  // Para editar se le pasa el item
  export let item: ClassroomItem | null = null;

  onMount((): void => {
    initForm(item);
  });

  function initForm(item: ClassroomItem | null): void {
    if (item) {
      cr.building_id = item.building_id;
      cr.building_number = item.building_number;
      cr.building_type = item.building_type;
      cr.capacity = item.capacity;
      cr.available = item.available;
    }
  }

  const handleSubmit = (): void => {
    if (item) {
      editClassroom(cr);
    } else {
      addClassroom(cr);
    }
    // Limpiamos los campos
    cr.building_number = 0;
    cr.building_id = "";
    cr.building_type = "";
    cr.capacity = 0;
  };
</script>

<section class="form-editor">
  <h1>{item ? "Editar aula existente" : "Generar nueva aula"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="building_id"><img src="/icons/door.svg" alt="Icon" /></label>
      <input
        type="text"
        placeholder="* ID del edificio"
        id="building_id"
        bind:value={cr.building_id}
      />
    </div>
    <div class="form-field">
      <label for="building_number"
        ><img src="/icons/door.svg" alt="Icon" /></label
      >
      <input
        type="number"
        placeholder="* Numero de aula"
        id="building_number"
        bind:value={cr.building_number}
      />
    </div>

    <div class="form-field">
      <label for="building_type"><img src="/icons/door.svg" alt="Icon" /></label
      >
      <input
        type="text"
        placeholder="Tipo de edificio"
        id="building_type"
        bind:value={cr.building_type}
      />
    </div>

    <div class="form-field">
      <label for="capacity"><img src="/icons/group.svg" alt="Icon" /></label>
      <input
        type="number"
        placeholder="Capacidad del aula"
        id="building_type"
        bind:value={cr.capacity}
      />
    </div>
    <button class="form-button" on:click={handleSubmit}>
      {item ? "Editar" : "Agregar nueva aula"}
    </button>
  </div>
</section>
