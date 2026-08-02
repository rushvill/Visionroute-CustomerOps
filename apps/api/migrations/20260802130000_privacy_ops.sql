-- Privacy consent on signup + data-subject request inbox (GDPR / PH DPA aligned ops)

ALTER TABLE signup_requests
    ADD COLUMN privacy_accepted_at TIMESTAMPTZ NULL,
    ADD COLUMN privacy_notice_version VARCHAR(32) NULL;

CREATE TYPE privacy_request_type AS ENUM (
    'access',
    'rectification',
    'erasure',
    'restriction',
    'portability',
    'objection',
    'other'
);

CREATE TYPE privacy_request_status AS ENUM (
    'received',
    'in_progress',
    'completed',
    'rejected'
);

CREATE TABLE privacy_requests (
    id UUID PRIMARY KEY,
    account_id UUID NULL REFERENCES accounts (id) ON DELETE SET NULL,
    requester_name VARCHAR(200) NULL,
    requester_email VARCHAR(255) NOT NULL,
    request_type privacy_request_type NOT NULL DEFAULT 'other',
    details TEXT NULL,
    status privacy_request_status NOT NULL DEFAULT 'received',
    handled_by UUID NULL REFERENCES users (id) ON DELETE SET NULL,
    handled_at TIMESTAMPTZ NULL,
    resolution_notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX privacy_requests_status_idx ON privacy_requests (status);
CREATE INDEX privacy_requests_email_idx ON privacy_requests (requester_email);
CREATE INDEX privacy_requests_created_at_idx ON privacy_requests (created_at);
