<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api";

  // let data = [];
  let data: any;
  // Filas seleccionadas
  let selectedRange = { start: 0, end: 0 };

  async function openFile() {
    try {
      // Abre explorador de archivos para seleccionar hoja de calculo
      const filePath = await open({
        filters: [{ name: "Excel Files", extensions: ["xlsx"] }],
      });

      // Si el usuario selecciono archivo entonces intentamos llamar a rust para que haga magia
      if (filePath) {
        console.log("Selected file:", filePath);
        const rows = await invoke("read_excel", { filePath });
        data = rows;
        console.log("Excel data:", rows);
      }
    } catch (e) {
      // Debug
      console.log(e);
    }
  }

  function setRange(start: any, end: any) {
    selectedRange.start = start;
    selectedRange.end = end;
    console.log("Selected range:", selectedRange);
  }
</script>

<button on:click={openFile}>Import</button>

{#if data.length > 0}
  <div>
    <h3>Muestrame los rangos para obtener los datos</h3>
    <table>
      <thead>
        <tr>
          {#each data[0].cells as _, index}
            <th>Column {index + 1}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each data as row, rowIndex}
          <tr>
            {#each row.cells as cell}
              <td>{cell}</td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
    <label>
      Inicio: <input type="number" min="0" max="{data.length - 1}" bind:value="{selectedRange.start}" />
    </label>
    <label>
      Final: <input type="number" min="0" max="{data.length - 1}" bind:value="{selectedRange.end}" />
    </label>
    <button on:click="{() => setRange(selectedRange.start, selectedRange.end)}">
        Confirmar rango {selectedRange.start} a {selectedRange.end}
    </button>
  </div>
{/if}
