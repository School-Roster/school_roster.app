<script lang="ts">
  import {
    assignmentsStore,
    loadAssignments,
  } from "$lib/modules/entities/assignments";
  import { teachers, loadTeachers } from "$lib/modules/entities/teachersStore";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
  import { configStore, loadConfig } from "$lib/modules/config/configStore";
  import { onMount } from "svelte";
  import jsPDF from "jspdf";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writeBinaryFile } from "@tauri-apps/api/fs";

  import "$styles/schedule_preview.scss";

  let selectedTeacherId: number | null = null;

  // Helper function to calculate time slots based on config
  function generateTimeSlots(config: any): string[] {
    const slots: string[] = [];
    let currentTime = new Date();
    currentTime.setHours(7, 0, 0, 0); // Start at 7:00 AM by default
    
    const durationMs = config.durationUnit === 'hours' 
      ? config.moduleDuration * 60 * 60 * 1000 
      : config.moduleDuration * 60 * 1000;
    
    const breakDurationMs = config.breakDuration * 60 * 1000;
    
    for (let i = 0; i < config.modulesPerDay; i++) {
      const startTime = new Date(currentTime);
      currentTime = new Date(currentTime.getTime() + durationMs);
      
      const startStr = startTime.toLocaleTimeString('es-MX', { 
        hour: '2-digit', 
        minute: '2-digit',
        hour12: false 
      });
      const endStr = currentTime.toLocaleTimeString('es-MX', { 
        hour: '2-digit', 
        minute: '2-digit',
        hour12: false 
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

  function generatePDF() {
    console.log("Trying to generate PDF...");
    const horarioElement = document.querySelector(
      ".grid-container",
    ) as HTMLElement;

    const selectedTeacher = teachersList.find(
      (teacher) => teacher.id === selectedTeacherId,
    );

    if (horarioElement && selectedTeacher) {
      const doc = new jsPDF({
        orientation: "landscape",
        unit: "mm",
        format: "a4",
      });

      doc.text(
        `Profesor(a): ${selectedTeacher.name} ${selectedTeacher.father_lastname}`,
        100,
        10,
      );

      // Wait for paint
      setTimeout(() => {
        doc.html(horarioElement, {
          callback: async (doc) => {
            const pdfOutput = doc.output("arraybuffer");
            const path = await invoke<string | null>("export_pdf_file");
            if (path) {
              await writeBinaryFile({
                contents: new Uint8Array(pdfOutput),
                path,
              });
              console.log("PDF guardado en:", path);
            } else {
              console.log("Guardado cancelado.");
            }
          },
          x: 0,
          y: 15,
          html2canvas: {
            scale: 0.15,
            useCORS: true,
          },
        });
      }, 100);
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
              <div
                class="time-block"
                style="background-color: {assignment.color}; color: white;"
              >
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
