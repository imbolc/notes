-- Many to many relationship
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE author (
    id serial PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE book (
    id serial PRIMARY KEY,
    title text NOT NULL
);

CREATE TABLE author_book (
    author_id INT REFERENCES author(id),
    book_id INT REFERENCES book(id),
    PRIMARY KEY (author_id, book_id)
);

INSERT INTO author (name) VALUES 
    ('Carol Nichols'),
    ('Eliezer Yudkowsky'),
    ('Marshall Rosenberg'),
    ('Donald Hoffman'),
    ('Steve Klabnik');
INSERT INTO book (title) VALUES 
    ('Harry Potter and the Methods of Rationality'),
    ('Nonviolent Communication'),
    ('Rationality: From AI to Zombies'),
    ('The Case Against Reality'),
    ('The Rust Programming Language');

-- Notice that we forgot to add `Donald Hoffman` as an author of `The Case Against Reality`
INSERT INTO author_book (author_id, book_id) VALUES
    (1, 5),  -- Carol Nichols co-authored 'The Rust Programming Language'
    (2, 1),  -- Eliezer Yudkowsky wrote 'Harry Potter and the Methods of Rationality'
    (2, 3),  -- Eliezer Yudkowsky wrote 'Rationality: From AI to Zombies'
    (3, 2),  -- Marshall Rosenberg wrote 'Nonviolent Communication: A Language of Life'
    (5, 5);  -- Steve Klabnik co-authored 'The Rust Programming Language'

-- Authors with their books
SELECT
    a.name,
    -- without `remove_array` we would get {NULL} instead of an empty array
    array_remove(array_agg(b.title), NULL) as books
FROM author a
LEFT JOIN author_book ab ON a.id = ab.author_id
LEFT JOIN book b ON ab.book_id = b.id
GROUP BY a.name
ORDER BY a.name;

-- Books with their authors
SELECT
    b.title,
    array_remove(array_agg(a.name), NULL) as authors
FROM book b
LEFT JOIN author_book ab ON b.id = ab.book_id
LEFT JOIN author a ON ab.author_id = a.id
GROUP BY b.title
ORDER BY b.title;

ROLLBACK;
