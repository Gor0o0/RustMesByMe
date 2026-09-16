<script setup lang="ts">

import { ref } from "vue";
import EmojiPickme from "./EmojiPickme.vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{
    send: [body: string, imagePath?: string];
}>();

const draft = ref("");

function giveEmoji(emoji: string) {
    draft.value += emoji;
}

function submitMessage() {
    const body = draft.value.trim();
    if (!body) return;
    emit("send", body);
    draft.value = "";
}

async function pickAndSendImage() {
    const selected = await open({
        multiple: false,
        directory: false,
        filters: [{
            name: "Images",
            extensions: ["png", "jpg", "jpeg", "gif", "webp", "bmp"]
        }]
    });

    if (!selected || typeof selected !== "string") return;

    try {
        const savedPath: string = await invoke("save_attachment", { source: selected });
        emit("send", "", savedPath);
    } catch (e) {
        console.error("Не удалось сохранить вложение:", e);
    }
}

</script>

<template>

    <form class="composer" @submit.prevent="submitMessage">
        <button type="button" class="clip-btn" @click="pickAndSendImage" title="Прикрепить картинку">
            📎
        </button>
        <input v-model="draft" type="text" placeholder="Write something finally" autocomplete="off" />
        <EmojiPickme @pick="giveEmoji" />
        <button type="submit">Send</button>
    </form>

</template>

<style scoped>

.composer{
  display: flex;
  gap: 10px;
  padding: 15px 20px;
  border-top: 1px solid #252830;
  background: #17191f;
  flex-shrink: 0;
}

.composer input{
  flex: 1;
  min-width: 0;
  padding: 11px 13px;
  border: 1px solid #343842;
  border-radius: 7px;
  outline: none;
  color: #f2f3f5;
  background: #20232a;
  font: inherit;
}

.composer input:focus{
  border-color: #4f7fea;
}

.composer button{
  padding: 0 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: white;
  background: #c77b18;
  font: inherit;
  font-weight: 600;
}

.clip-btn{
  padding: 0 12px;
  border: 1px solid #343842;
  border-radius: 7px;
  cursor: pointer;
  color: #f2f3f5;
  background: #20232a;
  font-size: 18px;
  transition: border-color 0.15s, background 0.15s;
}

.clip-btn:hover{
  border-color: #4f7fea;
  background: #2a2d35;
}

</style>
