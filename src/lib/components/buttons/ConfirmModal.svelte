<script lang="ts"> 
  import "$styles/buttons/modal.scss";
  export let isOpen = false;
  export let onConfirm: () => void;
  export let onCancel: () => void;
  export let message: string;

  export let title: string = "Confirmar Eliminación";

  export let confirmText: string = "Sí, eliminar";
  export let cancelText: string = "Cancelar";

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
      <h1>{title}</h1>
      <p>{message ? message : "¿Estás seguro de que quieres eliminar este elemento?"}</p>
      <p style="font-size: 14px; margin-top: 0;">Esta acción no se puede revertir</p>
      <button class="btn-confirm" on:click={handleConfirm}>{confirmText}</button>
      <button class="btn-cancel" on:click={handleCancel}>{cancelText}</button>
    </div>
  </div>
{/if}
