<script>
  //  import newFileIcon from '../newFile.svg'
</script>

<nav>
  <ul>
    <li>
      <div class="logo">
      <a href="/#" target="_blank" title="logo">
      <img src="/logo.png" class="" alt="" />  
      </div>

     <div class="navbar1">
      <a href="/login.svelte" target="_blank" title="Nuevo archivo">
        <img src="/new_file.svg" class="" alt="" />
        <span>Nuevo archivo</span>
      </a>

       <a href="/#" target="_blank" title="Nueva carpeta">
        <img src="/new_folder.svg" class="" alt="" />
        <span>Nueva carpeta</span>
      </a>

       <a href="/#" target="_blank" title="Guardar">
        <img src="/save.svg" class="" alt="" />
        <span>Guadar</span>
      </a>

      <a href="/#" target="_blank" title="preview">
      <img src="/preview.svg" class="" alt="">
      <span>Vista previa</span>
      </a>

     
      <a href="/#" target="_blank" title="printer">
      <img src="/printer.svg" class="" alt="">
      <span>Imprimir</span>
      </a>

      </div>

      <div class="navbar2">
      <a href="/#" target="_blank" title="group">
      <img src="/group.svg" class="" alt="">
      <span>Grupos</span>
      </a>

       <a href="/#" target="_blank" title="materias">
      <img src="/materias.svg" class="" alt="">
      <span>Materias</span>
      </a>

       <a href="/#" target="_blank" title="maestros">
      <img src="/maestro.svg" class="" alt="">
      <span>Maestros</span>
      </a>

       <a href="/#" target="_blank" title="aulas">
      <img src="/aulas.svg" class="" alt="">
      <span>Aulas</span>
      </a>

      </div>

      <div>
       <a href="/#" target="_blank" title="IA">
      <img src="/ia.svg" class="" alt="">
      <span>IA</span>
      </a>
       <a href="/#" target="_blank" title="hv">
      <img src="/hv.svg" class="" alt="">
      <span>Historial de versiones</span>
      </a>

      </div>

      <div>
       <a href="/#" target="_blank" title="generador">
      <img src="/generador.svg" class="" alt="">
      <span>Generar horario</span>
      </a>

      </div>

      <div class="navbar3">
       <a href="/#" target="_blank" title="school">
      <img src="/school.svg" class="" alt="">
      <span>Escuela/ Institucion</span>
      </a>

       <a href="/#" target="_blank" title="nosotros">
      <img src="/nosotros.svg" class="" alt="">
      <span>Sobre nosotros</span>
      </a>

       <a href="/#" target="_blank" title="ayuda">
      <img src="/ayuda.svg" class="" alt="">
      <span>Ayuda</span>
      </a>
      </div>
    
    </li>
  </ul>

 
</nav>

<style>

 
 nav {
    background-color: #094067;
    flex-direction: row;
  }

  .logo{
    border-radius: 50%;
  }

  nav ul {
    display: flex;
    list-style: none;
    margin: 0;
    padding: 0;
    flex-direction: row;
  }

 

  a {
    font-size: 10px;
    color: aliceblue;
    padding-left: 0.5rem;
    padding: 0.2rem 0.2rem;
    display:inline-flex;
    flex-direction:column; /* Apila los elementos verticalmente */
    align-items:center; /* Centra los elementos horizontalmente */


  }


 

</style>
