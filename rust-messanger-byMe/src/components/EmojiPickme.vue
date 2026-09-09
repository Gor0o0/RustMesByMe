<script setup lang="ts">

import { ref } from "vue";

const emit = defineEmits<{
    pick: [emoji: string];
}>();

const isOpen = ref(false);

const emojis = [
    "😀", "😂", "😍", "🤞", "🤔",
    "😭", "😡", "👍", "❤️", "🎉",
    "🔥", "💖", "🙏", "👀", "🤷‍♂️"
];

function toggle() {
    isOpen.value = !isOpen.value;
}

function pickmeEmoji(emoji: string) {
    emit("pick", emoji);
    isOpen.value = false;
}

</script>

<template>
    <div class="emoji">

        <button
            type="button"
            class="emoji--button"
            @click="toggle"
        >
            E
        </button>

        <div v-if="isOpen" class="emoji-popup">
            <button
                v-for="e in emojis"
                :key="e"
                type="button"
                class="emoji-popup__item"
                @click="pickmeEmoji(e)"
            >
                {{ e }}
            </button>
        </div>

    </div>
</template>

<style scoped>

.emoji {
    position: relative;
}

.emoji--button {
    padding: 0 14px;
    border: none;
    border-radius: 7px;
    cursor: pointer;
    background: #2a2e38;
    font-size: 18px;
    transition: background 0.15s;
}
.emoji--button:hover {
    background: #343842;
}

.emoji-popup {
    position: absolute;
    left: -100px;
    bottom: 100%;
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 4px;
    padding: 8px;
    background: #382a34;
    border: 1px solid #343842;
    border-radius: 8px;
}

.emoji-popup__item {
    width: 36px;
    height: 36px;
    border: none;
    border-radius: 6px;
    font-size: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
}
.emoji-popup__item:hover {
    background: #2f3340;
}
</style>