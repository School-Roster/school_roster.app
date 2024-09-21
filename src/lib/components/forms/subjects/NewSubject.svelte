<script lang="ts">
  import "$styles/form/editor.scss";

  import { invoke } from "@tauri-apps/api";
  import { loadSubjects } from "$lib/modules/entities/subjectsStore";
  import { emit } from "@tauri-apps/api/event";
  import { type SubjectItem } from "$lib/modules/entities/subjectsStore";
  import ColorPicker from "$lib/components/buttons/ColorPicker.svelte";

  let name = "";
  let shorten = "";

  /*
    TODO: Tendremos en cuenta los colores registrados en la base de datos
      para que no se repitan y darle al usuario una recomendación de color
  */
  let color = "#ff00aa";

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
    if (!name) {
      alert("Por favor, rellene todos los campos");
      return;
    }
    if (!shorten) {
      shorten = name.substring(0, 3).toUpperCase();
    }

    await invoke("create_subject", { name, shorten, color, spec });
    await loadSubjects(); // Recarga las materias
    await emit("subjects_updated"); // Emite un evento para actualizar la vista de materias

    // Limpiamos los campos
    name = "";
    shorten = "";
    color = "#a50044";
    spec = "Obligatoria";
  }

  // TODO: Cambiar estilo de <option> para que se vea mejor
  // TODO: En el select de tipo tenemos que agregar opcion para registrar nuevo tipo

</script>

<section class="form-editor">
  {#if item}
  <h1>Editar Materia</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="name"><img src="/icons/books.svg" alt="Materia" /></label>
      <input
        type="text"
        placeholder="Escribe nombre de materia"
        id="name"
        bind:value={item.name} />
    </div>
    
    <div class="form-field">
      <label for="name"><img src="/icons/text.svg" alt="Materia" /></label>
      <input
        type="text"
        placeholder="Abreviatura"
        id="shorten"
        bind:value={item.shorten} />
    </div>
    
    <div class="form-field">
      <ColorPicker bind:value={item.color} />
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
          required />
      </div>

      <div class="form-field">
        <label for="name"><img src="/icons/text.svg" alt="Materia" /></label>
        <input
          type="text"
          placeholder="Abreviatura (opcional)"
          id="shorten"
          bind:value={shorten} />
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
