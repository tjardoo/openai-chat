CREATE TABLE messages
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    chat_id INT UNSIGNED NOT NULL,
    role VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    used_model VARCHAR(255) NOT NULL,
    prompt_tokens INT,
    completion_tokens INT,
    last_used_model VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);

INSERT INTO messages (chat_id, role, content, used_model) VALUES (1, 'user', 'This is the first message', 'test');
INSERT INTO messages (chat_id, role, content, used_model) VALUES (1, 'assistant', 'This is the second message', 'test');
INSERT INTO messages (chat_id, role, content, used_model) VALUES (1, 'user', 'This is the third message', 'test');

INSERT INTO messages (chat_id, role, content, used_model) VALUES (2, 'user', 'This is the first message', 'test');
INSERT INTO messages (chat_id, role, content, used_model) VALUES (2, 'assistant', 'This is the second message', 'test');
INSERT INTO messages (chat_id, role, content, used_model) VALUES (2, 'user', 'This is the third message', 'test');
