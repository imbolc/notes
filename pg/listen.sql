-- Postgres notification on table changes

-- We should commit triggers before they work, so transaction based sandbox won't work here

-- Create a table about changes of which we'll be notified
CREATE TABLE sandbox.mytable (
    id serial PRIMARY KEY
);

-- Create a function to send notifications
CREATE FUNCTION sandbox.notify_mytable_changes()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify('mytable_changed', '');
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to run the function on the table changes
-- By default thiggers are created in the table schema
CREATE TRIGGER mytable_changed
AFTER INSERT OR UPDATE
ON sandbox.mytable
FOR EACH ROW
EXECUTE PROCEDURE sandbox.notify_mytable_changes();

-- We'll get notified on any insert or update of the table
LISTEN mytable_changed;

-- Try to insert somenthing
INSERT INTO sandbox.mytable DEFAULT VALUES;

--- Cleanup
DROP TRIGGER mytable_changed ON sandbox.mytable;
DROP FUNCTION sandbox.notify_mytable_changes;
DROP TABLE sandbox.mytable;
