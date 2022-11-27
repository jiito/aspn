ALTER TABLE hosts
ADD COLUMN is_online boolean
not null default false;
