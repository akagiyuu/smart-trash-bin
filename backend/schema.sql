CREATE TABLE IF NOT EXISTS statuses(
    time datetime PRIMARY KEY,
    is_open boolean NOT NULL
);

CREATE TABLE IF NOT EXISTS moistures(
    time datetime PRIMARY KEY,
    moisture real NOT NULL
);

CREATE TABLE IF NOT EXISTS trash_levels(
    time datetime PRIMARY KEY,
    trash_level real NOT NULL
);
