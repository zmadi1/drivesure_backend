-- Your SQL goes here
CREATE TABLE driver_verifications (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id           UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    licence_number    VARCHAR(30) UNIQUE NOT NULL,
    licence_front_url TEXT NOT NULL,
    selfie_url        TEXT NOT NULL,
    face_match_score  REAL,
    rtmc_verified     BOOLEAN DEFAULT FALSE,
    created_at        TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_driver_verifications_licence ON driver_verifications(licence_number);
