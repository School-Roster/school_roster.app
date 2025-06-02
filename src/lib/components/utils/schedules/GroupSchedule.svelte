<script lang="ts">
  import {
    assignmentsStore,
    loadAssignments,
  } from "$lib/modules/entities/assignments";
  import { teachers, loadTeachers } from "$lib/modules/entities/teachersStore";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
  import { onMount } from "svelte";
  import jsPDF from "jspdf";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writeBinaryFile } from "@tauri-apps/api/fs";
  import { configStore } from '$lib/modules/config/configStore';

  import "$styles/schedule_preview.scss";

  let selectedGroupId: number | null = null;

  let horas: string[] = [];
  
  $: {
    // Generate time slots based on config
    const { modulesPerDay, moduleDuration, durationUnit, hasBreaks, breakDuration, breakPositions } = $configStore;
    horas = [];
    let currentTime = new Date();
    currentTime.setHours(7, 0, 0); // Start at 7:00 AM

    for (let i = 0; i < modulesPerDay; i++) {
      const startTime = new Date(currentTime);
      currentTime.setMinutes(currentTime.getMinutes() + moduleDuration);
      const endTime = new Date(currentTime);
      
      horas.push(`${startTime.getHours()}:${startTime.getMinutes().toString().padStart(2, '0')} - ${endTime.getHours()}:${endTime.getMinutes().toString().padStart(2, '0')}`);
      
      // Add break if configured
      if (hasBreaks && breakPositions.includes(i)) {
        const breakStart = new Date(currentTime);
        currentTime.setMinutes(currentTime.getMinutes() + breakDuration);
        const breakEnd = new Date(currentTime);
        horas.push(`Receso: ${breakStart.getHours()}:${breakStart.getMinutes().toString().padStart(2, '0')} - ${breakEnd.getHours()}:${breakEnd.getMinutes().toString().padStart(2, '0')}`);
      }
    }
  }

  const dayMap: Record<string, number> = {
    lunes: 1,
    martes: 2,
    miercoles: 3,
    jueves: 4,
    viernes: 5,
  };

  onMount(() => {
    loadAssignments();
    loadTeachers();
    loadGroups();
  });

  $: assignmentsMap = $assignmentsStore;
  $: teachersList = $teachers;
  $: groupsList = $groups;

  function getTeacher(id: number): string {
    const teacher = teachersList.find((t) => t.id === id);
    return teacher
      ? `${teacher.name} ${teacher.father_lastname}`
      : "Maestro no encontrado";
  }

  $: groupSchedule = selectedGroupId
    ? Array.from(assignmentsMap.values()).filter(
        (a) => a.groupId === selectedGroupId,
      )
    : [];

  function findAssignment(day: number, moduleIndex: number) {
    return groupSchedule.find((a) => {
      const dayKey = a.day.toLowerCase() as keyof typeof dayMap;
      return dayMap[dayKey] === day && a.moduleIndex === moduleIndex;
    });
  }

  function generatePDF() {
    const horarioElement = document.querySelector(
      ".grid-container",
    ) as HTMLElement;

    const selectedGroup = groupsList.find(
      (group) => group.id === selectedGroupId,
    );

    if (horarioElement && selectedGroup) {
      const doc = new jsPDF({
        orientation: "landscape",
        unit: "mm",
        format: "a4",
      });

      doc.text(`Grupo: ${selectedGroup.grade} ${selectedGroup.group}`, 100, 10);

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
  <label for="group-select">Selecciona un grupo</label>
  <select id="group-select" class="custom-select" bind:value={selectedGroupId}>
    <option disabled selected value={null}>Selecciona</option>
    {#each groupsList as group}
      <option value={group.id}>{group.grade} {group.group}</option>
    {/each}
  </select>
  <button class="custom-select" on:click={generatePDF}>Descargar PDF</button>
</div>

<div class="grid-container">
  <div class="time"></div>
  {#each $configStore.days as day}
    <div class="header">{day}</div>
  {/each}
  
  {#each horas as hora, index}
    <div class="time">{hora}</div>
    {#each $configStore.days as day, dayIndex}
      <div class="cell">
        {#key `${selectedGroupId}-${dayIndex}-${index}`}
          {#if selectedGroupId}
            {@const assignment = findAssignment(dayIndex + 1, index)}
            {#if assignment}
              <div
                class="time-block"
                style="background-color: {assignment.color}; color: white;"
              >
                <div>{assignment.subject_name}</div>
                <div>Profesor: {getTeacher(assignment.teacherId)}</div>
              </div>
            {/if}
          {/if}
        {/key}
      </div>
    {/each}
  {/each}
</div>
