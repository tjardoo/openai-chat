CREATE TABLE chats
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    open_ai_id VARCHAR(255) NOT NULL,
    model_id INT UNSIGNED NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (model_id) REFERENCES models(id) ON DELETE CASCADE
);

INSERT INTO chats (open_ai_id, model_id, created_at) VALUES ('chat1', 1, NOW());
INSERT INTO chats (open_ai_id, model_id, created_at) VALUES ('chat2', 2, NOW());
INSERT INTO chats (open_ai_id, model_id, created_at) VALUES ('chat3', 3, NOW());
