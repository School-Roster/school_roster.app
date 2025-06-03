<script lang="ts">
  import "$styles/form/editor.scss";
  import { usedColors } from "$lib/stores/usedColors";

  import {
    addSubject,
    editSubject,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";
  import ColorPicker from "$lib/components/buttons/ColorPicker.svelte";
  import { onMount } from "svelte";
  import TooltipIcon from "$lib/components/buttons/TooltipIcon.svelte";
  import { get } from "svelte/store";
	/*
    TODO: Generar un color que pueda usarse sin cambiarlo para no perder el tiempo
    NOTE: Tendremos en cuenta los colores registrados en la base de datos
      para que no se repitan y darle al usuario una recomendación de color
  */
  let subject: SubjectItem = {
    name: "",
    shorten: "",
    color: "#a50044",
    spec: "",
    required_modules: null,
    priority: null,
  };
	  // Para editar una materia agregamos el item como propiedad
  export let item: SubjectItem | null = null;

  function initForm(item: SubjectItem | null): void {
    if (item) {
      subject.id = item.id;
      subject.name = item.name;
      subject.shorten = item.shorten;
      subject.color = item.color;
      subject.required_modules = item.required_modules;
      subject.priority = item.priority;
      subject.assigned_teacher = item.assigned_teacher;
    }
  }

  const flatColors = [
    '#DAAFE9', '#C7DBF5', '#AAD5FB', '#ADE5DA', '#B0EDC3', '#FDF0A4', '#F8D6A2',
    '#C47ADA', '#90BAEE', '#75BAFA', '#72D5BF', '#73DE8C', '#FBE66E', '#F5B969',
    '#AE44B7', '#5E7ABC', '#4DACA9', '#63B75A', '#EDBD4A', '#EC9740',
    '#501B87', '#021B6B', '#0C2794', '#337277', '#2F6A52', '#AE802F', '#AD6127'
  ];

  onMount((): void => {
    initForm(item);

    if (!item) {
      const used = get(usedColors);
      const available = flatColors.filter(c => !used.includes(c));
      if (available.length > 0) {
        const randomIndex = Math.floor(Math.random() * available.length);
        subject.color = available[randomIndex];
      }
    }
  });

  function assignRandomColor() {
    const used = get(usedColors);
    const available = flatColors.filter(c => !used.includes(c));
    if (available.length > 0) {
      const randomIndex = Math.floor(Math.random() * available.length);
      subject.color = available[randomIndex];
    } else {
      subject.color = flatColors[Math.floor(Math.random() * flatColors.length)];
    }
  }

  const handleSubmit = (): void => {
    if (item) {
      editSubject(subject);
    } else {
      addSubject(subject);

      usedColors.update((colors) => {
        if (!colors.includes(subject.color)) {
          return [...colors, subject.color];
        }
        return colors;
      });

      subject.name = "";
      subject.shorten = "";
      subject.spec = "";
      subject.required_modules = null;
      subject.priority = null;

      assignRandomColor();
    }
  };
</script>

<section class="form-editor">
  <h1>{item ? "Editar materia existente" : "Agregar nueva materia"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="name"><img src="/icons/books.svg" alt="Materia" /></label>
      <input
        type="text"
        placeholder="* Escribe nombre de materia"
        id="name"
        bind:value={subject.name}
      />
    </div>
    <div class="form-field">
      <label for="name"><img src="/icons/text.svg" alt="Materia" /></label>
      <input
        type="text"
        placeholder="Abreviatura (opcional)"
        id="shorten"
        bind:value={subject.shorten}
      />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/text.svg" alt="Materia" /></label>
      <input
        type="number"
        placeholder="* Modulos a la semana"
        id="required_modules"
        bind:value={subject.required_modules}
        on:input={() =>
          (subject.required_modules = subject.required_modules ?? 0)}
      />
      <TooltipIcon
        description="Este campo es necesario para generar el horario automaticamente."
      />
    </div>

    <div class="form-field">
      <ColorPicker bind:value={subject.color} />
    </div>
    <div class="form-field">
      <label for="spec">Tipo</label>
      <input
        type="text"
        placeholder="Especialidad"
        id="spec"
        bind:value={subject.spec}
      />
      <!-- TODO: Agregar especializaciones desde pantalla de configuracion -->
      <!--
      <select id="spec" bind:value={subject.spec}>
        <option class="opt" value="Obligatoria">Obligatoria</option>
        <option class="opt" value="Optativa">Optativa</option>
      </select>
      -->
    </div>
    <div class="form-field">
      <label for="name"><img src="/icons/text.svg" alt="Materia" /></label>
      <input
        type="number"
        placeholder="Prioridad de la materia (opcional)"
        id="priority"
        bind:value={subject.priority}
        on:input={() => (subject.priority = subject.priority ?? 0)}
      />
      <TooltipIcon
        description="Las materias con mayor prioridad se asignaran en los primeros modulos del dia (ejemplo: 5 -mayor prioridad-)."
      />
    </div>
    <button class="form-button" on:click={handleSubmit}>
      {item ? "Editar materia" : "Agregar materia"}
    </button>
  </div>
</section>
