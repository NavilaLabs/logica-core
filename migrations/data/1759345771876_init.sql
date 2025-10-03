-- Create role (no IF NOT EXISTS in Postgres)
DO $$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'logica_data_role') THEN
      CREATE ROLE logica_data_role LOGIN PASSWORD '${LOGICA_DATA_DB_PASSWORD}';
   END IF;
END
$$;

-- Create database
DO $$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_database WHERE datname = 'logica_data') THEN
      CREATE DATABASE logica_data OWNER logica_data_role;
   END IF;
END
$$;

-- Switch into the new DB (run this part *after connecting to logica_data*)

-- Create schemas
CREATE SCHEMA IF NOT EXISTS config AUTHORIZATION logica_data_role;
CREATE SCHEMA IF NOT EXISTS events AUTHORIZATION logica_data_role;
CREATE SCHEMA IF NOT EXISTS aggregates AUTHORIZATION logica_data_role;

-- Grant privileges
GRANT CONNECT ON DATABASE logica_data TO logica_data_role;
GRANT USAGE ON SCHEMA config, events, aggregates TO logica_data_role;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA config, events, aggregates TO logica_data_role;

-- Ensure new tables get the same rights
ALTER DEFAULT PRIVILEGES IN SCHEMA config, events, aggregates
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO logica_data_role;
