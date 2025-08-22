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
  import { readBinaryFile, writeBinaryFile } from "@tauri-apps/api/fs";
  import { schoolStore } from "$lib/modules/config/configStore";

  import "$styles/schedule_preview.scss";
  import html2canvas from "html2canvas";
  import { addNotification } from "$lib/stores/notificationsStore";
  import { subjectsWithTeachers } from "$lib/modules/entities/subjectsStore";

  let selectedTeacherId: number | null = null;
  let schoolName = "";
  let schoolLogoPath: string;

  // Nuevas variables para configuración de impresión
  let printOrientation: "portrait" | "landscape" = "portrait";
  let schedulesPerPage: number = 1;
  let isGeneratingAll: boolean = false;

  // Función para generar los slots de tiempo
  function generateTimeSlots(
    config: any,
  ): { time: string; isBreak: boolean; moduleIndex: number }[] {
    const slots: { time: string; isBreak: boolean; moduleIndex: number }[] = [];
    let currentTime = new Date();
    currentTime.setHours(7, 0, 0, 0);

    const durationMs =
      config.durationUnit === "hours"
        ? config.moduleDuration * 60 * 60 * 1000
        : config.moduleDuration * 60 * 1000;

    const breakDurationMs = config.breakDuration * 60 * 1000;

    let moduleIndex = 0; // Contador real de módulos

    for (let i = 0; i < config.modulesPerDay; i++) {
      // Agregar descanso antes del módulo si está configurado
      if (config.hasBreaks && config.breakPositions.includes(i)) {
        const breakStartTime = new Date(currentTime);
        currentTime = new Date(currentTime.getTime() + breakDurationMs);

        const breakStartStr = breakStartTime.toLocaleTimeString("es-MX", {
          hour: "2-digit",
          minute: "2-digit",
          hour12: false,
        });
        const breakEndStr = currentTime.toLocaleTimeString("es-MX", {
          hour: "2-digit",
          minute: "2-digit",
          hour12: false,
        });

        slots.push({
          time: `${breakStartStr} - ${breakEndStr}`,
          isBreak: true,
          moduleIndex: -1, // -1 para indicar que es un descanso
        });
      }

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

      slots.push({
        time: `${startStr} - ${endStr}`,
        isBreak: false,
        moduleIndex: moduleIndex, // Usar el índice real del módulo
      });

      moduleIndex++; // Incrementar solo para módulos reales
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

  /*
  function findAssignment(dayIndex: number, moduleIndex: number) {
    return teacherSchedule.find((a) => {
      const dayKey = a.day.toLowerCase();
      return dayMap[dayKey] === dayIndex && a.moduleIndex === moduleIndex;
    });
  }
  */
  function findAssignment(dayIndex: number, moduleIndex: number) {
    console.log(
      `Buscando asignación para día ${dayIndex}, módulo ${moduleIndex}`,
    );
    const assignment = teacherSchedule.find((a) => {
      const dayKey = a.day.toLowerCase();
      const matches =
        dayMap[dayKey] === dayIndex && a.moduleIndex === moduleIndex;
      if (matches) {
        console.log(`Encontrada asignación:`, a);
      }
      return matches;
    });

    if (!assignment && moduleIndex === 7) {
      // Debug específico para el módulo 8 (índice 7)
      console.log(
        `No se encontró asignación para módulo 8 (índice 7), día ${dayIndex}`,
      );
      console.log("Asignaciones del profesor:", teacherSchedule);
    }

    return assignment;
  }

  // Función corregida para generar el horario de un profesor como imagen
  async function generateTeacherScheduleImage(
    teacherId: number,
  ): Promise<{ imageData: string; teacherName: string }> {
    const teacher = teachersList.find((t) => t.id === teacherId);
    if (!teacher) throw new Error("Profesor no encontrado");

    // Crear un contenedor temporal con la estructura completa
    const printContainer = document.createElement("div");
    printContainer.style.position = "absolute";
    printContainer.style.left = "-9999px";
    printContainer.style.top = "0";
    printContainer.style.width =
      printOrientation === "portrait" ? "800px" : "1200px";
    printContainer.style.backgroundColor = "white";

    // Crear una copia completa del componente con los datos del profesor seleccionado
    const tempDiv = document.createElement("div");
    tempDiv.innerHTML = `
      <div class="grid-container">
          <div class="time"></div>
          ${days.map((day) => `<div class="header">${day}</div>`).join("")}
          
          ${horas
            .map((slot, slotIndex) => {
              if (slot.isBreak) {
                return `
                    <div class="time">${slot.time}</div>
                    <div class="cell break-cell" style="grid-column: 2 / ${days.length + 2};">
                      <div class="time-block break" style="color: black; font-weight: bold;">
                        RECESO
                      </div>
                    </div>
                  `;
              } else {
                return `
                    <div class="time">${slot.time}</div>
                    ${days
                      .map((day, colIndex) => {
                        const assignment = Array.from(
                          assignmentsMap.values(),
                        ).find((a) => {
                          const dayKey = a.day.toLowerCase();
                          return (
                            a.teacherId === teacherId &&
                            dayMap[dayKey] === colIndex + 1 &&
                            a.moduleIndex === slot.moduleIndex // AQUÍ ESTÁ EL CAMBIO CLAVE
                          );
                        });

                        return `
                            <div class="cell">
                                ${
                                  assignment
                                    ? `
                                    <div class="time-block" style="color: black;">
                                        <div>${assignment.subject_name}</div>
                                        <div>${getGroup(assignment.groupId)}</div>
                                    </div>
                                `
                                    : ""
                                }
                            </div>
                        `;
                      })
                      .join("")}
                  `;
              }
            })
            .join("")}
      </div>
  `;

    printContainer.appendChild(tempDiv);
    document.body.appendChild(printContainer);

    const canvas = await html2canvas(printContainer, {
      scale: 1.5,
      logging: false,
      useCORS: true,
    });

    document.body.removeChild(printContainer);

    return {
      imageData: canvas.toDataURL("image/png"),
      teacherName: `${teacher.name} ${teacher.father_lastname}`,
    };
  }

  // Helper para agregar la imagen al pdf
  async function loadImageAsBase64(path: string): Promise<string> {
    const imageBytes = await readBinaryFile(path);
    const base64String = btoa(
      imageBytes.reduce((data, byte) => data + String.fromCharCode(byte), ""),
    );
    return `data:image/jpeg;base64,${base64String}`;
  }

  // Función para generar PDF de un solo profesor
  async function generatePDF() {
    if (!selectedTeacherId) return;

    try {
      isGeneratingAll = false;
      const { imageData, teacherName } =
        await generateTeacherScheduleImage(selectedTeacherId);

      const doc = new jsPDF({
        orientation: printOrientation,
        unit: "mm",
        format: "a4",
      });

      // Encabezado

      doc.setFontSize(16);
      doc.text(
        schoolName ? `Escuela: ${schoolName}` : "Horario escolar",
        105,
        15,
        { align: "center" },
      );
      doc.setFontSize(14);
      doc.text(`Profesor(a): ${teacherName}`, 105, 25, { align: "center" });

      const imgProps = doc.getImageProperties(imageData);
      const pageWidth = doc.internal.pageSize.getWidth() - 20;
      const pageHeight = doc.internal.pageSize.getHeight() - 40;

      let imgWidth = imgProps.width * 0.264583; // Convertir px a mm (96dpi)
      let imgHeight = imgProps.height * 0.264583;

      // Ajustar tamaño para que quepa en la página
      const ratio = Math.min(pageWidth / imgWidth, pageHeight / imgHeight);
      imgWidth *= ratio;
      imgHeight *= ratio;

      doc.addImage(
        imageData,
        "PNG",
        10,
        30,
        imgWidth,
        imgHeight,
        undefined,
        "FAST",
      );

      // Convertir la imagen a base64
      const base64Logo = await loadImageAsBase64(schoolLogoPath);
      doc.addImage(base64Logo, 'JPEG', 18, 5, 15, 15);

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
    } catch (error) {
      console.error("Error al generar PDF:", error);
      addNotification({
        message: "Error al generar el PDF",
        type: "error",
        timeout: 2000,
      });
    }
  }

  // Función para generar PDF de todos los profesores
  async function generateAllPDFs() {
    if (teachersList.length === 0) return;

    try {
      isGeneratingAll = true;
      const doc = new jsPDF({
        orientation: printOrientation,
        unit: "mm",
        format: "a4",
      });

      // Configuración de diseño basado en la orientación y cantidad por página
      const maxSchedules =
        printOrientation === "portrait"
          ? Math.min(schedulesPerPage, 2)
          : Math.min(schedulesPerPage, 4);

      const pageWidth = doc.internal.pageSize.getWidth();
      const pageHeight = doc.internal.pageSize.getHeight();

      const margin = 10;
      const availableWidth = pageWidth - 2 * margin;
      const availableHeight = pageHeight - 2 * margin;

      // Calcular dimensiones de cada horario
      const schedulesPerRow =
        printOrientation === "portrait" ? 1 : Math.min(2, maxSchedules);
      const schedulesPerColumn = Math.ceil(maxSchedules / schedulesPerRow);

      const scheduleWidth = availableWidth / schedulesPerRow;
      const scheduleHeight = availableHeight / schedulesPerColumn;

      let currentPage = 0;
      let positionInPage = 0;

      for (let i = 0; i < teachersList.length; i++) {
        const teacher = teachersList[i];

        if (positionInPage >= maxSchedules) {
          doc.addPage();
          currentPage++;
          positionInPage = 0;
        }

        const { imageData, teacherName } = await generateTeacherScheduleImage(
          teacher.id ? teacher.id : 0,
        );

        const imgProps = doc.getImageProperties(imageData);
        let imgWidth = imgProps.width * 0.264583; // px to mm
        let imgHeight = imgProps.height * 0.264583;

        // Ajustar tamaño para el espacio asignado
        const ratio = Math.min(
          scheduleWidth / imgWidth,
          (scheduleHeight / imgHeight) * 0.9,
        );
        imgWidth *= ratio;
        imgHeight *= ratio;

        // Calcular posición
        const row = Math.floor(positionInPage / schedulesPerRow);
        const col = positionInPage % schedulesPerRow;

        const x = margin + col * scheduleWidth;
        const y = margin + row * scheduleHeight;

        // Agregar título del profesor
        doc.setFontSize(10);
        doc.text(teacherName, x + 5, y + 5);

        // Agregar imagen del horario
        doc.addImage(
          imageData,
          "PNG",
          x,
          y + 8, // Dejar espacio para el título
          imgWidth,
          imgHeight,
          undefined,
          "FAST",
        );

        positionInPage++;
      }

      // Guardar el PDF
      const pdfOutput = doc.output("arraybuffer");
      const path = await invoke<string | null>("export_pdf_file");

      if (path) {
        await writeBinaryFile({
          contents: new Uint8Array(pdfOutput),
          path,
        });
        addNotification({
          message: `Horarios de ${teachersList.length} profesores guardados`,
          type: "info",
          timeout: 2000,
        });
      }
    } catch (error) {
      console.error("Error al generar PDFs:", error);
      addNotification({
        message: "Error al generar los PDFs",
        type: "error",
        timeout: 2000,
      });
    } finally {
      isGeneratingAll = false;
    }
  }

  // Funcionalidad de la orientacion
  let isOpen = false;
  const orientationOptions = [
    { value: "portrait", label: "Vertical", icon: "/icons/portrait.svg" },
    { value: "landscape", label: "Horizontal", icon: "/icons/landscape.svg" },
  ];

  function selectOption(option: any): void {
    printOrientation = option.value;
    isOpen = false;
  }

  // Actualizar opciones de horarios por página según la orientación
  $: {
    if (printOrientation === "portrait") {
      schedulesPerPage = Math.min(schedulesPerPage, 2);
    } else {
      schedulesPerPage = Math.min(schedulesPerPage, 4);
    }
  }
</script>

<div class="print-controls">
  <div class="select-container">
    <select
      id="teacher-select"
      class="custom-select"
      bind:value={selectedTeacherId}
    >
      <option disabled selected value={null}>Selecciona un profesor</option>
      {#each teachersList as teacher}
        <option value={teacher.id}
          >{teacher.name} {teacher.father_lastname}</option
        >
      {/each}
    </select>
    <button
      class="custom-select"
      on:click={generatePDF}
      disabled={isGeneratingAll}
    >
      {isGeneratingAll
        ? "Generando..."
        : `Descargar horario ${selectedTeacherId ? `${teachersList[selectedTeacherId - 1].name} ${teachersList[selectedTeacherId - 1].father_lastname}` : ""}`}
    </button>

    <button
      class="custom-select"
      on:click={generateAllPDFs}
      disabled={isGeneratingAll}
    >
      {isGeneratingAll ? "Generando todos..." : "Imprimir todos los profesores"}
    </button>
  </div>

  <div class="print-options">
    <h3>Opciones de impresión</h3>

    <div class="option-group">
      <!-- svelte-ignore a11y-label-has-associated-control -->
      <label>Orientación:</label>
      <div class="dropdown">
        <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
        <div class="dropdown-toggle" on:click={() => (isOpen = !isOpen)}>
          <img
            src={orientationOptions.find((o) => o.value === printOrientation)
              ?.icon}
            alt=""
            width="24"
          />
          <span>
            {orientationOptions.find((o) => o.value === printOrientation)
              ?.label}
          </span>
        </div>
        {#if isOpen}
          <div class="dropdown-menu">
            {#each orientationOptions as option}
              <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
              <div class="dropdown-item" on:click={() => selectOption(option)}>
                <img src={option.icon} alt="" />
                <span>{option.label}</span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <div class="option-group">
      <!-- svelte-ignore a11y-label-has-associated-control -->
      <label>Horarios por página:</label>
      {#if printOrientation === "portrait"}
        <div class="select-wrapper">
          <select bind:value={schedulesPerPage}>
            <option value={1}>1</option>
          </select>
        </div>
      {:else}
        <div class="select-wrapper">
          <select bind:value={schedulesPerPage}>
            <option value={1}>1</option>
            <option value={2}>2</option>
            <option value={4}>4</option>
          </select>
        </div>
      {/if}
    </div>
  </div>
</div>

<div class="grid-container">
  <div class="time"></div>
  {#each days as day}
    <div class="header">{day}</div>
  {/each}

  {#each horas as slot, slotIndex}
    <div class="time">{slot.time}</div>
    {#if slot.isBreak}
      <!-- Celda unificada para receso que abarca todas las columnas -->
      <div class="cell" style="grid-column: 2 / {days.length + 2};">
        <div
          class="time-block"
          style="color: black; background-color: #fff; height: 90%; font-weight: bold;"
        >
          RECESO
        </div>
      </div>
    {:else}
      {#each days as day, colIndex}
        <div class="cell">
          {#key `${selectedTeacherId}-${colIndex + 1}-${slot.moduleIndex}`}
            {#if selectedTeacherId}
              {@const assignment = findAssignment(
                colIndex + 1,
                slot.moduleIndex,
              )}
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
    {/if}
  {/each}
</div>
