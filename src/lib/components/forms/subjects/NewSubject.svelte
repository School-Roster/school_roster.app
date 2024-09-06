<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { loadSubjects } from "$lib/modules/entities/subjectsStore";
  import { emit } from "@tauri-apps/api/event";
  import { type SubjectItem } from "$lib/modules/entities/subjectsStore";

  let name = "";
  let shorten = "";
  let color = "";
  let spec = "Obligatoria";

  // Para editar una materia agregamos el item como propiedad
  export let item: SubjectItem | null = null;
  async function editSubject() {
    if (!item) return;
    if (!item.name || !item.shorten) {
      alert("Por favor, rellene todos los campos");
      return;
    }
    // TODO: Pasar el item directamente en vez de sus propiedades (mas limpio)
    await invoke("update_subject", { id: item.id, name: item.name, shorten: item.shorten, color: item.color, spec: item.spec });
    await loadSubjects();
    await emit("subjects_updated");
  }

  // Manda la nueva materia a la base de datos en rust
  async function addSubject() {
    if (!name || !shorten) {
      alert("Por favor, rellene todos los campos");
      return;
    }
    await invoke("create_subject", { name, shorten, color, spec });
    await loadSubjects(); // Recarga las materias
    await emit("subjects_updated"); // Emite un evento para actualizar la vista de materias
  }

</script>

<section class="form-container">
  <span class="title">Nueva Materia</span>
  {#if item}
    <div class="form">
      <label for="name">Nombre</label>
      <input type="text" id="name" bind:value={item.name} />
      <label for="shorten">Abreviatura</label>
      <input type="text" id="shorten" bind:value={item.shorten} />
      <label for="color">Color</label>
      <input type="color" id="color" bind:value={item.color} />
      <label for="spec">Tipo</label>
      <select id="spec" bind:value={item.spec}>
        <option value="Obligatoria">Obligatoria</option>
        <option value="Optativa">Optativa</option>
      </select>
      <button on:click={editSubject}>Editar materia</button>
    </div>
  {:else}
    <div class="form">
      <label for="name">Nombre</label>
      <input type="text" id="name" bind:value={name} />
      <label for="shorten">Abreviatura</label>
      <input type="text" id="shorten" bind:value={shorten} />
      <label for="color">Color</label>
      <input type="color" id="color" bind:value={color} />
      <label for="spec">Tipo</label>
      <select id="spec" bind:value={spec}>
        <option value="Obligatoria">Obligatoria</option>
        <option value="Optativa">Optativa</option>
      </select>
      <button on:click={addSubject}>Agregar</button>
    </div>
  {/if}
</section>
