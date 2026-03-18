-- Add tag history fields for soft deletes
ALTER TABLE player_tags
    ADD COLUMN removed_by BIGINT,
    ADD COLUMN removed_on TIMESTAMPTZ;

CREATE INDEX idx_player_tags_active ON player_tags(player_id) WHERE removed_on IS NULL;
CREATE INDEX idx_player_tags_removed_by ON player_tags(removed_by) WHERE removed_by IS NOT NULL;
