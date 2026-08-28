ALTER TABLE api_tokens ADD COLUMN scopes TEXT NOT NULL DEFAULT 'writeMetrics';

UPDATE api_tokens SET scopes = scope WHERE scope IS NOT NULL AND scope != '';

ALTER TABLE api_tokens DROP COLUMN scope;
