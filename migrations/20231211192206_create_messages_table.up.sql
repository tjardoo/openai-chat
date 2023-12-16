CREATE TABLE messages
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    chat_id INT UNSIGNED NOT NULL,
    role VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    used_model VARCHAR(255) NOT NULL,
    used_tokens INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);

INSERT INTO messages (chat_id, role, content, used_model, created_at) VALUES (1, 'user', 'test', 'This is the first message', NOW());
INSERT INTO messages (chat_id, role, content, used_model, created_at) VALUES (1, 'assistant', 'test', 'This is the second message', NOW());
INSERT INTO messages (chat_id, role, content, used_model, created_at) VALUES (1, 'user', 'test', 'This is the third message', NOW());

INSERT INTO messages (chat_id, role, content, used_model, created_at) VALUES (2, 'user', 'test', 'This is the first message', NOW());
INSERT INTO messages (chat_id, role, content, used_model, created_at) VALUES (2, 'assistant', 'test', 'This is the second message', NOW());
INSERT INTO messages (chat_id, role, content, used_model, created_at) VALUES (2, 'user', 'test', 'This is the third message', NOW());
