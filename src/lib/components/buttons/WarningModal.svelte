<script>
  export let message = '';
  export let visible = false;
  export let onClose = () => {};
</script>

{#if visible}
  <div class="modal-overlay">
    <div class="modal-content">
      <p class="modal-message">{message}</p>
      <button class="modal-button" on:click={onClose}>
        Cerrar
      </button>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 999;
  }

  .modal-content {
    background-color: white;
    padding: 24px;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
    max-width: 400px;
    width: 90%;
    text-align: center;
    animation: fadeIn 0.2s ease-out;
  }

  .modal-message {
    font-size: 18px;
    color: #333;
    margin-bottom: 16px;
  }

  .modal-button {
    background-color: #e53935;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 16px;
  }

  .modal-button:hover {
    background-color: #c62828;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
</style>