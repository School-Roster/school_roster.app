<!--------------------
Cada pagina de la aplicacion se renderiza
y crea una nueva ventana, este componente
simplemente llama a la vista deseada.
---------------------->

<script lang="ts">
  import type { PageData } from "./$types";
  export let data: PageData;

  import Notification from "$lib/components/utils/Notification.svelte";

  /* Formularios */
  import SubjectsView from "$lib/components/forms/subjects/SubjectsView.svelte";
  import TeachersView from "$lib/components/forms/teachers/TeachersView.svelte";
  import GroupsView from "$lib/components/forms/groups/GroupsView.svelte";
  import ClassroomView from "$lib/components/forms/classrooms/ClassroomView.svelte";
  import loginView from "$lib/components/forms/login/login-view.svelte";
  import AiScheduler from "$lib/components/utils/AIScheduler.svelte";

  /* Utilidad */
  import SettingsView from "$lib/components/utils/Settings.svelte";
  import NotFoundView from "$lib/components/utils/NotFound.svelte";
  import MappingView from "$lib/components/utils/Mapping.svelte";
  import TeachersList from "$lib/components/utils/TeachersLists.svelte";

  /* Vistas previas */
  import TeacherSchedule from "$lib/components/utils/schedules/TeacherSchedule.svelte";
  import GroupSchedule from "$lib/components/utils/schedules/GroupSchedule.svelte";
  // import SubjectSchedule from "$lib/components/utils/schedules/SubjectSchedule.svelte";

  let view: any;
  switch (data.page) {
    case "subjects":
      view = SubjectsView;
      break;
    case "teachers":
      view = TeachersView;
      break;
    case "settings":
      view = SettingsView;
      break;
    case "groups":
      view = GroupsView;
      break;
    case "classroom":
      view = ClassroomView;
      break;
    case "mapping":
      view = MappingView;
      break;
    case "teacherSchedule":
      view = TeacherSchedule;
      break;
    case "groupSchedule":
      view = GroupSchedule;
      break;
    case "classroomSchedule":
      view = ClassroomSchedule;
      break;
    case "teachersLists":
      view = TeachersLists;
      break;
    case "login":
      view = loginView;
      break;
    case "ai":
      view = AiScheduler;
      break;
    default:
      view = NotFoundView;
      break;
  }

  /**
   * Carga el tema de la aplicación
   **/
  import { onMount } from "svelte";
    import ClassroomSchedule from "$lib/components/utils/schedules/ClassroomSchedule.svelte";
    import TeachersLists from "$lib/components/utils/TeachersLists.svelte";

  const applySystemTheme = () => {
    const darkModeMediaQuery = window.matchMedia(
      "(prefers-color-scheme: dark)",
    );
    if (darkModeMediaQuery.matches) {
      document.body.classList.add("dark");
    } else {
      document.body.classList.remove("dark");
    }
  };

  const applyTheme = () => {
    const theme = localStorage.getItem("theme");
    if (theme === "dark") {
      document.body.classList.add("dark");
    } else if (theme === "light") {
      document.body.classList.remove("dark");
    } else {
      applySystemTheme();
    }
  };

  onMount(() => {
    applyTheme();
  });
</script>

<svelte:component this={view} />

<slot />
<Notification />
