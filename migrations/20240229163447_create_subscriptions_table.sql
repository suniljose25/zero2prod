-- migrations/20240229163447_create_subscriptions_table.sql
-- Create Subscriptions Table
CREATE TABLE subscriptions(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL
);