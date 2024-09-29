<script lang="ts">
  import "$styles/buttons/modal.scss";
  export let isOpen = false;
  export let onConfirm: () => void;
  export let onCancel: () => void;

  const handleConfirm = () => {
    onConfirm();
    isOpen = false;
  }

  const handleCancel = () => {
    onCancel();
    isOpen = false;
  }
</script>

{#if isOpen}
  <div class="modal-overlay">
    <div class="modal-content">
      <h1>Confirmar Eliminación</h1>
      <p>¿Estás seguro de que quieres eliminar esta materia?</p>
      <button class="btn-confirm" on:click={handleConfirm}>Sí, eliminar</button>
      <button class="btn-cancel" on:click={handleCancel}>Cancelar</button>
    </div>
  </div>
{/if}
