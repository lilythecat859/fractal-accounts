CREATE TABLE users (
    id   UUID PRIMARY KEY,
    username      TEXT UNIQUE NOT NULL,
    password_hash TEXT        NOT NULL
);

CREATE TABLE ledger_entries (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id       UUID NOT NULL REFERENCES users(id),
    amount_cents  BIGINT NOT NULL,
    currency      CHAR(3) NOT NULL,
    balance_cents BIGINT NOT NULL,
    created_at    TIMESTAMPTZ DEFAULT NOW()
);
