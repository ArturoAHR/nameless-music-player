DROP TRIGGER track_updated_at;

CREATE TRIGGER track_updated_at
AFTER UPDATE ON track
FOR EACH ROW
WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE track SET updated_at = unixepoch() WHERE id = NEW.id;
END;

CREATE TABLE tag_group (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()) CHECK (created_at >= 0),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()) CHECK (updated_at >= 0),
    deleted_at INTEGER CHECK (deleted_at IS NULL OR deleted_at >= 0)
);

CREATE UNIQUE INDEX idx_tag_group_name_active
    ON tag_group (name)
    WHERE deleted_at IS NULL;

CREATE TRIGGER tag_group_updated_at
AFTER UPDATE ON tag_group
FOR EACH ROW
WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE tag_group SET updated_at = unixepoch() WHERE id = NEW.id;
END;

CREATE TABLE tag (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    tag_group_id INTEGER NOT NULL REFERENCES tag_group (id),
    created_at INTEGER NOT NULL DEFAULT (unixepoch()) CHECK (created_at >= 0),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()) CHECK (updated_at >= 0),
    deleted_at INTEGER CHECK (deleted_at IS NULL OR deleted_at >= 0)
);

CREATE UNIQUE INDEX idx_tag_group_id_name_active
    ON tag (tag_group_id, name)
    WHERE deleted_at IS NULL;

CREATE TRIGGER tag_updated_at
AFTER UPDATE ON tag
FOR EACH ROW
WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE tag SET updated_at = unixepoch() WHERE id = NEW.id;
END;

CREATE TABLE track_tag (
    id INTEGER PRIMARY KEY,
    track_id INTEGER NOT NULL REFERENCES track (id),
    tag_id INTEGER NOT NULL REFERENCES tag (id),
    created_at INTEGER NOT NULL DEFAULT (unixepoch()) CHECK (created_at >= 0)
);

CREATE UNIQUE INDEX idx_track_tag_tag_id_track_id
    ON track_tag (tag_id, track_id);
CREATE INDEX idx_track_tag_track_id_tag_id
    ON track_tag (track_id, tag_id);
