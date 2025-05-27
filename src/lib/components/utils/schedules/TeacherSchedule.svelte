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

  import "$styles/schedule_preview.scss";

  let selectedTeacherId: number | null = null;

  // TODO: Cambiar a horas por modulo
  let horas = [
    "7:00 - 7:50",
    "7:50 - 8:40",
    "8:40 - 9:30",
    "9:30 - 10:20",
    "10:20 - 11:10",
    "11:10 - 12:00",
    "12:00 - 12:50",
    "12:50 - 1:40",
    "1:40 - 2:30",
  ];

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

  function getGroup(id: number): string {
    const group = groupsList.find((t) => t.id === id);
    return group ? `${group.grade}${group.group}` : "Grupo no encontrado";
  }

  $: teacherSchedule = selectedTeacherId
    ? Array.from(assignmentsMap.values()).filter(
        (a) => a.teacherId === selectedTeacherId,
      )
    : [];

  function findAssignment(day: number, moduleIndex: number) {
    return teacherSchedule.find((a) => {
      const dayKey = a.day.toLowerCase() as keyof typeof dayMap;
      return dayMap[dayKey] === day && a.moduleIndex === moduleIndex;
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
  <div class="header">Lunes</div>
  <div class="header">Martes</div>
  <div class="header">Miércoles</div>
  <div class="header">Jueves</div>
  <div class="header">Viernes</div>

  {#each horas as hora, index}
    <div class="time">{hora}</div>
    {#each [1, 2, 3, 4, 5] as colIndex}
      <div class="cell">
        {#key `${selectedTeacherId}-${colIndex}-${index}`}
          {#if selectedTeacherId}
            {@const assignment = findAssignment(colIndex, index)}
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
