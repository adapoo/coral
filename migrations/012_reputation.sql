-- Add review tracking columns to members
ALTER TABLE members ADD COLUMN accepted_tags BIGINT NOT NULL DEFAULT 0;
ALTER TABLE members ADD COLUMN rejected_tags BIGINT NOT NULL DEFAULT 0;

-- Track who approved a tag
ALTER TABLE player_tags ADD COLUMN reviewed_by BIGINT;

-- Track accurate community review verdicts
ALTER TABLE members ADD COLUMN accurate_verdicts BIGINT NOT NULL DEFAULT 0;
