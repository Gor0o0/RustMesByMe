<script setup lang="ts">

import { ref, watchEffect } from "vue";
import { getFileUrl } from "../types/file.ts";
import type { Message } from "../types/message.ts";

const props = defineProps<{
  message: Message;
  isOwn: boolean;
}>();

const resolvedImageSrc = ref("");

async function resolveImage() {
  if (props.message.type !== "image" || !props.message.attachment) {
    resolvedImageSrc.value = "";
    return;
  }
  resolvedImageSrc.value = await getFileUrl(props.message.attachment);
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
    <p
      v-if="message.type === 'text' && message.body"
    >
      {{message.body}}
    </p>

    <img
        v-if="message.type === 'image' && message.attachment && resolvedImageSrc"
        class="message-image"
        :src="resolvedImageSrc"
        alt="Изображение"
    />
    <footer>
            <span>
              {{ message.author}}
            </span>
      <span>
              |
            </span>
      <span>
              {{message.created_at}}
            </span>
    </footer>
  </article>
</template>

<style scoped>

.message-image{
  max-width: 300px;
  max-height: 300px;
  border-radius: 12px;
  object-fit: cover;
  margin-bottom: 8px;
  display: block;
  background: #fff;
}

.message{
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
}
.message--own{
  align-self: flex-end;
  background: #386be0;
}
.message--other{
  align-self: flex-start;
  background: #252830;
}

.message p{
  margin: 0;
  line-height: 1.45;
  overflow-wrap: anywhere;
}

.message footer{
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 6px;
  color: #b5bbc7;
  font-size: 10px;
}

</style>
