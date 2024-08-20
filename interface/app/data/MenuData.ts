export const MenuData = [
  /* TODO: Estos 3 tienen el hover submenú */
  {
    it: 1, name: "Archivo", icon: "/app_icons/file.svg", menu: "principal",
    submenu: [
      { name: "Nuevo", icon: "/app_icons/new.svg", menu: "principal"},
      { name: "Abrir", icon: "/app_icons/open.svg", menu: "principal"},
      { name: "Guardar", icon: "/app_icons/save.svg", menu: "principal"},
      { name: "Guardar como", icon: "/app_icons/saveas.svg", menu: "principal"},
      { name: "Cerrar todo", icon: "/app_icons/close.svg", menu: "principal"},
    ],
  },
  {
    it: 0, name: "Vista previa", icon: "/app_icons/eye.svg", menu: "principal",
    submenu: [
      { name: "Vista previa maestro", icon: "/app_icons/preview.svg", menu: "principal"},
      { name: "Vista previa grupo", icon: "/app_icons/preview.svg", menu: "principal"},
      { name: "Vista previa materia", icon: "/app_icons/preview.svg", menu: "principal"},
      { name: "Vista previa aula", icon: "/app_icons/preview.svg", menu: "principal"},
    ],
  },
  {
    it: 0, name: "Imprimir", icon: "/app_icons/print.svg", menu: "principal",
    submenu: [
      { name: "Horario maestros", icon: "/app_icons/pdf.svg", menu: "principal"},
      { name: "Horario grupos", icon: "/app_icons/pdf.svg", menu: "principal"},
      { name: "Horario materias", icon: "/app_icons/pdf.svg", menu: "principal"},
      { name: "Horario aulas", icon: "/app_icons/pdf.svg", menu: "principal"},
      { name: "Descargar todo", icon: "/app_icons/pdf.svg", menu: "principal"},
    ],
  },

  { it: 2, name: "Grupos", icon: "/app_icons/group.svg", menu: "VistaGrupos", submenu: [] },
  { it: 0, name: "Materias", icon: "/app_icons/books.svg", menu: "VistaMaterias", submenu: [] },
  { it: 0, name: "Maestros", icon: "/app_icons/teacher.svg", menu: "VistaMaestros", submenu: [] },
  { it: 0, name: "Aulas", icon: "/app_icons/door.svg", menu: "VistaAulas", submenu: [] },

  { it: 3, name: "Inteligencia artificial", icon: "/app_icons/robot.svg", menu: "FormMaterias", submenu: [] },
  { it: 0, name: "Historial", icon: "/app_icons/clock.svg", menu: "FormAulas", submenu: [] },

  { it: 4, name: "Validar", icon: "/app_icons/signature.svg", menu: "Login", submenu: [] },
  { it: 0, name: "Escuela", icon: "/app_icons/school.svg", menu: "FormMaestros", submenu: [] },
];
