<script setup lang="ts">

import type { Chat } from '../types/chats';

defineProps<{
    chats: Chat[];

    activeChatId: number;
}>()

const emit = defineEmits<{
    select: [chat: Chat];
}>()

function selectChat(chat: Chat) {
    emit('select', chat);
}

</script>

<template>
    <aside class="sidebar">
        <div class="sidebar__header">
            Chats
        </div>

        <div class="sidebar__list">
            <button 
                v-for="chat in chats" 
                :key="chat.id" 
                type="button" 
                class="chat-button" 
                :class="{'chat-button--active': chat.id === activeChatId}" @click="selectChat(chat)"
            >
                <strong class="chat-button__title">
                    {{ chat.title }}
                </strong>

                <span class="chat-button__subtitle">
                    {{ chat.subtitle }}
                </span>
            </button>
        </div>
    </aside>
</template>

<style scoped>
.sidebar{
    width: 260px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;

    min-height: 0;

    border-right: 1px solid #252830;

    background: #17191f;
}
.sidebar__header{
    flex-shrink: 0;
    padding: 15px 20px;
    font-weight: 600;
    border-bottom: 1px solid #252830;
}
.sidebar__list{
    flex: 1;
    overflow-y: auto;
    padding: 8px;
}

.chat-button{
    width: 100%;

    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    padding: 12px;
    border: transparent;
    border-radius: 10px;
    color: #f2f3f5;
    cursor: pointer;
    font: inherit;
    text-align: left;
}

.chat-button:hover{
    background: #252830;
}

.chat-button--active{
    background: #20232a;
}

.chat-button__title{
    font-size: 16px;
}

.chat-button__subtitle{
    color: #858c98;
    font-size: 12px;
}
</style>