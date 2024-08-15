<script lang="ts">
  import { MenuData } from "../data/MenuData";
  let selectedMenu = "principal";
  function changeMenu(menu: string) {
    selectedMenu = menu;
    console.log(selectedMenu);
  }
</script>

<nav class="antialiased">
  <div class="sidebar">

    <div class="logo">
      <img src="/logicirculo.svg" alt="Logo" />
      <h1>School Schedule</h1>
    </div>

    {#each MenuData as item}
      <!-- Conditional rendering -->
      {#if item.it === 2 || item.it === 4} <div class="lg:mt-4"></div> {/if}
      {#if item.it === 1} <span class="menu"> Menu </span> {/if}
      {#if item.it === 3} <span class="menu"> Utilidad </span> {/if}
      <div class="menu-item">
        <button
          class="btn"
          data-menu={item.menu}
          data-icon={item.icon}
          data-name={item.name}
          on:click={() => changeMenu(item.menu)}
        >
          <img src={item.icon} alt={item.name} />
          <span>{item.name}</span>
        </button>
        {#if item.submenu.length > 0}
          <ul class="submenu"> 
            {#each item.submenu as subitem}
              <li>{subitem}</li>
            {/each}
          </ul>
        {/if}
      </div>
    {/each}
  </div>
</nav>

<style>
  nav {
    background-color: var(--blue-dark);
    color: var(--white);
    display: flex;

    width: 240px;
    height: 100vh;

    border-radius: 0 6px 6px 0;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);

    position: relative;
    flex-direction: column;
    flex-shrink: 0;

    /* overflow-y: auto; */
    /* overflow-x: hidden; */
  }

  .sidebar {
    font-family: "Inria Serif";
    width: 100%;
    height: 100%;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    transition-duration: 0.2s;
  }

  .sidebar .logo {
    display: flex;
    align-items: center;
    margin-bottom: 1.2rem;
    padding-top: 1rem;
    padding-left: 0.5rem;
    border-radius: 10px;
  }

  .sidebar .logo h1 {
    font-family: "Inria Serif";
    font-size: 18px;
    margin-left: 6px;
  }

  .sidebar .logo img {
    width: 50px;
    height: 50px;
  }

  .sidebar .btn img {
    width: 20px;
    height: 20px;
    margin-right: 1rem;
    filter: invert(1);
  }

  .sidebar .btn span {
    font-size: 12px;
  }

  .sidebar .menu {
    font-size: 14px;
    margin-top: 0.75rem;
    padding-left: 1rem;
    margin-bottom: 0.5rem;
    color: var(--gray);
  }

  .btn {
    display: flex;
    align-items: center;
    padding: 0.5rem 1rem;
    border-radius: 10px;
    margin-bottom: 0.5rem;
    cursor: pointer;
    transition-duration: 0.2s;
  }

  .btn:hover {
    background-color: var(--blue-light);
  }

  .btn:active {
    background-color: var(--blue-light);
  }

  .menu-item {
    position: relative;
  }

  /* Submenu */
  .submenu {
    list-style: none;
    padding-left: 0;
    margin: 0;
    display: none;
    position: absolute;
    top: 0; /* Aligns the submenu at the top of the button */
    left: 100%; /* Places the submenu to the right of the button */
    background-color: var(--blue-light);
    border-radius: 10px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    z-index: 1;
    width: 180px;
  }

  .submenu li {
    padding: 10px 15px;
    color: var(--white);
    font-size: 12px;
    cursor: pointer;
  }

  .submenu li:hover {
    background-color: var(--blue-dark);
  }

  /* Show submenu on hover */
  .menu-item:hover .submenu {
    display: block;
  }
</style>
