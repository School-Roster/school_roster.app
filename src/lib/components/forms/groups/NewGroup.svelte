<script lang="ts">
  import "$styles/form/editor.scss";

  import { invoke } from "@tauri-apps/api";
  import {
    loadGroups,
    type GroupItem,
  } from "$lib/modules/entities/groupsStore";
  import { emit } from "@tauri-apps/api/event";

  let grade: number;
  let group: string;
  let career: string;
  let students: number;

  // Para editar se le pasa el item
  export let item: GroupItem | null = null;
  async function editGroup() {
    if (!item) return;
    if (!item.grade || !item.group) {
      alert("Por favor, rellene todos los campos");
      return;
    }
    // TODO: Pasar el item directamente en vez de sus propiedades (mas limpio)
    await invoke("update_group", {
      id: item.id,
      grade: item.grade,
      group: item.group,
      career: item.career,
      students: item.students,
    });
    await loadGroups();
    await emit("groups_updated");
  }

  async function addGroup() {
    if (!grade || !group) {
      alert("Por favor, rellene todos los campos");
      return;
    }

    await invoke("create_group", {
      grade,
      group,
      career: career || null,
      students: students || null,
    });
    await loadGroups(); // Recarga las vistas
    await emit("groups_updated"); // Emite un evento para actualizar la vista de materias

    // Limpiamos los campos
    grade = 0;
    group = "";
    career = "";
    students = 0;
  }
</script>

<section class="form-editor">
  {#if item}
    <h1>Editar Grupo</h1>
    <div class="form-group">
      <div class="form-field">
        <label for="grade"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="* Grado"
          id="grade"
          bind:value={item.grade}
        />
      </div>

      <div class="form-field">
        <label for="group"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="* Grupo"
          id="group"
          bind:value={item.group}
        />
      </div>

      <div class="form-field">
        <label for="career"><img src="/icons/books.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="Especialidad o carrera"
          id="career"
          bind:value={item.career}
        />
      </div>

      <button class="form-button" on:click={editGroup}>Editar</button>
    </div>
  {:else}
    <h1>Generar nuevo grupo</h1>
    <div class="form-group">
      <div class="form-field">
        <label for="grade"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="number"
          placeholder="* Grado"
          id="grade"
          bind:value={grade}
        />
      </div>

      <div class="form-field">
        <label for="group"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="* Grupo"
          id="group"
          bind:value={group}
        />
      </div>

      <div class="form-field">
        <label for="career"><img src="/icons/books.svg" alt="Icon" /></label>
        <input
          type="number"
          placeholder="Especialidad o carrera"
          id="career"
          bind:value={career}
        />
      </div>

      <button class="form-button" on:click={addGroup}>Agregar</button>
    </div>
  {/if}
</section>
