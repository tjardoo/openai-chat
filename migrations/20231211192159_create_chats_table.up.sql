CREATE TABLE chats
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NULL,
    last_used_model VARCHAR(255) NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO chats (title) VALUES ('Chat 1');
INSERT INTO chats (title) VALUES ('Chat 2');
INSERT INTO chats (title) VALUES ('Chat 3');
