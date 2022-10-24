-- This file should undo anything in `up.sql`
DELETE FROM users WHERE id IN ('abcd', 'abce');
