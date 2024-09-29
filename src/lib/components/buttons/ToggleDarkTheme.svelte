<script lang="ts">
  import "$styles/global.scss";
  let selectedTheme = localStorage.getItem("theme") || "system";

  // Función para cambiar el tema (light, dark, system)
  const changeTheme = (event: Event) => {
    const theme = (event.target as HTMLSelectElement).value;

    if (theme === "light") {
      document.body.classList.remove("dark");
      localStorage.setItem("theme", "light");
    } else if (theme === "dark") {
      document.body.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else if (theme === "system") {
      document.body.classList.remove("dark");
      localStorage.removeItem("theme");
      applySystemTheme();
    }
  };

  // Función para aplicar el tema del sistema
  export const applySystemTheme = () => {
    const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

    // matches regresa true si el tema del sistema es dark
    if (darkModeMediaQuery.matches) {
      document.body.classList.add("dark");
    } else {
      document.body.classList.remove("dark");
    }
  };

  // Inicializar el tema al cargar la página
  const storedTheme = localStorage.getItem("theme");
  if (storedTheme === "dark") {
    document.body.classList.add("dark");
  } else if (storedTheme === "light") {
    document.body.classList.remove("dark");
  } else {
    applySystemTheme();
  }
</script>

<select class="toggle-section" on:change={changeTheme} bind:value={selectedTheme}>
  <option value="light">Tema claro</option>
  <option value="dark">Tema oscuro</option>
  <option value="system">Sistema</option>
</select>
