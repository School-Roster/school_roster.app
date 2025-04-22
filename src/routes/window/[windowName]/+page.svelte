<!--------------------
Cada pagina de la aplicacion se renderiza
y crea una nueva ventana, este componente
simplemente llama a la vista deseada.
---------------------->

<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;

  /* Formularios */
  import SubjectsView from '$lib/components/forms/subjects/SubjectsView.svelte';
  import TeachersView from '$lib/components/forms/teachers/TeachersView.svelte';
  import GroupsView from '$lib/components/forms/groups/GroupsView.svelte';
  import ClassroomView from '$lib/components/forms/classrooms/ClassroomView.svelte';
  import loginView from'$lib/components/forms/login/login-view.svelte';
  import AiScheduler from '$lib/components/utils/AIScheduler.svelte';

  /* Utilidad */
  import SettingsView from '$lib/components/utils/Settings.svelte';
  import NotFoundView from '$lib/components/utils/NotFound.svelte';
 
  let view: any;
  switch (data.page) {
    case 'subjects':
      view = SubjectsView;
      break;
    case 'teachers':
      view = TeachersView;
      break;
    case 'settings':
      view = SettingsView;
      break;
    case 'groups':
      view = GroupsView;
      break;
    case 'classroom':
      view = ClassroomView;
      break;
    case 'login':
      view = loginView;
      break;
    case 'ai':
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

  const applySystemTheme = () => {
    const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
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
  }

  onMount(() => {
    applyTheme();
  });
</script>

<svelte:component this={view} />
