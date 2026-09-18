// Ключевое слово export
// Разрещает другим файлам экспортить его

export interface Message{
    id: number;
    author: string;
    type: "text" | "image",
    body: string | null;
    attachment: string | null,
    created_at: string;
}
