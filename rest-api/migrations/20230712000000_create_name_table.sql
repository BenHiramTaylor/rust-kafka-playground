-- 20230712000000_create_name_table.sql
CREATE TABLE Name (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    processed_on TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Add an index on the name column for faster lookups
CREATE INDEX idx_name_name ON Name(name);