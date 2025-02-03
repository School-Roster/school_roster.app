<script lang="ts">
  import {
    addGroup,
    editGroup,
    type GroupItem,
  } from "$lib/modules/entities/groupsStore";

  // Variables locales
  let grade: number;
  let group: string;
  let career: string;
  let students: number;

  // Para editar se le pasa el item
  export let item: GroupItem | null = null;

  const handleSubmit = (): void => {
    if (item) {
      editGroup(item);
    } else {
      addGroup(grade, group, career, students);
    }
  };
</script>

<section class="form-editor">
  <h1>{item ? "Editar grupo existente" : "Generar nuevo grupo"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="grade"><img src="/icons/group.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="text"
          placeholder="* Grado"
          id="grade"
          bind:value={item.grade}
        />
      {:else}
        <input
          type="number"
          placeholder="* Grado"
          id="grade"
          bind:value={grade}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="group"><img src="/icons/group.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="text"
          placeholder="* Grupo"
          id="group"
          bind:value={item.group}
        />
      {:else}
        <input
          type="text"
          placeholder="* Grupo"
          id="group"
          bind:value={group}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="career"><img src="/icons/books.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="text"
          placeholder="Especialidad o carrera"
          id="career"
          bind:value={item.career}
        />
      {:else}
        <input
          type="text"
          placeholder="Especialidad o carrera"
          id="career"
          bind:value={career}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="students"><img src="/icons/group.svg" alt="Icon" /></label>
      {#if item}
        <input
          type="number"
          placeholder="Cantidad de alumnos (opcional)"
          id="career"
          bind:value={item.students}
        />
      {:else}
        <input
          type="number"
          placeholder="Cantidad de alumnos (opcional)"
          id="career"
          bind:value={students}
        />
      {/if}
    </div>
    <button class="form-button" on:click={handleSubmit}>
      {item ? "Editar grupo" : "Agregar grupo"}
    </button>
  </div>
</section>
