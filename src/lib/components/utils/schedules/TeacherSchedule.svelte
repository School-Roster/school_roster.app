<script lang="ts">
  import {
    assignmentsStore,
    loadAssignments,
  } from "$lib/modules/entities/assignments";
  import { teachers, loadTeachers } from "$lib/modules/entities/teachersStore";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
  import {
    configStore,
    loadConfig,
    loadSchoolInfo,
  } from "$lib/modules/config/configStore";
  import { onMount } from "svelte";
  import jsPDF from "jspdf";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writeBinaryFile } from "@tauri-apps/api/fs";
  import { schoolStore } from "$lib/modules/config/configStore";

  import "$styles/schedule_preview.scss";
  import html2canvas from "html2canvas";
  import { addNotification } from "$lib/stores/notificationsStore";

  let selectedTeacherId: number | null = null;
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
    await loadConfig(); // Load config first
    await loadAssignments();
    await loadTeachers();
    await loadGroups();
    await loadSchoolInfo();
    schoolStore.subscribe((info) => {
      schoolName = info.name;
      schoolLogoPath = info.logo_path;
    });
  });

  // Reactive values from config
  $: days = $configStore.days;
  $: dayMap = createDayMap(days);
  $: horas = generateTimeSlots($configStore);
  $: assignmentsMap = $assignmentsStore;
  $: teachersList = $teachers;
  $: groupsList = $groups;

  function getGroup(id: number): string {
    const group = groupsList.find((t) => t.id === id);
    return group ? `${group.grade}${group.group}` : "Grupo no encontrado";
  }

  $: teacherSchedule = selectedTeacherId
    ? Array.from(assignmentsMap.values()).filter(
        (a) => a.teacherId === selectedTeacherId,
      )
    : [];

  function findAssignment(dayIndex: number, moduleIndex: number) {
    return teacherSchedule.find((a) => {
      const dayKey = a.day.toLowerCase();
      return dayMap[dayKey] === dayIndex && a.moduleIndex === moduleIndex;
    });
  }

  async function generatePDF() {
    console.log("Generando PDF...");
    const selectedTeacher = teachersList.find(
      (teacher) => teacher.id === selectedTeacherId,
    );

    if (!selectedTeacher) return;

    try {
      const orientation = "portrait";

      // 1. Crear un elemento off-screen para renderizar
      const printContainer = document.createElement("div");
      printContainer.style.position = "absolute";
      printContainer.style.left = "-9999px";
      printContainer.style.top = "0";
      printContainer.style.width = "800px";
      printContainer.style.backgroundColor = "white";

      // 2. Clonar solo lo necesario
      const horarioClone = document
        .querySelector(".grid-container")!
        .cloneNode(true) as HTMLElement;
      horarioClone.style.width = "100%";
      horarioClone.style.margin = "0";
      horarioClone.style.padding = "0";

      printContainer.appendChild(horarioClone);
      document.body.appendChild(printContainer);

      // 3. Configuración optimizada de html2canvas
      const canvas = await html2canvas(printContainer, {
        scale: 1, // Reducir scale mejora rendimiento
        logging: false,
        useCORS: true,
        removeContainer: true, // Limpiar automáticamente
        ignoreElements: (element) => {
          // Ignorar elementos innecesarios
          return element.classList.contains("no-print");
        },
      });

      const doc = new jsPDF({
        orientation,
        unit: "mm",
        format: "a4",
      });

      // Encabezado
      doc.setFontSize(16);
      doc.text(schoolName || "Horario escolar", 105, 15, { align: "center" });

      doc.setFontSize(14);
      doc.text(
        `Profesor(a): ${selectedTeacher.name} ${selectedTeacher.father_lastname}`,
        105,
        25,
        { align: "center" },
      );

      // 4. Optimizar imagen para PDF
      const imgData = canvas.toDataURL("image/jpeg", 0.85); // JPEG con compresión
      const imgProps = doc.getImageProperties(imgData);
      const pdfWidth = doc.internal.pageSize.getWidth() - 20;
      const pdfHeight = (imgProps.height * pdfWidth) / imgProps.width;

      doc.addImage(
        imgData,
        "JPEG",
        10,
        35, // Más espacio para el encabezado
        pdfWidth,
        pdfHeight,
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
      alert("Error al generar el PDF");
    } finally {
      // Cerrar loader
      // if (typeof loading === "function") loading();
    }
  }
</script>

<div class="select-container">
  <label for="teacher-select">Selecciona un profesor</label>
  <select
    id="teacher-select"
    class="custom-select"
    bind:value={selectedTeacherId}
  >
    <option disabled selected value={null}>Selecciona</option>
    {#each teachersList as teacher}
      <option value={teacher.id}
        >{teacher.name} {teacher.father_lastname}</option
      >
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
    {#each days as day, colIndex}
      <div class="cell">
        {#key `${selectedTeacherId}-${colIndex + 1}-${index}`}
          {#if selectedTeacherId}
            {@const assignment = findAssignment(colIndex + 1, index)}
            {#if assignment}
              <div class="time-block" style="color: black;">
                <div>{assignment.subject_name}</div>
                <div>{getGroup(assignment.groupId)}</div>
              </div>
            {/if}
          {/if}
        {/key}
      </div>
    {/each}
  {/each}
</div>
