CREATE TABLE IF NOT EXISTS users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    avatar_path TEXT,
    status TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO users(
    id, username, display_name, status
)
VALUES(
    1,
    'Trippi777',
    'Trippi',
    'Online'
);

INSERT OR IGNORE INTO users(
    id, username, display_name, status
)
VALUES(
    2,
    'Sahur98',
    'Sahur',
    'Online'
);

INSERT OR IGNORE INTO users(
    id, username, display_name, status
)
VALUES(
    3,
    'Abryz99',
    'Ari',
    'Online'
);

INSERT OR IGNORE INTO users(
    username,
    display_name
)
SELECT
    -- Технически username legacy_1 ...2.
    -- CAST превращает число в тек
    'legacy_' || CAST(old_authors.first_nessage_id AS TEXT)

    old_authors.author
FROM (
    SELECT
        MIN(id) AS first_nessage_id,
        author
    FROM messages
    -- Создаём одну группу для каждого имени автора
    GROUP BY author
) AS old_authors

WHERE NOT EXISTS(
    SELECT 1
    FROM users
    WHERE users.display_name = old_authors
);

CREATE TABLE messages_new(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    chat_id INTEGER NOT NULL
        REFERENCES chats(id) --Ссылка на users.id
        ON DELETE CASCADE,  --Связанные с ним сообщения так же будут удалены
    
    author_id INTEGER NOT NULL
        REFERENCES users(id)
        ON DELETE RESTRICT,  --Нельзя удалить пользователя если на него ссылаются сообщения
    
    type TEXT NOT NULL DEFAULT 'text',

    body TEXT,

    attachment TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- проверяет и разрешает только типы из данного списка
    CHECK(
        type IN(
            'text',
            'image'
        )
    )
);

INSERT INTO messages_new(
    id,
    chat_id,
    author_id,
    type,
    body,
    attachment,
    created_at
)

SELECT
    -- Оста
    messages.id,
    messages.chat_id,
    (
        SELECT users.id

        WHERE
            users.display_name = messages.author
        -- Если есть одинаковый display_name берём пользователя с меньшим id
        ORDER BY users,id ASC
        LIMIT 1
    ),
    messages.type,
    messages.body,
    messages.attachment,
    messages.created_at
FROM messages;

DROP TABLE messages;

ALTER TABLE messages_new
RENAME TO messages;

-- Индексы нужны для быстрого поиска по сообщению или автору
CREATE INDEX IF NOT EXISTS
inx_messages_chat_id
ON messages(chat_id);

CREATE INDEX IF NOT EXISTS
inx_messages_author_id
ON messages(author_id);