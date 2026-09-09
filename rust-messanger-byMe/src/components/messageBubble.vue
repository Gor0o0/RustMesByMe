<script setup lang="ts">

import type { Message } from "../types/message.ts";

defineProps < {
    message: Message;
    isOwn: boolean;
} > ();

</script>

<template>
    <article 
        class="message"
        :class="{
            'message--own': isOwn,
            'message--other': !isOwn,
        }"
    >
        <p>
            {{ message.body }}
        </p>
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
  color: #ccd8f7;
  font-size: 10px;
}

</style>
