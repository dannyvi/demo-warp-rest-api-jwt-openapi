CREATE TABLE users (
    id VARCHAR PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    mobile VARCHAR(11) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE OR REPLACE FUNCTION update_time_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_time BEFORE UPDATE
    ON users FOR EACH ROW EXECUTE PROCEDURE 
    update_time_column();
