<script lang="ts">
  import {
    assignmentsStore,
    loadAssignments,
  } from "$lib/modules/entities/assignments";
  import { subjects, loadSubjects } from "$lib/modules/entities/subjectsStore";
  import { teachers, loadTeachers } from "$lib/modules/entities/teachersStore";
  import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
  import { onMount } from "svelte";
  import jsPDF from "jspdf";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writeBinaryFile } from "@tauri-apps/api/fs";

  import "$styles/schedule_preview.scss";

  let selectedSubjectId: string | null = null;
  $: parsedSubjectId =
    selectedSubjectId !== null ? Number(selectedSubjectId) : null;

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
    loadSubjects();
    loadTeachers();
    loadGroups();
  });

  $: assignmentsMap = $assignmentsStore;
  $: subjectsList = $subjects;
  $: teachersList = $teachers;
  $: groupsList = $groups;

  $: subjectSchedule = parsedSubjectId
    ? Array.from(assignmentsMap.values()).filter(
        (a) => a.subjectId === parsedSubjectId,
      )
    : [];

  function getGroup(id: number): string {
    const g = groupsList.find((x) => x.id === id);
    return g ? `${g.grade}${g.group}` : "Grupo no encontrado";
  }

  function getTeacher(id: number): string {
    const t = teachersList.find((x) => x.id === id);
    return t ? `${t.name} ${t.father_lastname}` : "Maestro no encontrado";
  }

  function findAssignment(day: number, moduleIndex: number) {
    return subjectSchedule.find((a) => {
      const dk = a.day.toLowerCase() as keyof typeof dayMap;
      return dayMap[dk] === day && a.moduleIndex === moduleIndex;
    });
  }

  function generatePDF() {
    const el = document.querySelector(".grid-container") as HTMLElement;
    const subj = subjectsList.find((s) => s.id === parsedSubjectId);
    if (!el || !subj) return;

    const doc = new jsPDF({
      orientation: "landscape",
      unit: "mm",
      format: "a4",
    });
    doc.text(`Materia: ${subj.name}`, 10, 5);
    /*
        doc.html(el, {
            callback: d => d.save('horario-materia.pdf'),
            x: 10, y: 10,
            html2canvas: { scale: 0.2 }
        });
        */
    setTimeout(() => {
      doc.html(el, {
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
</script>

<div class="select-container">
  <label for="subject-select">Selecciona una materia</label>
  <select
    id="subject-select"
    class="custom-select"
    bind:value={selectedSubjectId}
  >
    <option disabled selected value={null}>Selecciona</option>
    {#each subjectsList as subject}
      <option value={subject.id}>{subject.name}</option>
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
        {#key `${parsedSubjectId}-${colIndex}-${index}`}
          {#if parsedSubjectId}
            {@const assignment = findAssignment(colIndex, index)}
            {#if assignment}
              <div
                class="time-block"
                style="background-color: {assignment.color}; color: white;"
              >
                <div>{getGroup(assignment.groupId)}</div>
                <div>{getTeacher(assignment.teacherId)}</div>
              </div>
            {/if}
          {/if}
        {/key}
      </div>
    {/each}
  {/each}
</div>
