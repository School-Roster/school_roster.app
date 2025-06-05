<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writeBinaryFile } from "@tauri-apps/api/fs";
  import * as XLSX from "xlsx";
  import {
    assignmentsStore,
    loadAssignments,
    type AssignmentItem,
  } from "$lib/modules/entities/assignments";
  import { teachers, loadTeachers } from "$lib/modules/entities/teachersStore";
  import {
    groups,
    loadGroups,
    type Student,
  } from "$lib/modules/entities/groupsStore";
  import { addNotification } from "$lib/stores/notificationsStore";
  import { schoolStore, loadSchoolInfo } from "$lib/modules/config/configStore";
  import "$styles/teachers-lists.scss";

  let selectedTeacherId: number | null = null;
  let teacherGroups: Array<{
    groupId: number;
    groupName: string;
    subjects: string[];
    studentCount: number;
  }> = [];
  let studentsData: Array<{
    groupId: number;
    groupName: string;
    students: Student[];
  }> = [];
  let isLoading = false;

  onMount(async () => {
    await loadAssignments();
    await loadTeachers();
    await loadGroups();
    await loadSchoolInfo();
  });

  $: teachersList = $teachers;
  $: groupsList = $groups;
  $: assignmentsMap = $assignmentsStore;

  // Función para obtener los grupos asignados a un profesor
  function getTeacherGroups(teacherId: number) {
    const assignments = Array.from(assignmentsMap.values()).filter(
      (assignment) => assignment.teacherId === teacherId,
    );

    // Agrupar por grupo
    const groupsMap = new Map<
      number,
      {
        groupId: number;
        groupName: string;
        subjects: Set<string>;
      }
    >();

    assignments.forEach((assignment) => {
      const group = groupsList.find((g) => g.id === assignment.groupId);
      if (group) {
        const groupName = `${group.grade}${group.group} - ${group.career}`;

        if (!groupsMap.has(assignment.groupId)) {
          groupsMap.set(assignment.groupId, {
            groupId: assignment.groupId,
            groupName,
            subjects: new Set(),
          });
        }

        groupsMap
          .get(assignment.groupId)!
          .subjects.add(assignment.subject_name);
      }
    });

    return Array.from(groupsMap.values()).map((group) => ({
      groupId: group.groupId,
      groupName: group.groupName,
      subjects: Array.from(group.subjects),
      studentCount: 0, // Se llenará cuando carguemos los estudiantes
    }));
  }

  // Función para cargar estudiantes de los grupos del profesor
  async function loadStudentsForTeacher(teacherId: number) {
    if (!teacherId) return;

    isLoading = true;
    try {
      const groups = getTeacherGroups(teacherId);
      teacherGroups = groups;

      // Cargar estudiantes para cada grupo
      const studentsPromises = groups.map(async (group) => {
        try {
          const students = await invoke<Student[]>("get_students_by_group", {
            groupId: group.groupId,
          });

          return {
            groupId: group.groupId,
            groupName: group.groupName,
            students: students || [],
          };
        } catch (error) {
          console.error(
            `Error loading students for group ${group.groupId}:`,
            error,
          );
          return {
            groupId: group.groupId,
            groupName: group.groupName,
            students: [],
          };
        }
      });

      studentsData = await Promise.all(studentsPromises);

      // Actualizar el conteo de estudiantes
      teacherGroups = teacherGroups.map((group) => {
        const groupData = studentsData.find(
          (data) => data.groupId === group.groupId,
        );
        return {
          ...group,
          studentCount: groupData?.students.length || 0,
        };
      });
    } catch (error) {
      console.error("Error loading teacher data:", error);
      addNotification({
        message: "Error al cargar los datos del profesor",
        type: "error",
        timeout: 3000,
      });
    } finally {
      isLoading = false;
    }
  }

  // Recargar datos cuando cambie el profesor seleccionado
  $: if (selectedTeacherId) {
    loadStudentsForTeacher(selectedTeacherId);
  }

  // Función para exportar a Excel
  async function exportToExcel() {
    if (!selectedTeacherId || studentsData.length === 0) {
      addNotification({
        message: "Selecciona un profesor con grupos asignados",
        type: "warning",
        timeout: 2000,
      });
      return;
    }

    const selectedTeacher = teachersList.find(
      (t) => t.id === selectedTeacherId,
    );
    if (!selectedTeacher) return;

    try {
      isLoading = true;

      // Crear un nuevo workbook
      const workbook = XLSX.utils.book_new();

      // Crear una hoja para cada grupo
      studentsData.forEach((groupData) => {
        if (groupData.students.length === 0) return;

        // Preparar datos para la hoja
        const worksheetData = [
          [`LISTA DE ESTUDIANTES - ${groupData.groupName.toUpperCase()}`],
          [
            `Profesor(a): ${selectedTeacher.name} ${selectedTeacher.father_lastname}`,
          ],
          [
            `Ciclo Escolar: ${new Date().getFullYear()}-${new Date().getFullYear() + 1}`,
          ],
          [`Fecha de generación: ${new Date().toLocaleDateString("es-MX")}`],
          [], // Fila vacía
          [
            "No.",
            "Nombre",
            "Apellido Paterno",
            "Apellido Materno",
            "Nombre Completo",
          ],
          ...groupData.students.map((student, index) => [
            index + 1,
            student.name,
            student.father_lastname,
            student.mother_lastname || "",
            `${student.name} ${student.father_lastname} ${student.mother_lastname || ""}`.trim(),
          ]),
        ];

        // Crear la hoja de trabajo
        const worksheet = XLSX.utils.aoa_to_sheet(worksheetData);

        // Establecer ancho de columnas
        worksheet["!cols"] = [
          { wch: 5 }, // No.
          { wch: 20 }, // Nombre
          { wch: 20 }, // Apellido Paterno
          { wch: 20 }, // Apellido Materno
          { wch: 35 }, // Nombre Completo
        ];

        // Fusionar celdas para el título
        worksheet["!merges"] = [
          { s: { r: 0, c: 0 }, e: { r: 0, c: 4 } }, // Título del grupo
          { s: { r: 1, c: 0 }, e: { r: 1, c: 4 } }, // Profesor
          { s: { r: 2, c: 0 }, e: { r: 2, c: 4 } }, // Ciclo escolar
          { s: { r: 3, c: 0 }, e: { r: 3, c: 4 } }, // Fecha
        ];

        // Nombre de la hoja (máximo 31 caracteres para Excel)
        const sheetName = groupData.groupName.substring(0, 31);
        XLSX.utils.book_append_sheet(workbook, worksheet, sheetName);
      });

      // Si hay múltiples grupos, crear una hoja resumen
      if (studentsData.length > 1) {
        const summaryData = [
          [
            `RESUMEN - PROFESOR(A): ${selectedTeacher.name} ${selectedTeacher.father_lastname}`.toUpperCase(),
          ],
          [`Total de grupos: ${studentsData.length}`],
          [
            `Total de estudiantes: ${studentsData.reduce((sum, group) => sum + group.students.length, 0)}`,
          ],
          [],
          ["Grupo", "Materias que imparte", "Número de estudiantes"],
          ...teacherGroups.map((group) => [
            group.groupName,
            group.subjects.join(", "),
            group.studentCount,
          ]),
        ];

        const summarySheet = XLSX.utils.aoa_to_sheet(summaryData);
        summarySheet["!cols"] = [
          { wch: 25 }, // Grupo
          { wch: 40 }, // Materias
          { wch: 20 }, // Estudiantes
        ];

        summarySheet["!merges"] = [
          { s: { r: 0, c: 0 }, e: { r: 0, c: 2 } },
          { s: { r: 1, c: 0 }, e: { r: 1, c: 2 } },
          { s: { r: 2, c: 0 }, e: { r: 2, c: 2 } },
        ];

        XLSX.utils.book_append_sheet(workbook, summarySheet, "Resumen");
      }

      // Generar archivo Excel
      const excelBuffer = XLSX.write(workbook, {
        bookType: "xlsx",
        type: "array",
      });

      // Guardar archivo
      const teacherName =
        `${selectedTeacher.name}_${selectedTeacher.father_lastname}`.replace(
          /\s+/g,
          "_",
        );
      const fileName = `Listas_${teacherName}_${new Date().getFullYear()}.xlsx`;

      const path = await invoke<string>("save_excel_file", {
        defaultName: fileName,
      });

      if (path) {
        await writeBinaryFile({
          path,
          contents: new Uint8Array(excelBuffer),
        });

        addNotification({
          message: "Listas exportadas exitosamente",
          type: "success",
          timeout: 2000,
        });
      }
    } catch (error) {
      console.error("Error exportando Excel:", error);
      addNotification({
        message: "Error al exportar las listas",
        type: "error",
        timeout: 3000,
      });
    } finally {
      isLoading = false;
    }
  }
</script>
<div class="student-lists">
  <h2>Listas de Estudiantes por Profesor</h2>

  <div class="controls">
    <div class="select-container">
      <label for="teacher-select">Selecciona un profesor:</label>
      <select
        id="teacher-select"
        class="custom-select"
        bind:value={selectedTeacherId}
        disabled={isLoading}
      >
        <option disabled selected value={null}>Selecciona un profesor</option>
        {#each teachersList as teacher}
          <option value={teacher.id}>
            {teacher.name} {teacher.father_lastname}
          </option>
        {/each}
      </select>
    </div>

    {#if selectedTeacherId && teacherGroups.length > 0}
      <button
        class="export-button"
        on:click={exportToExcel}
        disabled={isLoading}
      >
        {#if isLoading}
          <span class="spinner">↻</span> Generando...
        {:else}
          Exportar Listas a Excel
        {/if}
      </button>
    {/if}
  </div>

  {#if isLoading}
    <div class="loading">
      <p>Cargando datos...</p>
    </div>
  {:else if selectedTeacherId && teacherGroups.length > 0}
    <div class="groups-info">
      <h3>Grupos asignados:</h3>
      <div class="groups-list">
        {#each teacherGroups as group}
          <div class="group-card">
            <h4>{group.groupName}</h4>
            <p><strong>Materias:</strong> {group.subjects.join(", ")}</p>
            <p><strong>Estudiantes:</strong> {group.studentCount}</p>
          </div>
        {/each}
      </div>

      <div class="summary">
        <p><strong>Total de grupos:</strong> {teacherGroups.length}</p>
        <p>
          <strong>Total de estudiantes:</strong>
          {teacherGroups.reduce((sum, group) => sum + group.studentCount, 0)}
        </p>
      </div>
    </div>
  {:else if selectedTeacherId}
    <div class="no-data">
      <p>Este profesor no tiene grupos asignados.</p>
    </div>
  {/if}
</div>
