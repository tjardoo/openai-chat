CREATE TABLE messages
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    chat_id INT UNSIGNED NOT NULL,
    role VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    used_model VARCHAR(255) NOT NULL,
    prompt_tokens INT,
    completion_tokens INT,
    temperature FLOAT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);
