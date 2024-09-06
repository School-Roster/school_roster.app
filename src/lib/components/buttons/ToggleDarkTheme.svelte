<script lang="ts">
  import "$styles/global.scss";

  let selectedTheme = localStorage.getItem("theme") || "system";

  const changeTheme = () => {
    if (selectedTheme === "light") {
      selectedTheme = "dark";
      document.body.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      selectedTheme = "light";
      document.body.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  };

  const applySystemTheme = () => {
    const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    if (darkModeMediaQuery.matches) {
      document.body.classList.add("dark");
    } else {
      document.body.classList.remove("dark");
    }
  };

  // Inicializar el tema
  if (selectedTheme === "dark") {
    document.body.classList.add("dark");
  } else if (selectedTheme === "light") {
    document.body.classList.remove("dark");
  } else {
    applySystemTheme();
  }
</script>

<button class="theme-toggle" on:click={changeTheme} aria-label="Cambiar tema">
  <span class="icon light">🌞</span>
  <span class="icon dark">🌙</span>
</button>

<style lang="scss">
  .theme-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 50px;
    height: 50px;
    background: var(--toggle-bg);
    color: var(--text-color);
    border: none;
    border-radius: 50%;
    cursor: pointer;
    transition: background 0.3s, transform 0.2s;
    box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.2);

    &:hover {
      transform: scale(1.1);
      background: var(--toggle-hover);
    }

    &:active {
      transform: scale(0.9);
    }

    .icon {
      font-size: 1.5rem;
      transition: opacity 0.3s;
    }

    .light {
      opacity: 1;
    }

    .dark {
      opacity: 0;
    }
  }

  .dark .theme-toggle {
    .light {
      opacity: 0;
    }

    .dark {
      opacity: 1;
    }
  }

  /* Variables */
  :root {
    --toggle-bg: #e0e0e0;
    --toggle-hover: #d0d0d0;
    --text-color: #333;
  }

  .dark {
    --toggle-bg: #333;
    --toggle-hover: #444;
    --text-color: #f5f5f5;
  }
</style>
