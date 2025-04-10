/*
  Este archivo contiene la información de los items que se
  muestran en el menú de la aplicación. Cada item tiene un
  nombre, un icono y un menú al que pertenece. Además, cada
  item puede tener un submenu con otros items.
*/

export const itemData = [
  {
    it: 1, name: "Archivo", icon: "/icons/file.svg", menu: "todo",
    submenu: [ { name: "Nuevo", icon: "/icons/new.svg", menu: "todo"},
      { name: "Abrir", icon: "/icons/open.svg", menu: "todo"},
      { name: "Guardar", icon: "/icons/save.svg", menu: "todo"},
      { name: "Guardar como", icon: "/icons/saveas.svg", menu: "todo"},
      { name: "Cerrar todo", icon: "/icons/close.svg", menu: "todo"},
    ],
  },
  {
    it: 0, name: "Vista previa", icon: "/icons/eye.svg", menu: "todo",
    submenu: [
      { name: "Vista previa profesor", icon: "/icons/preview.svg", menu: "teacherSchedule", submenu: []},
      { name: "Vista previa grupo", icon: "/icons/preview.svg", menu: "todo"},
      { name: "Vista previa materia", icon: "/icons/preview.svg", menu: "todo"},
      { name: "Vista previa aula", icon: "/icons/preview.svg", menu: "todo"},
    ],
  },
  {
    it: 0, name: "Imprimir", icon: "/icons/print.svg", menu: "todo",
    submenu: [
      { name: "Horario profesores", icon: "/icons/pdf.svg", menu: "todo"},
      { name: "Horario grupos", icon: "/icons/pdf.svg", menu: "todo"},
      { name: "Horario materias", icon: "/icons/pdf.svg", menu: "todo"},
      { name: "Horario aulas", icon: "/icons/pdf.svg", menu: "todo"},
      { name: "Descargar todo", icon: "/icons/pdf.svg", menu: "todo"},
    ],
  },

  { it: 2, name: "Grupos", icon: "/icons/group.svg", menu: "groups", submenu: [] },
  { it: 0, name: "Materias", icon: "/icons/books.svg", menu: "subjects", submenu: [] },
  { it: 0, name: "Profesores", icon: "/icons/teacher.svg", menu: "teachers", submenu: [] },
  { it: 0, name: "Aulas", icon: "/icons/door.svg", menu: "classroom", submenu: [] },

  { it: 3, name: "IA", icon: "/icons/robot.svg", menu: "todo", submenu: [] },
  { it: 0, name: "Historial", icon: "/icons/clock.svg", menu: "todo", submenu: [] },

  { it: 0, name: "Configuracion", icon: "/icons/school.svg", menu: "settings", submenu: [] },

  { it: 0, name: "Login", icon: "/icons/login.svg", menu: "login", submenu: [] },
  { it: 0, name: "Generar horario", icon: "/icons/edit.svg", menu: "generate", submenu: [] },


];
