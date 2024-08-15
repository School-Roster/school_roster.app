export const MenuData = [
  /* TODO: Estos 3 tienen el hover submenú */
  { it: 1, name: "Archivo", icon: "/bar_icons/file.svg", menu: "principal", submenu: ["Nuevo", "Abrir", "Guardar", "Guardar como", "Cerrar"] },
  { it: 0, name: "Vista previa", icon: "/bar_icons/eye.svg", menu: "principal", submenu: ["Vista previa maestro", "Vista previa grupo", "Vista previa materia", "Vista previa aula"] },
  { it: 0, name: "Imprimir", icon: "/bar_icons/print.svg", menu: "principal", submenu: ["Imprimir maestro", "Imprimir grupo", "Imprimir materia", "Imprimir aula", "Imprimir horario"] },

  { it: 2, name: "Grupos", icon: "/bar_icons/group.svg", menu: "VistaGrupos", submenu: [] },
  { it: 0, name: "Materias", icon: "/bar_icons/books.svg", menu: "VistaMaterias", submenu: [] },
  { it: 0, name: "Maestros", icon: "/bar_icons/teacher.svg", menu: "VistaMaestros", submenu: [] },
  { it: 0, name: "Salones", icon: "/bar_icons/door.svg", menu: "VistaAulas", submenu: [] },

  { it: 3, name: "Inteligencia artificial", icon: "/bar_icons/robot.svg", menu: "FormMaterias", submenu: [] },
  { it: 0, name: "Historial", icon: "/bar_icons/clock.svg", menu: "FormAulas", submenu: [] },

  { it: 4, name: "Validar", icon: "/bar_icons/signature.svg", menu: "Login", submenu: [] },
  { it: 0, name: "Escuela", icon: "/bar_icons/school.svg", menu: "FormMaestros", submenu: [] },
];
