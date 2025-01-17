CREATE TABLE IF NOT EXISTS devices(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS statuses(
    time timestamptz,
    device_id uuid,

    is_open boolean NOT NULL,
    trash_level real NOT NULL,

    PRIMARY KEY(time, device_id)
);
