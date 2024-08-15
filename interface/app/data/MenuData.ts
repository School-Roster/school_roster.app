export const MenuData = [
  /* TODO: Estos 3 tienen el hover submenú */
  {
    it: 1, name: "Archivo", icon: "/bar_icons/file.svg", menu: "principal",
    submenu: [
      { name: "Nuevo", icon: "/bar_icons/new.svg", menu: "principal"},
      { name: "Abrir", icon: "/bar_icons/open.svg", menu: "principal"},
      { name: "Guardar", icon: "/bar_icons/save.svg", menu: "principal"},
      { name: "Guardar como", icon: "/bar_icons/saveas.svg", menu: "principal"},
      { name: "Cerrar todo", icon: "/bar_icons/close.svg", menu: "principal"},
    ],
  },
  {
    it: 0, name: "Vista previa", icon: "/bar_icons/eye.svg", menu: "principal",
    submenu: [
      { name: "Vista previa maestro", icon: "/bar_icons/preview.svg", menu: "principal"},
      { name: "Vista previa grupo", icon: "/bar_icons/preview.svg", menu: "principal"},
      { name: "Vista previa materia", icon: "/bar_icons/preview.svg", menu: "principal"},
      { name: "Vista previa aula", icon: "/bar_icons/preview.svg", menu: "principal"},
    ],
  },
  {
    it: 0, name: "Imprimir", icon: "/bar_icons/print.svg", menu: "principal",
    submenu: [
      { name: "Horario maestros", icon: "/bar_icons/pdf.svg", menu: "principal"},
      { name: "Horario grupos", icon: "/bar_icons/pdf.svg", menu: "principal"},
      { name: "Horario materias", icon: "/bar_icons/pdf.svg", menu: "principal"},
      { name: "Horario aulas", icon: "/bar_icons/pdf.svg", menu: "principal"},
      { name: "Descargar todo", icon: "/bar_icons/pdf.svg", menu: "principal"},
    ],
  },

  { it: 2, name: "Grupos", icon: "/bar_icons/group.svg", menu: "VistaGrupos", submenu: [] },
  { it: 0, name: "Materias", icon: "/bar_icons/books.svg", menu: "VistaMaterias", submenu: [] },
  { it: 0, name: "Maestros", icon: "/bar_icons/teacher.svg", menu: "VistaMaestros", submenu: [] },
  { it: 0, name: "Salones", icon: "/bar_icons/door.svg", menu: "VistaAulas", submenu: [] },

  { it: 3, name: "Inteligencia artificial", icon: "/bar_icons/robot.svg", menu: "FormMaterias", submenu: [] },
  { it: 0, name: "Historial", icon: "/bar_icons/clock.svg", menu: "FormAulas", submenu: [] },

  { it: 4, name: "Validar", icon: "/bar_icons/signature.svg", menu: "Login", submenu: [] },
  { it: 0, name: "Escuela", icon: "/bar_icons/school.svg", menu: "FormMaestros", submenu: [] },
];
