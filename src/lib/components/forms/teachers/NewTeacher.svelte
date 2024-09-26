<script lang="ts">
  import "$styles/form/editor.scss";

  import { invoke } from "@tauri-apps/api";
  import { emit, listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  import { subjects, loadSubjects, type SubjectItem } from "$lib/modules/entities/subjectsStore";
  import { loadTeachers } from "$lib/modules/entities/teachersStore";

  let name = "";
  let father_lastname = "";
  let mother_lastname = "";
  let email = "";
  let phone = "";
  let degree = "";
  let comissioned_hours: number;
  let active_hours: number; // Horas activas es automatico con la suma total de las horas de las materias, no se puede editar
  let performance: number;

  let selectedSubjects: SubjectItem[] = [];
  let showSubjects = false;

  onMount(() => {
    loadSubjects();
    // Carga las materias cuando se actualizan
    listen("subjects_updated", async () => {
      await loadSubjects();
    });
  });

  // Para editar un profesor agregamos el item como propiedad
  // export const item: TeacherItem | null = null;

  // Manda la nueva materia a la base de datos en rust
  async function addTeacher() {
    if (!name || !father_lastname) {
      alert("Por favor, rellene todos los campos necesarios");
      return;
    }

    if (comissioned_hours < 0 || performance < 0) {
      alert("Por favor, rellene los campos con valores positivos");
      return;
    }

    if (selectedSubjects.length > 0) {
      emit("subjects_with_teachers_updated");
    }

    // Registrar nuevo profesor
    await invoke("add_teacher", {
      name,
      father_lastname,
      mother_lastname: mother_lastname || null,
      email: email || null,
      phone: phone || null,
      degree: degree || null,
      comissioned_hours: comissioned_hours || null,
      active_hours: active_hours || null,
      performance: performance || null,
      subjects: selectedSubjects.length > 0 ? selectedSubjects.map((s) => s.id) : null, // Pasamos solo los ids de las materias seleccionadas
    });
    await loadTeachers(); // Recarga las materias
    await emit("teachers_updated"); // Emite un evento para actualizar la vista de materias

    // Limpiamos los campos
    name = "";
    father_lastname = "";
    mother_lastname = "";
    email = "";
    phone = "";
    degree = "";
    comissioned_hours = 0;
    active_hours = 0;
    performance = 0;
    selectedSubjects = [];
  }

  // Cambia el estado de la materia seleccionada
  function toggleSelection(subject: SubjectItem) {
    const index = selectedSubjects.findIndex((s) => s.id === subject.id);
    if (index >= 0) {
      // Si la materia ya esta seleccionada la quitamos
      selectedSubjects = selectedSubjects.filter((s) => s.id !== subject.id);
    } else {
      // Si no esta seleccionada la agregamos
      selectedSubjects = [...selectedSubjects, subject];
    }
  }

  // Muestra las materias seleccionadas
  const showSelectedSubjects = () => showSubjects = !showSubjects;
</script>

<!-- Formulario para agregar un nuevo profesor -->
<section class="form-editor">
  <h1>Registro de profesores</h1>
  <div class="form-group">
    <div class="form-field">
      <label for="name"><img src="/icons/teacher.svg" alt="Nombre" /></label>
      <input
        type="text"
        placeholder="*Escribe nombre del profesor"
        id="name"
        bind:value={name}
        required />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/text.svg" alt="Apellido Paterno" /></label>
      <input
        type="text"
        placeholder="*Apellido Paterno"
        id="father_lastname"
        bind:value={father_lastname}
        required />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/text.svg" alt="Apellido Materno" /></label>
      <input
        type="text"
        placeholder="Apellido Materno"
        id="mother_lastname"
        bind:value={mother_lastname} />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/at.svg" alt="Correo" /></label>
      <input
        type="email"
        placeholder="Correo"
        id="email"
        bind:value={email} />
    </div>

    <div class="form-field">
      <label for="name"><img style="width: 20px;" src="/icons/phone.svg" alt="Teléfono" /></label>
      <input
        type="tel"
        placeholder="Teléfono"
        id="phone"
        bind:value={phone} />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/degree.svg" alt="Título" /></label>
      <input
        type="text"
        placeholder="Título"
        id="degree"
        bind:value={degree} />
    </div>

    <div class="form-field">
      <label for="name"><img style="width: 16px;" src="/icons/hourglass.svg" alt="Horas (comisión)" /></label>
      <input
        type="number"
        placeholder="Horas (comisión)"
        id="comissioned_hours"
        bind:value={comissioned_hours} />
    </div>

    <div class="form-field">
      <label for="name"><img src="/icons/chart.svg" alt="Rendimiento" /></label>
      <input
        type="number"
        placeholder="Rendimiento"
        id="performance"
        bind:value={performance} />
    </div>

    <!-- Aqui iran las materias que imparte el profesor -->
    <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
    <div class="form-field" style="cursor: pointer;" on:click={showSelectedSubjects}>
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
              id={subject.id.toString()}
              checked={selectedSubjects.some((s) => s.id === subject.id)}
              on:change={() => toggleSelection(subject)} />
            <label for={subject.id.toString()}>{subject.name}</label>
          </div>
        {/each}
      </div>
    {/if}

    <button class="form-button" on:click={addTeacher}>Agregar</button>
  </div>
</section>
