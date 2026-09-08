<script setup lang="ts">


import{
    nextTick, // Позволяет дождаться момента когда vue обновит html 
    onMounted,
    useTemplateRef, // Даёт возможность получить ссылку на html-элемент из template
    watch, //  Позволяет следить за изменением выбранных данных
} from "vue";

import MessageBubble from "./messageBubble.vue";
import type { Message } from "../types/message.ts";

const props = defineProps<{
    messages: Message[];
}>();

const bottomAnchor = useTemplateRef<HTMLDivElement>("bottom-anchor");

async function scrollToBottom() {
    // Нужно дождаться обновления DOM
    await nextTick();

    bottomAnchor.value?.scrollIntoView({
        behavior: "smooth",

        block: "end",
    });
}

function getMessageCount() {
    return props.messages.length;
}

watch(
    getMessageCount,
    scrollToBottom,
);

onMounted(scrollToBottom);

</script>

<template>
    <div class="messages">
        <div class="messages-inner">
            <div v-if="messages.length === 0" class="empty">
                <!-- Данный div будет отображаться когда сообщений нет -->
                <strong> Here empty for now </strong>
                <span> Enter your first message </span>
            </div>
            <!== Vue Создаёт article для каждого сообщения из базы -->
            <MessageBubble v-for="message in messages" :key="message.id" :message="message" />
            <div ref="bottom-anchor" class="bottom-anchor" aria-hidden="true"></div>
        </div>
    </div>
</template>

<style scoped>

.buttom-anchor {
    height: 1px;
    flex-shrink: 0;
}

.messages {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
}

.messages-inner{
    min-height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: 10px;
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