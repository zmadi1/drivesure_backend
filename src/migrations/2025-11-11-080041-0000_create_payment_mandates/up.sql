-- Your SQL goes here
CREATE TYPE mandate_status AS ENUM ('active', 'failed', 'completed');

CREATE TABLE payment_mandands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vehicle_id UUID NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id),
    driver_id UUID NOT NULL REFERENCES users(id),
    amount_cents INT NOT NULL,
    retry_count SMALLINT DEFAULT 0,
    status mandate_status DEFAULT 'active',
    due_date DATE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_mandate_status ON payment_mandates(status);
CREATE INDEX idx_mandate_due ON payment_mandates(due_date);
