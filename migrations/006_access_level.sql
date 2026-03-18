ALTER TABLE members ADD COLUMN access_level SMALLINT NOT NULL DEFAULT 0;

UPDATE members SET access_level = CASE
    WHEN is_admin THEN 3
    WHEN is_mod THEN 2
    WHEN is_private THEN 1
    ELSE 0
END;

ALTER TABLE members
    DROP COLUMN is_admin,
    DROP COLUMN is_mod,
    DROP COLUMN is_beta,
    DROP COLUMN is_private;
