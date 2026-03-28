/*
Here is a diagram for creating tables in a database. 
In addition to the tables listed here, there will also be another table in the database, sessions, 
which is created automatically from tower sessions.

Since some kind of basic administrator is needed, 
the basic administrator with the login "Admin" and password "1111" is specified in the scheme.

*/

-- Table for store categories:
CREATE TABLE IF NOT EXISTS categories (
  id BIGSERIAL PRIMARY KEY,
  parent_id BIGINT REFERENCES categories(id) ON DELETE SET NULL,
  name VARCHAR(255) NOT NULL
);

-- Table for store products:
CREATE TABLE IF NOT EXISTS products (
  id BIGSERIAL PRIMARY KEY,
  category_id BIGINT REFERENCES categories(id) ON DELETE SET NULL,
  quantity INT NOT NULL,
  article VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  price NUMERIC(10, 2) NOT NULL
);


-- Table for store admins:
CREATE TABLE IF NOT EXISTS admins (
  id BIGSERIAL PRIMARY KEY,
  login VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL
);


-- Insert base admin into table "admins":
INSERT INTO admins (login, password) VALUES('Admin', '$argon2id$v=19$m=19456,t=2,p=1$9mlFPGin9R5DnMxyyHcBTw$SuysCeL1+1DFdAKWU++tgNqSSci1R1KUg2DG0sjGJQ0');


-- Add indexes into tables:
CREATE INDEX category_id ON categories (id);
CREATE INDEX category_parent_id ON categories (parent_id);
CREATE INDEX products_article ON products (article);
CREATE INDEX admins_login ON admins (login);

-- Создаём функцию, которая будет отрабатывать для триггера:

CREATE OR REPLACE FUNCTION prevent_admins_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Удаление записей из таблицы admins запрещено';
    RETURN NULL;  
END;
$$ LANGUAGE plpgsql;

-- Создаём триггер:

DROP TRIGGER IF EXISTS admins_delete_protect ON admins;
CREATE TRIGGER admins_delete_protect
    BEFORE DELETE ON admins
    FOR EACH ROW 
    EXECUTE FUNCTION prevent_admins_delete();

