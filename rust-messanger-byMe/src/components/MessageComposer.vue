<script setup lang="ts">

import { ref } from "vue";
import EmojiPickme from "./EmojiPickme.vue";
import { open } from "@tauri-apps/plugin-dialog";
import { readFile, writeFile, mkdir, exists } from "@tauri-apps/plugin-fs";
import { join, appDataDir, resolve } from "@tauri-apps/api/path";

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

function uint8ToBase64(bytes: Uint8Array): string {
    let binary = "";
    const chunk = 0x8000;
    for (let i = 0; i < bytes.length; i += chunk) {
        binary += String.fromCharCode.apply(
            null,
            Array.from(bytes.subarray(i, i + chunk)) as number[]
        );
    }
    return btoa(binary);
}

function detectMime(ext: string): string {
    switch (ext.toLowerCase()) {
        case "png": return "image/png";
        case "jpg":
        case "jpeg": return "image/jpeg";
        case "gif": return "image/gif";
        case "webp": return "image/webp";
        case "bmp": return "image/bmp";
        default: return "image/png";
    }
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

    const pathParts = selected.split(/[\\/]/);
    const originalName = pathParts[pathParts.length - 1] || "image";
    const dotIdx = originalName.lastIndexOf(".");
    let cleanName = originalName;
    let ext = "png";
    if (dotIdx > 0) {
        cleanName = originalName.substring(0, dotIdx);
        ext = originalName.substring(dotIdx + 1);
    }
    const timestamp = Date.now();
    const newFileName = `${timestamp}_${cleanName}.${ext}`;

    const fileBytes = await readFile(selected);
    const mime = detectMime(ext);
    const base64 = uint8ToBase64(new Uint8Array(fileBytes.buffer, fileBytes.byteOffset, fileBytes.byteLength));
    const dataUrl = `data:${mime};base64,${base64}`;

    try {
        const catalogDir = await join(await appDataDir(), "Catalog");
        if (!(await exists(catalogDir))) {
            await mkdir(catalogDir, { recursive: true });
        }
        const destPath = await join(catalogDir, newFileName);
        await writeFile(destPath, fileBytes);
    } catch (e) {
        console.warn("Не удалось сохранить в appDataDir Catalog:", e);
    }

    try {
        const cwd = await resolve(".");
        const srcCatalog = await join(cwd, "src", "Catalog");
        if (!(await exists(srcCatalog))) {
            await mkdir(srcCatalog, { recursive: true });
        }
        const srcDestPath = await join(srcCatalog, newFileName);
        await writeFile(srcDestPath, fileBytes);
    } catch (e) {
        console.warn("Не удалось сохранить в src/Catalog:", e);
    }

    const imgHtml = `<img src="${dataUrl}" alt="Изображение" style="max-width:100%;max-height:300px;border-radius:8px;display:block;" />`;
    emit("send", imgHtml, dataUrl);
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
