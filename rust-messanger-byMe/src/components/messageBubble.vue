<script setup lang="ts">

import { ref, watchEffect } from "vue";
import type { Message } from "../types/message.ts";
import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir, join, resolveResource } from "@tauri-apps/api/path";

const props = defineProps < {
    message: Message;
    isOwn: boolean;
} > ();

const resolvedImageSrc = ref("");

async function resolveImage() {
    if (!props.message.image_path) {
        resolvedImageSrc.value = "";
        return;
    }
    const ip = props.message.image_path;
    if (ip.startsWith("http://") || ip.startsWith("https://") || ip.startsWith("data:") || ip.startsWith("asset://") || ip.startsWith("file://")) {
        resolvedImageSrc.value = ip;
        return;
    }
    if (ip.includes("/") || ip.includes("\\")) {
        try {
            const rp = await resolveResource("../" + ip);
            resolvedImageSrc.value = convertFileSrc(rp);
            return;
        } catch (_e) {
            resolvedImageSrc.value = "../" + ip;
            return;
        }
    }
    try {
        const catalogDir = await join(await appDataDir(), "Catalog");
        const fullPath = await join(catalogDir, ip);
        resolvedImageSrc.value = convertFileSrc(fullPath);
    } catch (_e) {
        resolvedImageSrc.value = `../Catalog/${ip}`;
    }
}

watchEffect(resolveImage);
resolveImage();

</script>

<template>
    <article 
        class="message"
        :class="{
            'message--own': isOwn,
            'message--other': !isOwn,
        }"
    >
        <img
            v-if="message.image_path && resolvedImageSrc && message.body === '[Изображение]'"
            :src="resolvedImageSrc"
            class="message-image"
            alt="Изображение"
        />
        <div v-if="message.body" class="message-body" v-html="message.body"></div>
        <footer>
            <span>
                {{ message.author }}
            </span>
            <span>
                {{ message.created_at }}
            </span>
        </footer>
    </article>
</template>

<style scoped>

.message{
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
}

.message--own{
    align-self: flex-end;
    background: #e6a225;
}

.message--other{
    align-self: flex-start;
    background: #e625a6;
}

.message-body{
  margin: 0;
  line-height: 1.45;
  overflow-wrap: anywhere;
}

.message-body p{
  margin: 0;
}

.message-body img{
  max-width: 100%;
  max-height: 300px;
  border-radius: 8px;
  display: block;
  margin-bottom: 8px;
  object-fit: contain;
  background: #fff;
}

.message footer{
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 6px;
  color: #ccd8f7;
  font-size: 10px;
}

.message-image{
  max-width: 100%;
  max-height: 300px;
  border-radius: 8px;
  margin-bottom: 8px;
  display: block;
  object-fit: contain;
  background: #fff;
}

</style>
