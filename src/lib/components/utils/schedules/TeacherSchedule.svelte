<script lang="ts">
    import { assignmentsStore, loadAssignments } from "$lib/modules/entities/assignments";
    import { teachers, loadTeachers } from "$lib/modules/entities/teachersStore";
    import { groups, loadGroups } from "$lib/modules/entities/groupsStore";
    import { onMount } from 'svelte';
    import jsPDF from "jspdf";

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
        "1:40 - 2:30"
    ];

    const dayMap: Record<string, number> = {
        lunes: 1,
        martes: 2,
        miercoles: 3,
        jueves: 4,
        viernes: 5
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
        const group = groupsList.find(t => t.id === id);
        return group ? `${group.grade}${group.group}` : "Grupo no encontrado";
    }

    $: teacherSchedule = selectedTeacherId ? Array.from(assignmentsMap.values()).filter(a => a.teacherId === selectedTeacherId) : [];

    function findAssignment(day: number, moduleIndex: number) {
        return teacherSchedule.find(a => {
            const dayKey = a.day.toLowerCase() as keyof typeof dayMap;
            return (
                dayMap[dayKey] === day &&
                a.moduleIndex === moduleIndex
            );
        });
    }

    function generatePDF() {
        const horarioElement = document.querySelector('.grid-container') as HTMLElement;

        const selectedTeacher = teachersList.find(teacher => teacher.id === selectedTeacherId);

        if (horarioElement && selectedTeacher) {
            const doc = new jsPDF({
                orientation: 'landscape', 
                unit: 'mm',
                format: 'a4'
            });

            doc.text(`Profesor: ${selectedTeacher.name} ${selectedTeacher.father_lastname}`, 10, 5);

            doc.html(horarioElement, {
                callback: (doc) => {
                    doc.save('horario.pdf');
                },
                x: 10,
                y: 10,
                html2canvas: {
                    scale: 0.2, 
                }
            });
        }
    }

</script>

<div class="select-container">
    <label for="teacher-select">Selecciona un profesor</label>
    <select id="teacher-select" class="custom-select" bind:value={selectedTeacherId}>
        <option disabled selected value={null}>Selecciona</option>
        {#each teachersList as teacher}
            <option value={teacher.id}>{teacher.name} {teacher.father_lastname}</option>
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
                            <div class="time-block"
                                style="background-color: {assignment.color}; color: white;">
                                <div>{assignment.shorten}</div>
                                <div>{getGroup(assignment.groupId)}</div>
                            </div>
                        {/if}
                    {/if}
                {/key}
            </div>
        {/each}
    {/each}
</div>

<style>
    .select-container {
      margin: 1rem 0;
      color: #aee6f9; 
      font-family: 'Segoe UI', sans-serif;
    }
  
    label {
      margin-bottom: 0.5rem;
      margin-right: 1rem;
      margin-left: 1rem;
      font-size: 1.1rem;
      font-weight: 600;
    }
  
    .custom-select {
      padding: 1rem;
      background-color: #4a75a7;
      border: none;
      border-radius: 0.5rem;
      color: #ffffff;
      font-size: 1rem;
      appearance: none;
      outline: none;
      cursor: pointer;
      transition: background-color 0.3s ease;
      text-align: center;
      margin-left: 0.5rem;
    }
  
    .custom-select:hover {
      background-color: #5d86b8;
    }
  
    option {
      color: #000;
    }

    .grid-container {
        display: grid;
        grid-template-columns: 150px repeat(5, 1fr); 
        grid-template-rows: repeat(9, 50px); 
        font-family: Arial, sans-serif;
    }

    .header {
        background-color: #f0f0f0;
        text-align: center;
        font-weight: bold;
        color: black;
        padding: 10px;
    }

    .time {
        background-color: #f0f0f0;
        text-align: center;
        font-weight: bold;
        padding: 10px;
        color: black;
    }

    .cell {
        background-color: #fff;
        border: 1px solid #ddd;
        padding: 10px;
        text-align: center;
    }

    .header,
    .time {
        font-size: 16px;
    }

    .time-block {
        width: 100%;
        height: 100%;
        padding: 0.3rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        font-size: 0.9rem;
        font-weight: 500;
    }
</style>
  
