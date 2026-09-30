<script setup lang="ts">

import { getFileUrl } from "../types/file.ts";

import type { Message } from "../types/message.ts";

const props = defineProps<{
  message: Message;
  isOwn: boolean;
}>();

const emit = defineEmits<{
  edit: [id: number, body: string];
  delete: [id: number];
}>();

function editMessage(){
  if (props.message.type !== "text") return;

  emit(
      "edit",
      props.message.id,
      props.message.body || ""
  );
}

function deleteMessage(){
  if (!props.isOwn) return;

  emit(
      "delete",
      props.message.id
  );
}

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
      v-if="
        message.type==='text'
      "
    >
      {{message.body}}
    </p>

    <img
        v-if="
          message.type === 'image'
          &&
          message.attachment
        "
        class="message-image"
        :src="
          getFileUrl(
            message.attachment
          )
        "
    />

    <footer>
      <span>
        {{ message.author_name}}
      </span>

      <span>
        |
      </span>

      <span>
        {{message.created_at}}
      </span>
      <button
          v-if="isOwn"
          type="button"
          class="edit-button"
          @click="deleteMessage"
      >
        D
      </button>
      <button
          v-if="message.type === 'text' && isOwn"
          type="button"
          class="edit-button"
          @click="editMessage"
      >
        R
      </button>
    </footer>

  </article>
</template>

<style scoped>

.message-image{
  max-width: 300px;
  max-height: 300px;
  border-radius: 12px;
  object-fit: cover;
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
  align-items: center;
  gap: 5px;
  margin-top: 6px;
  color: #b5bbc7;
  font-size: 10px;
}

.edit-button{
  border: none;
  border-radius: 5px;
  padding: 3px 6px;
  cursor: pointer;
  color: white;
  background: #2d55ad;
  font-size: 10px;
}

.edit-button:hover{
  background: #24458f;
}

</style>