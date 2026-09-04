<script setup lang="ts">

import MessageBubble from "./messageBubble.vue";
import type { Message } from "../types/message.ts";

defineProps<{
    messages: Message[];
}>();

</script>

<template>
    <div class="messages">
        <div v-if="messages.length === 0" class="empty">
            <!-- Данный div будет отображаться когда сообщений нет -->
            <strong> Here empty for now </strong>
            <span> Enter your first message </span>
        </div>
        <!== Vue Создаёт article для каждого сообщения из базы -->
            <MessageBubble v-for="message in messages" :key="message.id" :message="message" />
    </div>
</template>

<style scoped>
.messages {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 24px;
}

.empty {
    margin: auto;
    display: flex;
    flex-direction: column;
    gap: 6px;
    text-align: center;
    color: #858c98;
}
</style>