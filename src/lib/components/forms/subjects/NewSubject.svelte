<script lang="ts">
  import "$styles/form/editor.scss";

  import {
    addSubject,
    editSubject,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";
  import ColorPicker from "$lib/components/buttons/ColorPicker.svelte";

  let name: string = "";
  let shorten: string = "";
  /*
    TODO: Generar un color que pueda usarse sin cambiarlo para no perder el tiempo
    NOTE: Tendremos en cuenta los colores registrados en la base de datos
      para que no se repitan y darle al usuario una recomendación de color
  */
  let color: string = "#ff00aa";
  let spec: string = "";

  // Para editar una materia agregamos el item como propiedad
  export let item: SubjectItem | null = null;

  const handleSubmit = (): void => {
    if (item) {
      editSubject(item);
    } else {
      addSubject(name, shorten, color, spec);

      // Limpiamos los campos
      name = "";
      shorten = "";
      color = "#a50044";
      spec = "Obligatoria";
    }
  };
</script>

<section class="form-editor">
  <h1>{item ? "Editar materia existente" : "Agregar nueva materia"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="name"><img src="/icons/books.svg" alt="Materia" /></label>
      {#if item}
        <input
          type="text"
          placeholder="Escribe nombre de materia"
          id="name"
          bind:value={item.name}
        />
      {:else}
        <input
          type="text"
          placeholder="Escribe nombre de materia"
          id="name"
          bind:value={name}
        />
      {/if}
    </div>
    <div class="form-field">
      <label for="name"><img src="/icons/text.svg" alt="Materia" /></label>
      {#if item}
        <input
          type="text"
          placeholder="Abreviatura"
          id="shorten"
          bind:value={item.shorten}
        />
      {:else}
        <input
          type="text"
          placeholder="Abreviatura"
          id="shorten"
          bind:value={shorten}
        />
      {/if}
    </div>

    <div class="form-field">
      {#if item}
        <ColorPicker bind:value={item.color} />
      {:else}
        <ColorPicker bind:value={color} />
      {/if}
    </div>
    <div class="form-field">
      <label for="spec">Tipo</label>
      {#if item}
        <select id="spec" bind:value={item.spec}>
          <option class="opt" value="Obligatoria">Obligatoria</option>
          <option class="opt" value="Optativa">Optativa</option>
        </select>
      {:else}
        <select id="spec" bind:value={spec}>
          <option class="opt" value="Obligatoria">Obligatoria</option>
          <option class="opt" value="Optativa">Optativa</option>
        </select>
      {/if}
    </div>
    <button class="form-button" on:click={handleSubmit}>
      {item ? "Editar materia" : "Agregar materia"}
    </button>
  </div>
</section>
