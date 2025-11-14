-- Your SQL goes here
CREATE TABLE vehicles (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plate_number  VARCHAR(20) NOT NULL UNIQUE,
    make          VARCHAR(80) NOT NULL,
    model         VARCHAR(80) NOT NULL,
    year          SMALLINT    CHECK (year >= 1900 AND year <= 2100),
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vehicles_owner ON vehicles(owner_user_id);