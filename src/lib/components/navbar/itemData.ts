/*
  Este archivo contiene la información de los items que se
  muestran en el menú de la aplicación. Cada item tiene un
  nombre, un icono y un menú al que pertenece. Además, cada
  item puede tener un submenu con otros items.

  * it: Posicion
  * name: Nombre que se muestra
  * icon: Icono que muestra
  * menu: Nombre de funcion que despliega
  * submenu: Arreglo para items que aparecen en hover
*/

export const itemData = [
  {
    it: 1, name: "Archivo", icon: "/icons/file.svg", menu: "submenu",
    submenu: [ { name: "Nuevo", icon: "/icons/new.svg", menu: "newSchedule"},
      { name: "Abrir", icon: "/icons/open.svg", menu: "import"},
      { name: "Guardar horario", icon: "/icons/saveas.svg", menu: "export"},
      { name: "Cerrar", icon: "/icons/close.svg", menu: "deleteAll"},
    ],
  },
  {
    it: 0, name: "Vista previa", icon: "/icons/eye.svg", menu: "submenu",
    submenu: [
      { name: "Vista previa profesor", icon: "/icons/preview.svg", menu: "teacherSchedule", submenu: []},
      { name: "Vista previa grupo", icon: "/icons/preview.svg", menu: "groupSchedule", submenu: []},
      { name: "Vista previa aula", icon: "/icons/preview.svg", menu: "classroomSchedule"},
    ],
  },
  {
    it: 0, name: "Exportar listas", icon: "/icons/print.svg", menu: "submenu",
    submenu: [
      { name: "Exportar listas de profesores", icon: "/icons/pdf.svg", menu: "teachersLists"},
    ],
  },

  { it: 2, name: "Grupos", icon: "/icons/group.svg", menu: "groups", submenu: [] },
  { it: 0, name: "Materias", icon: "/icons/books.svg", menu: "subjects", submenu: [] },
  { it: 0, name: "Profesores", icon: "/icons/teacher.svg", menu: "teachers", submenu: [] },
  { it: 0, name: "Aulas", icon: "/icons/door.svg", menu: "classroom", submenu: [] },

  { it: 3, name: "IA", icon: "/icons/robot.svg", menu: "ai", submenu: [] },
  { it: 0, name: "Generar horario", icon: "/icons/edit.svg", menu: "generate", submenu: [] },

  { it: 0, name: "Configuracion", icon: "/icons/school.svg", menu: "settings", submenu: [] },

  { it: 0, name: "Login", icon: "/icons/login.svg", menu: "login", submenu: [] },


];
