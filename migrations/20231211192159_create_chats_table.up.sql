CREATE TABLE chats
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    model_id INT UNSIGNED NOT NULL,
    external_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (model_id) REFERENCES models(id) ON DELETE CASCADE
);

INSERT INTO chats (title, model_id, external_id) VALUES ('Chat 1', 1, 'chat1');
INSERT INTO chats (title, model_id, external_id) VALUES ('Chat 2', 2, 'chat2');
INSERT INTO chats (title, model_id, external_id) VALUES ('Chat 3', 3, 'chat3');
