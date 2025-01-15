CREATE TABLE IF NOT EXISTS devices(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS statuses(
    time timestamptz,
    device_id uuid,

    is_open boolean NOT NULL,

    PRIMARY KEY(time, device_id)
);

CREATE TABLE IF NOT EXISTS moistures(
    time timestamptz,
    device_id uuid,

    moisture real NOT NULL,

    PRIMARY KEY(time, device_id)
);

CREATE TABLE IF NOT EXISTS trash_levels(
    time timestamptz,
    device_id uuid,

    trash_level real NOT NULL,

    PRIMARY KEY(time, device_id)
);
