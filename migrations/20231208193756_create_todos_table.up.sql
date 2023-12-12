CREATE TABLE todos
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL
);

INSERT INTO todos (title, description) VALUES ('First todo', 'This is the first todo');
INSERT INTO todos (title, description) VALUES ('Second todo', 'This is the second todo');
INSERT INTO todos (title, description) VALUES ('Third todo', 'This is the third todo');
