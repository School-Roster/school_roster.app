<script>
  let value;
</script>

<main>
  z
  <div class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" />
        <h1>Grupos</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>
      <p class="titulo">Registro de Grupos</p>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100% "
      >
        <img style="width:35px; align-self:end" src="/antes.svg" />
      </div>
    </div>

    <div class="card">
      <div style="display: flex; flex-direction:column; margin:auto">
        <div style="display: flex; flex-direction:column; margin:auto">
          <input
            type="text"
            placeholder="Grado"
            style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px;"
            bind:value={grado}
          />
          <div style="display: flex; flex-direction:column; margin:auto">
            <input
              type="text"
              placeholder="Grupo"
              style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px; "
              bind:value={grupo}
            />
            <div style="display: flex; flex-direction:column; margin:auto">
              <input
                type="text"
                placeholder="Carrera"
                style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px;"
                bind:value={carrera}
              />
              <div style="display: flex; flex-direction:column; margin:auto">
                <input
                  type="text"
                  placeholder="Turno"
                  style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px;"
                  bind:value={turno}
                />
                <div style="display: flex; flex-direction:column; margin:auto">
                  <input
                    type="text"
                    placeholder="Cantidad de alumnos"
                    style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px;"
                    bind:value={cantidadAlumnos}
                  />
                  <button type="button">Guardar</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</main>

<style>
  .prueba {
    display: flex;
    flex-direction: row;
    width: 50%;
    margin: 100px auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 50px;
    border-radius: 5rem;
  }
  h1 {
    color: var(--azulp);
    font-size: 28px;
  }

  p {
    color: #5f6c7b;
    font-weight: bold;
    font-size: 20px;
    margin-left: 50px;
  }

  .card {
    background-color: #ffffff;
    height: 600px;
    width: 70%;
    display: flex;
    flex-direction: column;
    border-color: #000000;
    border: 1px solid;
    border-radius: 20px;
  }

  input[type="text"] {
    border-radius: 20px;
    margin-bottom: 15px;
    background-color: #edf4f8;
    border-color: rgb(148 163 184);
    height: 3rem;
    width: 35rem;
    justify-content: center;
    align-items: center;
    font-weight: bold;
  }

  button {
    background-color: #ef4565;
    border-color: #ef4565;
    height: 3rem;
    width: 18rem;
    border-radius: 50px;
    color: #ffffff;
    font-size: 1rem;
    justify-content: left;
    font-weight: bold;
    border: none;
  }
</style>
