<script lang="ts">
  import "$styles/form/editor.scss";

  import { invoke } from "@tauri-apps/api";
  import { loadGroups, type GroupItem } from "$lib/modules/entities/groupStore";
  import { emit } from "@tauri-apps/api/event";


  let grade: number;
  let group: string;
  let career: string;
  let students: string[];

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
      students: item.students
    });
    await loadGroups();
    await emit("groups_updated");
  }

  async function addGroup() {
    if (!grade || !group) {
      alert("Por favor, rellene todos los campos");
      return;
    }

    // await invoke("create_subject", { name, shorten, color, spec });
    await invoke("create_group", { grade, group, career, students });
    await loadGroups(); // Recarga las vistas
    await emit("groups_updated"); // Emite un evento para actualizar la vista de materias

    // Limpiamos los campos
    grade = 0;
    group = "";
    career = "";
    students = [];
  }
</script>

<section class="form-editor">
  {#if item}
    <h1>Editar Grupo</h1>
    <div class="form-group">
      <div class="form-field">
        <label for="name"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="* Grado"
          id="grade"
          bind:value={item.grade}
        />
      </div>

      <div class="form-field">
        <label for="name"><img src="/icons/group.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="* Grupo"
          id="group"
          bind:value={item.group}
        />
      </div>

      <div class="form-field">
        <label for="name"><img src="/icons/books.svg" alt="Icon" /></label>
        <input
          type="text"
          placeholder="Especialidad o carrera"
          id="career"
          bind:value={item.career}
        />
      </div>

      <div class="form-field">
        <label for="spec">Tipo</label>
        <select id="spec" bind:value={item.spec}>
          <option class="opt" value="Obligatoria">Obligatoria</option>
          <option class="opt" value="Optativa">Optativa</option>
        </select>
      </div>

      <button class="form-button" on:click={editSubject}>Editar</button>
    </div>
  {:else}
    <h1>Nueva Materia</h1>
    <div class="form-group">
      <div class="form-field">
        <label for="name"><img src="/icons/books.svg" alt="Materia" /></label>
        <input
          type="text"
          placeholder="*Escribe nombre de materia"
          id="name"
          bind:value={name}
          required
        />
      </div>

      <div class="form-field">
        <label for="name"><img src="/icons/text.svg" alt="Materia" /></label>
        <input
          type="text"
          placeholder="Abreviatura (opcional)"
          id="shorten"
          bind:value={shorten}
        />
      </div>

      <div class="form-field">
        <ColorPicker bind:value={color} />
      </div>

      <div class="form-field">
        <label for="spec">Tipo</label>
        <select id="spec" bind:value={spec}>
          <option class="opt" value="Obligatoria">Obligatoria</option>
          <option class="opt" value="Optativa">Optativa</option>
        </select>
      </div>

      <button class="form-button" on:click={addSubject}>Agregar</button>
    </div>
  {/if}
</section>
