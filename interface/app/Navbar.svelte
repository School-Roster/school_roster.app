<script>
  //  import newFileIcon from '../newFile.svg'
</script>

<nav class="flex items-center justify-between">
  <ul>
    <li>
      <a href="/#" target="_blank" title="Nuevo archivo">
        <img src="/new_file.svg" class="w-4 h-4" alt="" />
        <span>Nuevo archivo</span>
      </a>

      <a href="/#">nueva carpeta</a>

      <a href="/#">Guardar</a>

      <a href="/#">Vista previa</a>

      <a href="nosotros.svelte">Acerca de nosotros</a>


    </li>
  </ul>

 
</nav>

<style>
  nav {
    background-color: #094067;
    flex-direction: row;
  }

  nav ul {
    display: flex;
    list-style: none;
    margin: 0;
    padding: 0;
    flex-direction: row;
  }

  nav li {
    margin: 0;
  }

  a {
    color: aliceblue;
    padding: 0.5rem 1rem;
  }
</style>
