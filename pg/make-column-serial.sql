-- Make a column serial
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE users (
    id int,
    name text
);

CREATE SEQUENCE users_id_seq;
ALTER TABLE users ALTER COLUMN id SET DEFAULT nextval('users_id_seq');
ALTER SEQUENCE users_id_seq OWNED BY users.id;

INSERT INTO users (name) VALUES ('Alice'), ('Bob');

SELECT * FROM users;

ROLLBACK;
