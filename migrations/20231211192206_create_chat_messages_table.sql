CREATE TABLE chat_messages
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    chat_id INT UNSIGNED NOT NULL,
    role VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);

INSERT INTO chat_messages (chat_id, role, description, created_at) VALUES (1, 'user', 'This is the first message', NOW());
INSERT INTO chat_messages (chat_id, role, description, created_at) VALUES (1, 'assistant', 'This is the second message', NOW());
INSERT INTO chat_messages (chat_id, role, description, created_at) VALUES (1, 'user', 'This is the third message', NOW());

INSERT INTO chat_messages (chat_id, role, description, created_at) VALUES (2, 'user', 'This is the first message', NOW());
INSERT INTO chat_messages (chat_id, role, description, created_at) VALUES (2, 'assistant', 'This is the second message', NOW());
INSERT INTO chat_messages (chat_id, role, description, created_at) VALUES (2, 'user', 'This is the third message', NOW());
