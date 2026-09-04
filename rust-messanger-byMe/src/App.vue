<script setup lang="ts">

// Импорт 2 функций из vue
// onMounted() - запускает код после появления компонента
// ref - создаёт быстрые перемещения
import {onMounted, ref} from "vue";

import Database from "@tauri-apps/plugin-sql";

// Создаём структуру одного сообщения


// Текст который пользователь вводит
const draft = ref("");

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
async function sendMessage(){
  // Взять введённый пользователем текст и убрать проблемы по краям
  const body = draft.value.trim();

  if (!body) return;
  if (!db) return;

  // Если все проверки прошли заносим сообщение в базу
  await db.execute(
      "INSERT INTO messages (author, body) VALUES ($1, $2)",
      ["You", body],
  );

  // После отправки очищаем поле ввода
  draft.value = "";

  // Обновляем историю сообщений в чате
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
    <header class="header">
      <div>
        <h1>Rust-Messenger(byMe)</h1>

        <p>{{status}}</p>
      </div>

      <span class="badge">
        Local
      </span>
    </header>

    <section class="chat">
      <div class="chat-info">
        <h2>Первый чат</h2>

        <p>Первый локальной месседжер</p>
      </div>
      <div class="messanges">
        <div v-if="messages.length === 0" class="empty">
          <!-- Данный div будет отображаться когда сообщений нет -->
          <strong> Here empty for now </strong>
          <span> Enter your first message </span>
        </div>
        <!== Vue Создаёт article для каждого сообщения из базы -->
        <article v-for="message in messages" :key="message.id" class="message">
          <p>
            {{message.body}}
          </p>
          <footer>
            <span>
              {{message.author}}
            </span>
            <span>
              {{message.created_at}}
            </span>
          </footer>
        </article>
      </div>
      <form class="composer" @submit.prevent="sendMessage">
        <input
          v-model="draft"
          type="text"
          placeholder="Write something more"
          autocomplete="off"
        />
        <button type="submit">Send</button>
      </form>
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

.header{
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 24px;
  border-bottom: 1px solid #292c34;
  background: #17191f;
}

.header h1 {
  margin: 0;
  font-size: 18px;
}

.header p{
  margin: 4px 0 0;
  color: #8f96a3;
}

.badge{
  padding: 6px 10px;
  border: 1px solid #343842;
  border-bottom: 6px;
  color: #afb5c0;
  background: #20232a;
  font-size: 12px;
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

.messages{
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 24px;
}

.empty{
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: center;
  color: #858c98;
}

.message{
  align-self: flex-end;
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
  background: #386be0;
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

.composer{
  display: flex;
  gap: 10px;
  padding: 15px 20px;
  border-top: 1px solid #252830;
  background: #17191f;
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
  background: #386be0;
  font: inherit;
  font-weight: 600;

}

</style>