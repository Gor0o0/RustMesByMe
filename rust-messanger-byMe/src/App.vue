<script setup lang="ts">

// Импорт 2 функций из vue
// onMounted() - запускает код после появления компонента
// ref - создаёт быстрые перемещения
import {onMounted, ref} from "vue";

// Мои импорты
import type { Message } from "./types/message.ts";
import Database from "@tauri-apps/plugin-sql";
import AppHeader from "./components/AppHeader.vue";
import MessageList from "./components/MessageList.vue";
import MessageComposer from "./components/MessageComposer.vue";

// Создаём структуру одного сообщения

// список сообщений который vue отображает в диалоге на экране
const messages = ref<Message[]>([]);

// статус подключения к бд
const status = ref("Connection...")

// Здесь будет подключение к бд (честно) но пока тут null
let db: Database | null = null;

// Асинхронная функция загрузки сообщений из sql
async function loadMessages(){
  // Если база ещё не подключена прерываем выполнение
  if (!db) return;

  // Читаем данные из таблицы messages
  messages.value = await db.select<Message[]>(
      "SELECT id, author, body, created_at FROM messages ORDER BY id ASC",

  );
}

// Функция отправки нового сообщения
async function sendMessage(body: string){
  // Если база ещё не подключена прерываем выполнение
  if (!db) return;

  await db.execute(
      "INSERT INTO messages (author, body) VALUES ($1, $2)",
      ["You", body]
  );

  await loadMessages();
}



// VUE ыполнит код ниже когда интерфейс программы уже загрузится
onMounted(async()=>{
  try{
    // Открываем бд
    db = await Database.load("sqlite:messanger.db");

    // загружаем из базы старые сообщения
    await loadMessages();

    // Показываем успешное состояние
    status.value = "История сохраняется локально";
  }catch (error){
    console.error(error);
    console.log(error);

    status.value = "Ошибка подключения к базе";
  }
});

</script>

<template>
  <main class="app">
    <AppHeader :status="status"/>

    <section class="chat">
      <div class="chat-info">
        <h2>First chat</h2>

        <p>Second local messager</p>
      </div>

      <MessageList :messages="messages"/>
      <!-- потому-что событие -->
      <MessageComposer @send="sendMessage"/>
    </section>

  </main>
</template>

<style scoped>
/* Все элементы будут */
:global(*){
  box-sizing: border-box;
}

:global(html){
  background: #111318;
  color-scheme: dark;
}

:global(body){
  margin: 0;

  font-family:
  Inter,
  system-ui,
  -apple-system,
  BlinkMacSystemFont,
  "Segoe UI",
  sans-serif;

  color: #f2f3f5;

  background: #111318;
}

.app{
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.chat{
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.chat-info{
  padding: 20px 24px;
  border-bottom: 1px solid #252830;
}

.chat-info h2{
  margin: 0;
  font-size: 16px;
}

.chat-info p{
  margin: 5px 0 0;
  color: #858c98;
  font-size: 13px;
}


</style>