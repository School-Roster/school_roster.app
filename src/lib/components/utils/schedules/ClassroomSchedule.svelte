<script lang="ts">
  import {
    assignmentsStore,
    loadAssignments,
  } from "$lib/modules/entities/assignments";
  import { teachers, loadTeachers } from "$lib/modules/entities/teachersStore";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
  import { classrooms, loadClassrooms } from "$lib/modules/entities/classroomStore";
  import { onMount } from "svelte";
  import jsPDF from "jspdf";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writeBinaryFile } from "@tauri-apps/api/fs";
  import { configStore, loadConfig, loadSchoolInfo } from '$lib/modules/config/configStore';
  import { schoolStore } from '$lib/modules/config/configStore';
  import html2canvas from "html2canvas";
  import { addNotification } from "$lib/stores/notificationsStore";

  let selectedClassroomId: number | null = null;
  let schoolName = "";
  let schoolLogoPath: string | null = null;

  // Helper function to calculate time slots based on config
  function generateTimeSlots(config: any): string[] {
    const slots: string[] = [];
    let currentTime = new Date();
    currentTime.setHours(7, 0, 0, 0); // Start at 7:00 AM by default

    const durationMs =
      config.durationUnit === "hours"
        ? config.moduleDuration * 60 * 60 * 1000
        : config.moduleDuration * 60 * 1000;

    const breakDurationMs = config.breakDuration * 60 * 1000;

    for (let i = 0; i < config.modulesPerDay; i++) {
      const startTime = new Date(currentTime);
      currentTime = new Date(currentTime.getTime() + durationMs);

      const startStr = startTime.toLocaleTimeString("es-MX", {
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      });
      const endStr = currentTime.toLocaleTimeString("es-MX", {
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      });

      slots.push(`${startStr} - ${endStr}`);

      // Add break time if needed
      if (config.hasBreaks && config.breakPositions.includes(i)) {
        currentTime = new Date(currentTime.getTime() + breakDurationMs);
      }
    }

    return slots;
  }

  // Create day mapping dynamically based on config
  function createDayMap(days: string[]): Record<string, number> {
    const map: Record<string, number> = {};
    days.forEach((day, index) => {
      map[day.toLowerCase()] = index + 1;
    });
    return map;
  }

  onMount(async () => {
    await loadConfig();
    await loadAssignments();
    await loadTeachers();
    await loadGroups();
    await loadClassrooms();
    await loadSchoolInfo();
    schoolStore.subscribe((info) => {
      schoolName = info.name;
      schoolLogoPath = info.logo_path;
    });
  });

  // Reactive values
  $: days = $configStore.days;
  $: dayMap = createDayMap(days);
  $: horas = generateTimeSlots($configStore);
  $: assignmentsMap = $assignmentsStore;
  $: teachersList = $teachers;
  $: groupsList = $groups;
  $: classroomList = $classrooms;

  function getClassroomName(id: number): string {
    const classroom = classroomList.find((c) => c.id === id);
    if (!classroom) return "Aula no encontrada";
    return `${classroom.building_id || ''}${classroom.building_number}`;
  }

  function getTeacher(id: number): string {
    const teacher = teachersList.find((t) => t.id === id);
    return teacher
      ? `${teacher.name} ${teacher.father_lastname}`
      : "Maestro no encontrado";
  }

  function getGroup(id: number): string {
    const group = groupsList.find((g) => g.id === id);
    return group ? `${group.grade}${group.group}` : "Grupo no encontrado";
  }

  $: classroomSchedule = selectedClassroomId
    ? Array.from(assignmentsMap.values()).filter(
        (a) => a.classroomId === selectedClassroomId,
      )
    : [];

  function findAssignment(dayIndex: number, moduleIndex: number) {
    return classroomSchedule.find((a) => {
      const dayKey = a.day.toLowerCase();
      return dayMap[dayKey] === dayIndex && a.moduleIndex === moduleIndex;
    });
  }

  async function generatePDF() {
    console.log("Generando PDF...");
    const selectedClassroom = classroomList.find(
      (classroom) => classroom.id === selectedClassroomId,
    );

    if (!selectedClassroom) return;

    try {
    /*
      const orientation = confirm("¿Usar orientación horizontal?") 
        ? "landscape" 
        : "portrait";
      */

      const orientation = "portrait";

      // 1. Crear un elemento off-screen para renderizar
      const printContainer = document.createElement('div');
      printContainer.style.position = 'absolute';
      printContainer.style.left = '-9999px';
      printContainer.style.top = '0';
      printContainer.style.width = '800px';
      printContainer.style.backgroundColor = 'white';
      
      // 2. Clonar solo lo necesario
      const horarioClone = document.querySelector('.grid-container')!.cloneNode(true) as HTMLElement;
      horarioClone.style.width = '100%';
      horarioClone.style.margin = '0';
      horarioClone.style.padding = '0';
      
      printContainer.appendChild(horarioClone);
      document.body.appendChild(printContainer);

      // 3. Configuración optimizada de html2canvas
      const canvas = await html2canvas(printContainer, {
        scale: 1,
        logging: false,
        useCORS: true,
        removeContainer: true,
        ignoreElements: (element) => element.classList.contains("no-print")
      });

      const doc = new jsPDF({
        orientation,
        unit: 'mm',
        format: 'a4',
      });

      // Encabezado
      doc.setFontSize(16);
      doc.text(schoolName || "Escuela", 105, 15, { align: "center" });
      
      doc.setFontSize(14);
      doc.text(
        `Aula: ${getClassroomName(selectedClassroom.id!)}${selectedClassroom.building_type ? ` (${selectedClassroom.building_type})` : ''}`,
        105,
        25,
        { align: "center" }
      );

      if (selectedClassroom.capacity) {
        doc.setFontSize(12);
        doc.text(
          `Capacidad: ${selectedClassroom.capacity} alumnos`,
          105,
          32,
          { align: "center" }
        );
      }

      // 4. Optimizar imagen para PDF
      const imgData = canvas.toDataURL('image/jpeg', 0.85);
      const imgProps = doc.getImageProperties(imgData);
      const pdfWidth = doc.internal.pageSize.getWidth() - 20;
      const pdfHeight = (imgProps.height * pdfWidth) / imgProps.width;

      doc.addImage(
        imgData,
        'JPEG',
        10,
        selectedClassroom.capacity ? 40 : 35, // Ajustar posición según contenido
        pdfWidth,
        pdfHeight
      );

      // 5. Guardar en un paso separado
      setTimeout(async () => {
        const pdfOutput = doc.output("arraybuffer");
        const path = await invoke<string | null>("export_pdf_file");
        
        if (path) {
          await writeBinaryFile({
            contents: new Uint8Array(pdfOutput),
            path,
          });
          addNotification({
            message: "Horario guardado satisfactoriamente",
            type: "info",
            timeout: 1000,
          });
        }
        
        // Limpiar
        document.body.removeChild(printContainer);
      }, 100);

    } catch (error) {
      console.error("Error al generar PDF:", error);
      addNotification({
        message: "Error al generar el PDF",
        type: "error",
        timeout: 2000,
      });
    }
  }
</script>

<div class="select-container">
  <label for="classroom-select">Selecciona un aula</label>
  <select id="classroom-select" class="custom-select" bind:value={selectedClassroomId}>
    <option disabled selected value={null}>Selecciona</option>
    {#each classroomList as classroom}
      <option value={classroom.id}>
        {classroom.building_id || ''}{classroom.building_number}
        {classroom.building_type ? ` (${classroom.building_type})` : ''}
        {classroom.capacity ? ` - ${classroom.capacity} alumnos` : ''}
      </option>
    {/each}
  </select>
  <button class="custom-select" on:click={generatePDF}>Descargar PDF</button>
</div>

<div class="grid-container">
  <div class="time"></div>
  {#each days as day}
    <div class="header">{day}</div>
  {/each}
  
  {#each horas as hora, index}
    <div class="time">{hora}</div>
    {#each days as day, dayIndex}
      <div class="cell">
        {#key `${selectedClassroomId}-${dayIndex + 1}-${index}`}
          {#if selectedClassroomId}
            {@const assignment = findAssignment(dayIndex + 1, index)}
            {#if assignment}
              <div
                class="time-block"
                style="color: black;"
              >
                <div>{assignment.subject_name}</div>
                <div>Grupo: {getGroup(assignment.groupId)}</div>
                <div>Profesor: {getTeacher(assignment.teacherId)}</div>
              </div>
            {/if}
          {/if}
        {/key}
      </div>
    {/each}
  {/each}
</div>
