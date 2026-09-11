// Импорты типов необходимых для migrations
use tauri_plugin_sql::{Migration, MigrationKind};

// Аннотация необходимая Таури для мобильных платформ
// на win она не мешает
#[cfg_attr(mobile, tauri::mobile_entry_point)]

// Главная функция для запуска приложения
pub fn run() {
    // Создание списка миграций
    let migrations = vec![
        // Описание первой миграции
        Migration {
            version: 1,

            description: "create_message_table",

            // Берём SQL запрос из нашего файла
            sql: include_str!("../migrations/0001_initial.sql"),

            // Up - означает что база сдвинется вперёд
            kind: MigrationKind::Up,
        },
    ];

    // Создаём сборщик приложения Tauri
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // Подключаем sql плагин
        .plugin(
            // Сборщик плагинов
            tauri_plugin_sql::Builder::default()
                // Связываем migrations с базой sql
                .add_migrations("sqlite:messanger.db", migrations)
                // Собираем плагины
                .build(),
        )
        // Создаём plugin opener
        .plugin(tauri_plugin_opener::init())
        // Запускаем приложение
        .run(tauri::generate_context!())
        // Если запуск завершился с ошибкой то сообщаем об этом
        .expect("error while running tauri application");
}
