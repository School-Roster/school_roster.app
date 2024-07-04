<script lang="ts">
  // import SafeBtn from "../components/SafeBtn.svelte";
  import ColorPicker from "../components/ColorPicker.svelte";

  import { invoke } from "@tauri-apps/api";

  let name = "";
  let shorten = "";
  let color = "#5E7ABC";
  let subject_type = "";

  let res = "";

  async function register_subject() {
    console.log("register_subject");
    res = await invoke("register_subject", {
      name,
      shorten,
      color,
      stype: subject_type,
    });
  }
</script>

<main>
  <div style="margin-top: 150px;" class="container">
    <div>
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" alt="logo" style="width: 60px;" />
        <h1 style="margin-top: 15px; margin-left: 5px; margin-right: 10px; font-size: 18px;">Materias</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100%; margin-top:100px "
      >
        <img
          style="width:45px; align-self:end margin-top:2px"
          src="/antes.svg"
          alt="antes"
        />
      </div>
    </div>

    <div class="card">
      <div class="placeholder" style="margin-top: 50px;">
        <input
          type="text"
          placeholder="Nombre"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={name}
        />

        <input
          type="text"
          placeholder="Abreviatura"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={shorten}
        />
        <input
          type="text"
          placeholder="Tipo"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={subject_type}
        />
      </div>
      <div
        style="margin-top: 10px;    display: flex;
              align-items: center;
              max-width: 8500px;
              margin: 20px;
              padding: 10px;
              background-color: #edf4f8;
              border-color:#094067;
              border-radius: 25px;
              box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);"
      >
        <ColorPicker bind:value={color} />
      </div>
      <span>{res}</span>
      <div style="margin-top: 30px; align-self: center;">
        <!-- <SafeBtn /> -->
        <button on:click="{register_subject}">Registrar</button>
      </div>
    </div>
  </div>
</main>

<style>
  .container {
    display: flex;
    flex-direction: row;
    width: 50%;
    height: 700px;
    margin: 100px auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 50px;
    border-radius: 5rem;
  }
  h1 {
    color: var(--azulp);
    font-size: 28px;
  }

  /* p { */
  /*   color: #5f6c7b; */
  /*   font-weight: bold; */
  /*   font-size: 20px; */
  /*   margin-left: 50px; */
  /* } */

  .card {
    background-color: #ffffff;
    height: 100%;
    width: 70%;
    display: flex;
    flex-direction: column;
    border-color: #000000;
    border: 1px solid;
    border-radius: 20px;
  }

  input[type="text"] {
    border-radius: 20px;
    margin-bottom: 10px;
    background-color: #edf4f8;
    height: 3rem;
    width: 350px;
    justify-content: center;
    align-items: center;
    border-color: #094067;

    margin-left: 25px;
  }
  button {
    background-color: #094067;
    border-color: #094067;
    height: 3rem;
    width: 100px;
    border-radius: 50px;
    color: #ffffff;
    font-size: 1rem;
    justify-content: left;
    font-weight: bold;
    border: none;
  }

  button:hover {
    background-color: #ffffff;
    color: #094067;
    transition-timing-function: ease-in-out;
    transition: background-color 0.5s;
  }
</style>
