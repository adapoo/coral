-- Convert reviewed_by from single BIGINT to array for storing up to 3 voter IDs
ALTER TABLE player_tags
    ALTER COLUMN reviewed_by TYPE BIGINT[]
    USING CASE WHEN reviewed_by IS NOT NULL THEN ARRAY[reviewed_by] ELSE NULL END;
