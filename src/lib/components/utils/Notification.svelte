<script lang="ts">
  import { notifications, removeNotification } from "$lib/stores/notificationsStore";

  $: currentNotifications = $notifications;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
<div class="notifications-container">
  {#each currentNotifications as notification (notification.id)}
    <div
      class="notification {notification.type}"
      on:click={() => removeNotification(notification.id)}
    >
      <div class="message">{notification.message}</div>
    </div>
  {/each}
</div>

<style lang="scss">
  @use "../../../styles/_variables.scss";
  .notifications-container {
    position: fixed;
    top: 20px;
    right: 20px;
    z-index: 1000;
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-width: 400px;
  }

  .notification {
    padding: 15px;
    border-radius: 5px;
    color: white;
    box-shadow: 0 3px 10px rgba(0, 0, 0, 0.2);
    cursor: pointer;
    transition: all 0.3s ease;
    transform: translateX(0);
    opacity: 1;

    &:hover {
      transform: translateX(-5px);
    }

    &.success {
      background-color: variables.$blue;
    }

    &.error {
      background-color: variables.$red;
    }

    &.warning {
      background-color: variables.$other-color;
    }

    &.info {
      background-color: variables.$blue;
    }

    .message {
      font-size: 12px;
    }
  }
</style>
