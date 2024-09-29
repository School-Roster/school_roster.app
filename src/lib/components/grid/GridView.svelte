<script lang="ts">
  import "$styles/grid.scss";
  export let days: string[] = ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes"];
  export let hours: string[] = [
    "7:00 - 7:50",
    "7:50 - 8:40",
    "8:40 - 9:30",
    "9:30 - 10:20",
    "10:20 - 11:10",
    "11:10 - 12:00",
    "12:00 - 12:50",
    "12:50 - 1:40",
    "1:40 - 2:30",
  ];
</script>

<section class="grid">
  <div class="header"></div>
  {#each days as day}
    <div class="header">{day}</div>
  {/each}

  {#each hours as hour}
    <div class="header">{hour}</div>
    {#each days as _day}
      <div>
      </div>
    {/each}
  {/each}
</section>
