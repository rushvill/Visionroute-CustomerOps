-- Small-business billing: customer invoices/payments + SIM data costs

CREATE TYPE invoice_status AS ENUM (
    'draft',
    'sent',
    'partial',
    'paid',
    'overdue',
    'cancelled'
);

CREATE TYPE payment_method AS ENUM (
    'cash',
    'bank_transfer',
    'gcash',
    'maya',
    'other'
);

CREATE TABLE customer_invoices (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    number VARCHAR(32) NOT NULL,
    description VARCHAR(255) NOT NULL,
    amount_cents INT NOT NULL CHECK (amount_cents >= 0),
    currency CHAR(3) NOT NULL DEFAULT 'PHP',
    status invoice_status NOT NULL DEFAULT 'sent',
    issued_at DATE NOT NULL DEFAULT CURRENT_DATE,
    due_date DATE NULL,
    paid_at TIMESTAMPTZ NULL,
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT customer_invoices_number_unique UNIQUE (number)
);

CREATE INDEX customer_invoices_account_id_idx ON customer_invoices (account_id);
CREATE INDEX customer_invoices_status_idx ON customer_invoices (status);
CREATE INDEX customer_invoices_due_date_idx ON customer_invoices (due_date);

CREATE TABLE customer_payments (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    invoice_id UUID NULL REFERENCES customer_invoices (id) ON DELETE SET NULL,
    amount_cents INT NOT NULL CHECK (amount_cents > 0),
    currency CHAR(3) NOT NULL DEFAULT 'PHP',
    method payment_method NOT NULL DEFAULT 'cash',
    paid_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    reference VARCHAR(120) NULL,
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX customer_payments_account_id_idx ON customer_payments (account_id);
CREATE INDEX customer_payments_invoice_id_idx ON customer_payments (invoice_id);
CREATE INDEX customer_payments_paid_at_idx ON customer_payments (paid_at);

CREATE TABLE sim_data_costs (
    id UUID PRIMARY KEY,
    account_id UUID NULL REFERENCES accounts (id) ON DELETE SET NULL,
    sim_card_id UUID NULL REFERENCES sim_cards (id) ON DELETE SET NULL,
    amount_cents INT NOT NULL CHECK (amount_cents >= 0),
    currency CHAR(3) NOT NULL DEFAULT 'PHP',
    carrier sim_carrier NULL,
    description VARCHAR(255) NOT NULL,
    period_start DATE NULL,
    period_end DATE NULL,
    paid_at DATE NOT NULL DEFAULT CURRENT_DATE,
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX sim_data_costs_account_id_idx ON sim_data_costs (account_id);
CREATE INDEX sim_data_costs_sim_card_id_idx ON sim_data_costs (sim_card_id);
CREATE INDEX sim_data_costs_paid_at_idx ON sim_data_costs (paid_at);

CREATE SEQUENCE invoice_number_seq START 1;
