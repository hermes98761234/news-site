CREATE TABLE IF NOT EXISTS settings (
    key        TEXT NOT NULL PRIMARY KEY,
    value      TEXT NOT NULL DEFAULT ''
);
INSERT OR IGNORE INTO settings(key, value) VALUES
    ('site_name', 'My News Site'),
    ('site_description', ''),
    ('site_url', 'http://localhost:3000');
