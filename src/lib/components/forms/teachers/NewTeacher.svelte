<script lang="ts">
  import "$styles/form/editor.scss";

  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  import {
    subjects,
    loadSubjects,
    type SubjectItem,
  } from "$lib/modules/entities/subjectsStore";
  import {
    addTeacher,
    editTeacher,
    type TeacherItem,
  } from "$lib/modules/entities/teachersStore";
  import { addNotification } from "$lib/stores/notificationsStore";

  let t: TeacherItem = {
    name: "",
    father_lastname: "",
    mother_lastname: "",
    email: "",
    phone: "",
    degree: "",
    commisioned_hours: 0,
    active_hours: 0,
    performance: 0,
    preferred_days: [],
    preferred_modules: [],
  };

  let selectedSubjects: SubjectItem[] = [];
  let showSubjects: boolean = false;

  // Para editar un profesor agregamos el item como propiedad
  export let item: any | null = null;

  function initForm(item: any | null) {
    if (item) {
      t.id = item.id;
      t.name = item.name;
      t.father_lastname = item.father_lastname;
      t.mother_lastname = item.mother_lastname || "";
      t.email = item.email || "";
      t.phone = item.phone || "";
      t.degree = item.degree || "";
      t.commisioned_hours = item.commisioned_hours;
      t.active_hours = item.active_hours;
      t.performance = item.performance;
      t.preferred_days = item.preferred_days;
      t.preferred_modules = item.preferred_modules;
      // Map assigned_subjects names to the SubjectItem objects
      selectedSubjects = item.assigned_subjects.map((subjectName: string) => {
        const subject = $subjects.find((s) => s.name === subjectName);
        return subject ? subject : { id: -1, name: subjectName }; // Return a default if not found
      });
    } else {
      selectedSubjects = [];
    }
  }

  onMount((): void => {
    loadSubjects();
    initForm(item);
    // Carga las materias si se actualizan
    listen("subjects_updated", async () => {
      await loadSubjects();
    });
  });

  const handleSubmit = (): void => {
    if (item) {
      editTeacher(t, selectedSubjects);
      addNotification({
        message: "Edicion del profesor exitoso: El profesor ha sido modificado en el sistema.",
        type: "success",
        timeout: 1500,
      });
    } else {
      addTeacher(t, selectedSubjects);
      addNotification({
        message: "Registro exitoso: Profesor agregado al sistema.",
        type: "success",
        timeout: 1500,
      });
      // Limpiamos los campos
      t.name = "";
      t.father_lastname = "";
      t.mother_lastname = "";
      t.email = "";
      t.phone = "";
      t.degree = "";
      t.commisioned_hours = 0;
      t.active_hours = 0;
      t.performance = 0;
      selectedSubjects = [];
    }
  };

  // Cambia el estado de la materia seleccionada
  function toggleSelection(subject: SubjectItem): void {
    const index: number = selectedSubjects.findIndex(
      (s) => s.id === subject.id,
    );
    if (index >= 0) {
      // Si la materia ya esta seleccionada la quitamos
      selectedSubjects = selectedSubjects.filter((s) => s.id !== subject.id);
    } else {
      // Si no esta seleccionada la agregamos
      selectedSubjects = [...selectedSubjects, subject];
    }
  }

  // Muestra las materias seleccionadas
  const showSelectedSubjects = () => (showSubjects = !showSubjects);
</script>

<!-- Formulario para agregar un nuevo profesor -->
<section class="form-editor">
  <h1>{item ? "Editar Profesor" : "Registrar nuevo profesor"}</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="name"><img src="/icons/teacher.svg" alt="Nombre" /></label>
      <input
        type="text"
        placeholder="*Escribe nombre del profesor"
        id="name"
        bind:value={t.name}
        required
      />
    </div>

    <div class="form-field">
      <label for="name"
        ><img src="/icons/text.svg" alt="Apellido Paterno" /></label
      >
      <input
        type="text"
        placeholder="*Apellido Paterno"
        id="father_lastname"
        bind:value={t.father_lastname}
        required
      />
    </div>

    <div class="form-field">
      <label for="name"
        ><img src="/icons/text.svg" alt="Apellido Materno" /></label
      >
      <input
        type="text"
        placeholder="Apellido Materno"
        id="mother_lastname"
        bind:value={t.mother_lastname}
      />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/at.svg" alt="Correo" /></label>
      <input type="email" placeholder="Correo" id="email" bind:value={t.email} />
    </div>

    <div class="form-field">
      <label for="name"
        ><img
          style="width: 20px;"
          src="/icons/phone.svg"
          alt="Teléfono"
        /></label
      >
      <input
        type="tel"
        placeholder="Teléfono"
        id="phone"
        bind:value={t.phone}
      />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/degree.svg" alt="Título" /></label>
      <input
        type="text"
        placeholder="Título"
        id="degree"
        bind:value={t.degree}
      />
    </div>

    <div class="form-field">
      <label for="name"
        ><img
          style="width: 16px;"
          src="/icons/hourglass.svg"
          alt="Horas (comisión)"
        /></label
      >
      <input
        type="number"
        placeholder="Horas (comisión)"
        id="comissioned_hours"
        bind:value={t.commisioned_hours}
      />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/chart.svg" alt="Rendimiento" /></label>
      <input
        type="number"
        placeholder="Rendimiento"
        id="performance"
        bind:value={t.performance}
      />
    </div>

    <div class="form-field">
      <label for="name"
        ><img src="/icons/chart.svg" alt="DiasPreferidos" /></label
      >
      <input
        type="text"
        placeholder="Dias preferidos del profesor (Opcional)"
        id="preferred_days"
        bind:value={t.preferred_days}
      />
    </div>

    <div class="form-field">
      <label for="name"
        ><img src="/icons/chart.svg" alt="DiasPreferidos" /></label
      >
      <input
        type="number"
        placeholder="Modulos preferidos del profesor (Opcional)"
        id="preferred_days"
        bind:value={t.preferred_modules}
      />
    </div>

    <!-- Aqui iran las materias que imparte el profesor -->
    <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
    <div
      class="form-field"
      style="cursor: pointer;"
      on:click={showSelectedSubjects}
    >
      <label for="name"><img src="/icons/books.svg" alt="Materias" /></label>
      <!-- Muestra las materias seleccionadas -->
      {#if selectedSubjects.length > 0}
        {#each selectedSubjects as subject}
          <span class="form-name">{subject.name}</span>
        {/each}
      {:else}
        <span>Seleccione materias</span>
      {/if}
    </div>
    <!-- Lista de materias -->
    {#if showSubjects}
      <div class="subject-list">
        {#each $subjects as subject}
          <div class="subject-item">
            <input
              type="checkbox"
              class="form-checkbox"
              id={subject.id == undefined ? "" : subject.id.toString()}
              checked={selectedSubjects.some((s) => s.id === subject.id)}
              on:change={() => toggleSelection(subject)}
            />
            <label for={subject.id == undefined ? "" : subject.id.toString()}>{subject.name}</label>
          </div>
        {/each}
      </div>
    {/if}

    <button class="form-button" on:click={handleSubmit}
      >{item ? "Editar profesor" : "Registrar profesor"}</button
    >
  </div>
</section>
