BEGIN;
-- noinspection SqlConstantExpression
UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status is NULL;

    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
