CREATE TABLE vehicles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL,
    reg_number TEXT NOT NULL,
    imei TEXT,
    is_available BOOLEAN DEFAULT TRUE
);
