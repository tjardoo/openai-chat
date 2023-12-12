CREATE TABLE models
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owned_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL
);

INSERT INTO models (name, owned_by, created_at) VALUES ('First model', 'user1', NOW());
INSERT INTO models (name, owned_by, created_at) VALUES ('Second model', 'user2', NOW());
INSERT INTO models (name, owned_by, created_at) VALUES ('Third model', 'user3', NOW());
