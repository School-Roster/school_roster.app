<script context="module" lang="ts">
  export interface SubjectItem {
    id: number;
    name: string;
    shorten: string;
    color: string;
    type: string;
  }

</script>

<script lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { onMount } from 'svelte';
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';

  // export let items: SubjectItem[] = [
  //   { id: 1, name: "Math", shorten: "Math", color: "#F67280", type: "Science" },
  //   { id: 2, name: "Science", shorten: "Science", color: "#F9ED69", type: "Science" },
  //   { id: 3, name: "History", shorten: "History", color: "#46CDCF", type: "Science" },
  //   { id: 4, name: "English", shorten: "English", color: "#B1B2FF", type: "Science" },
  //   { id: 5, name: "Art", shorten: "Art", color: "#FFC7C7", type: "Science" }
  // ];
  export let items: SubjectItem[] = [];

  async function getSubject() {
    items = await invoke('list_subjects');
  }
  
  onMount(() => {
    getSubject();
  });
  const flipDurationMs = 150;

  const handleConsider = (e: any) => {
    items = e.detail.items;
  };

  const handleFinalize = (e: any) => {
    items = e.detail.items;
  };

</script>

<section
  use:dndzone={{ items, flipDurationMs }}
  on:consider={handleConsider}
  on:finalize={handleFinalize}
  class="items-container"
>
  {#each items as item (item.id)}
    <div class="subject" style="background-color: {item.color}" animate:flip="{{duration: flipDurationMs}}">
      {item.shorten}
    </div>
  {/each}
</section>

<style>
  .items-container {
    display: flex;
    gap: 10px;
    padding: 10px;
    border: 1px solid #ccc;
    background-color: white;
    min-height: 50px;
  }
  .subject {
    padding: 10px;
    border: 1px solid #ccc;
    min-height: 50px;
    max-width: 100px;
  }
</style>
