<script lang="ts">
  import {
    notifications,
    removeNotification,
  } from "$lib/stores/notificationsStore";

  import "$styles/notification.scss";

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
