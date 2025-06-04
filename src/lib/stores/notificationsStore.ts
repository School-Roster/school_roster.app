import { writable } from 'svelte/store';

interface Notification {
    id: string;
    message: string;
    type: 'success' | 'error' | 'warning' | 'info';
    timeout?: number;
}

export const notifications = writable<Notification[]>([]);

export function addNotification(notification: Omit<Notification, 'id'>) {
    const id = Math.random().toString(36).substring(2, 9);
    notifications.update((current) => [...current, { ...notification, id }]);
    
    if (notification.timeout !== 0) {
        setTimeout(() => {
            removeNotification(id);
        }, notification.timeout || 3000);
    }
    
    return id;
}

export function removeNotification(id: string) {
    notifications.update((current) => current.filter((n) => n.id !== id));
}
