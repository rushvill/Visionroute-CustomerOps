-- Phase 4: core CRM domain

CREATE TYPE account_status AS ENUM ('pending', 'active', 'suspended', 'churned');
CREATE TYPE signup_status AS ENUM ('new', 'reviewing', 'approved', 'rejected');
CREATE TYPE device_status AS ENUM ('pending_install', 'active', 'inactive', 'retired');
CREATE TYPE sim_status AS ENUM ('inventory', 'assigned', 'active', 'suspended', 'exhausted', 'retired');
CREATE TYPE sim_carrier AS ENUM ('smart', 'globe', 'tnt', 'other');
CREATE TYPE subscription_status AS ENUM ('trial', 'active', 'past_due', 'paused', 'cancelled', 'expired');
CREATE TYPE coverage_policy AS ENUM ('shouldered_by_us', 'customer_paid', 'undecided');
CREATE TYPE ticket_status AS ENUM ('open', 'in_progress', 'waiting_customer', 'resolved', 'closed');
CREATE TYPE ticket_priority AS ENUM ('p1', 'p2', 'p3', 'p4');
CREATE TYPE ticket_category AS ENUM ('device', 'sim_data', 'billing', 'login', 'install', 'other');

CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    account_code VARCHAR(32) NOT NULL,
    company_name VARCHAR(200) NOT NULL,
    display_name VARCHAR(200) NULL,
    status account_status NOT NULL DEFAULT 'pending',
    industry VARCHAR(100) NULL,
    tax_id VARCHAR(64) NULL,
    billing_email VARCHAR(255) NULL,
    operations_email VARCHAR(255) NULL,
    phone VARCHAR(40) NULL,
    address_line1 VARCHAR(255) NULL,
    address_line2 VARCHAR(255) NULL,
    city VARCHAR(100) NULL,
    province VARCHAR(100) NULL,
    postal_code VARCHAR(20) NULL,
    country VARCHAR(2) NOT NULL DEFAULT 'PH',
    notes TEXT NULL,
    tracksolid_account_ref VARCHAR(120) NULL,
    source VARCHAR(64) NULL,
    approved_at TIMESTAMPTZ NULL,
    approved_by UUID NULL REFERENCES users (id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NULL,
    updated_by UUID NULL,
    CONSTRAINT accounts_account_code_unique UNIQUE (account_code)
);

CREATE INDEX accounts_status_idx ON accounts (status);
CREATE INDEX accounts_company_name_idx ON accounts (company_name);

-- Clear orphan account refs from Phase 3 seed before enforcing FK.
UPDATE users
SET account_id = NULL
WHERE account_id IS NOT NULL
  AND NOT EXISTS (SELECT 1 FROM accounts a WHERE a.id = users.account_id);

ALTER TABLE users
    ADD CONSTRAINT users_account_id_fkey
    FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE SET NULL;

CREATE TABLE signup_requests (
    id UUID PRIMARY KEY,
    status signup_status NOT NULL DEFAULT 'new',
    full_name VARCHAR(200) NOT NULL,
    company_name VARCHAR(200) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(40) NULL,
    requested_username VARCHAR(64) NULL,
    estimated_devices INT NULL,
    message TEXT NULL,
    preferred_contact VARCHAR(32) NULL,
    ip_hash VARCHAR(128) NULL,
    user_agent_hash VARCHAR(128) NULL,
    reviewed_by UUID NULL REFERENCES users (id) ON DELETE SET NULL,
    reviewed_at TIMESTAMPTZ NULL,
    rejection_reason TEXT NULL,
    converted_account_id UUID NULL REFERENCES accounts (id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX signup_requests_status_idx ON signup_requests (status);
CREATE INDEX signup_requests_email_idx ON signup_requests (email);
CREATE INDEX signup_requests_created_at_idx ON signup_requests (created_at);

CREATE TABLE devices (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    name VARCHAR(120) NOT NULL,
    plate_number VARCHAR(32) NULL,
    imei VARCHAR(32) NULL,
    provider VARCHAR(32) NOT NULL DEFAULT 'tracksolid',
    provider_device_id VARCHAR(64) NULL,
    provider_account_ref VARCHAR(120) NULL,
    status device_status NOT NULL DEFAULT 'pending_install',
    install_date DATE NULL,
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX devices_account_id_idx ON devices (account_id);
CREATE INDEX devices_status_idx ON devices (status);

CREATE TABLE sim_cards (
    id UUID PRIMARY KEY,
    carrier sim_carrier NOT NULL DEFAULT 'smart',
    iccid VARCHAR(32) NULL,
    msisdn VARCHAR(20) NULL,
    sim_label VARCHAR(64) NULL,
    status sim_status NOT NULL DEFAULT 'inventory',
    purchase_date DATE NULL,
    purchase_cost_cents INT NULL,
    data_plan_label VARCHAR(120) NULL,
    account_id UUID NULL REFERENCES accounts (id) ON DELETE SET NULL,
    device_id UUID NULL REFERENCES devices (id) ON DELETE SET NULL,
    activated_at TIMESTAMPTZ NULL,
    last_status_check_at TIMESTAMPTZ NULL,
    data_exhausted_at TIMESTAMPTZ NULL,
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT sim_cards_iccid_unique UNIQUE (iccid),
    CONSTRAINT sim_cards_msisdn_unique UNIQUE (msisdn),
    CONSTRAINT sim_cards_identity_check CHECK (iccid IS NOT NULL OR msisdn IS NOT NULL)
);

CREATE INDEX sim_cards_status_idx ON sim_cards (status);
CREATE INDEX sim_cards_account_id_idx ON sim_cards (account_id);
CREATE INDEX sim_cards_device_id_idx ON sim_cards (device_id);

CREATE TABLE plans (
    id UUID PRIMARY KEY,
    code VARCHAR(32) NOT NULL,
    name VARCHAR(120) NOT NULL,
    description TEXT NULL,
    price_cents INT NOT NULL DEFAULT 0,
    currency CHAR(3) NOT NULL DEFAULT 'PHP',
    billing_cycle VARCHAR(16) NOT NULL DEFAULT 'yearly',
    device_limit INT NOT NULL DEFAULT 1,
    included_sims INT NOT NULL DEFAULT 1,
    includes_data_months INT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT plans_code_unique UNIQUE (code)
);

CREATE TABLE subscriptions (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    plan_id UUID NOT NULL REFERENCES plans (id),
    promo_id UUID NULL,
    status subscription_status NOT NULL DEFAULT 'active',
    coverage_policy coverage_policy NOT NULL DEFAULT 'shouldered_by_us',
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NULL,
    data_coverage_starts_at DATE NULL,
    data_coverage_ends_at DATE NULL,
    continue_shouldering BOOLEAN NULL,
    renews_at TIMESTAMPTZ NULL,
    amount_cents INT NULL,
    currency CHAR(3) NOT NULL DEFAULT 'PHP',
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX subscriptions_account_id_idx ON subscriptions (account_id);
CREATE INDEX subscriptions_status_idx ON subscriptions (status);
CREATE INDEX subscriptions_coverage_ends_idx ON subscriptions (data_coverage_ends_at);

CREATE TABLE subscription_sims (
    id UUID PRIMARY KEY,
    subscription_id UUID NOT NULL REFERENCES subscriptions (id) ON DELETE CASCADE,
    sim_card_id UUID NOT NULL REFERENCES sim_cards (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT subscription_sims_unique UNIQUE (subscription_id, sim_card_id)
);

CREATE TABLE tickets (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    number VARCHAR(32) NOT NULL,
    created_by_user_id UUID NOT NULL REFERENCES users (id),
    assigned_to_user_id UUID NULL REFERENCES users (id) ON DELETE SET NULL,
    device_id UUID NULL REFERENCES devices (id) ON DELETE SET NULL,
    sim_card_id UUID NULL REFERENCES sim_cards (id) ON DELETE SET NULL,
    subject VARCHAR(200) NOT NULL,
    description TEXT NULL,
    status ticket_status NOT NULL DEFAULT 'open',
    priority ticket_priority NOT NULL DEFAULT 'p2',
    category ticket_category NOT NULL DEFAULT 'other',
    resolved_at TIMESTAMPTZ NULL,
    closed_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT tickets_number_unique UNIQUE (number)
);

CREATE INDEX tickets_account_id_idx ON tickets (account_id);
CREATE INDEX tickets_status_idx ON tickets (status);

CREATE TABLE ticket_comments (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tickets (id) ON DELETE CASCADE,
    author_user_id UUID NOT NULL REFERENCES users (id),
    body TEXT NOT NULL,
    is_internal BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ticket_comments_ticket_id_idx ON ticket_comments (ticket_id);

CREATE SEQUENCE account_code_seq START 1;
CREATE SEQUENCE ticket_number_seq START 1;
