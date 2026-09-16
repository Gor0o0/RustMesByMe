// Импорты типов необходимых для migrations
use tauri_plugin_sql::{Migration, MigrationKind};

// Аннотация необходимая Таури для мобильных платформ
// на win она не мешает
#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command]
fn save_attachment(source: String) -> Result<String, String> {
    let allowed_extensions = ["png", "jpg", "jpeg", "gif", "webp", "bmp"];

    let source_path = std::path::Path::new(&source);

    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| "Не удалось определить расширение файла".to_string())?
        .to_lowercase();

    if !allowed_extensions.contains(&extension.as_str()) {
        return Err(format!(
            "Недопустимое расширение файла: .{}. Разрешены: {}",
            extension,
            allowed_extensions.join(", .")
        ));
    }

    let app_dir = std::env::current_dir()
        .map_err(|e| e.to_string())?;

    let attachment_dir = app_dir.join("attachment");

    std::fs::create_dir_all(&attachment_dir)
        .map_err(|e| e.to_string())?;

    let file_name = format!("image_{}.{}", chrono::Utc::now().timestamp(), extension);

    let destination = attachment_dir.join(&file_name);

    std::fs::copy(
        source, &destination
    )
        .map_err(|e| e.to_string())?;

    Ok(
        format!(
            "attachment/{}",
            file_name
        )
    )
}

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
        Migration {
            version: 2,

            description: "create_chats",

            sql: include_str!("../migrations/0002_chats.sql"),

            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,

            description: "message_attachments",

            sql: include_str!("../migrations/0003_message_attachments.sql"),

            kind: MigrationKind::Up,
        }
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
                .add_migrations("sqlite:messenger.db", migrations)
                // Собираем плагины
                .build(),
        )
        // Создаём plugin opener
        .plugin(tauri_plugin_opener::init())
        // Запускаем приложение
        .invoke_handler(
            tauri::generate_handler![save_attachment]
        )
        .run(tauri::generate_context!())

        // Если запуск завершился с ошибкой то сообщаем об этом
        .expect("error while running tauri application");
}
