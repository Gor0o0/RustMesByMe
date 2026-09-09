<script setup lang="ts">

// функция которая позволяет импортировать данные из одного файла в другой
import { ref } from "vue";
import EmojiPickme from "./EmojiPickme.vue";

// defineEmits - сообщает vue какие из событий компонент имеет право рассылать
const emit = defineEmits<{
    send: [body: string];
}>();

// Текст который пользователь вводит
const draft = ref("");

function giveEmoji(emoji: string) {
    draft.value += emoji;
}

function submitMessage() {
    // Взять введённый пользователем текст и убрать проблемы по краям
    const body = draft.value.trim();

    if (!body) return;

    emit("send", body);

    // После отправки очищаем поле ввода
    draft.value = "";
}

</script>

<template>

    <form class="composer" @submit.prevent="submitMessage">
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
  flex-shrink: 0; /* - */
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

</style>
