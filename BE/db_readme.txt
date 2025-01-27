
brew update
brew install postgresql
brew services start postgresql
psql --version

CREATE DATABASE postgres;
\l

CREATE ROLE "postgres.hhjgxwbwdsvklegckumk" WITH LOGIN PASSWORD 'f188G8Gdj0LgH38x';
\du

GRANT ALL PRIVILEGES ON DATABASE postgres TO "postgres.hhjgxwbwdsvklegckumk";

\c postgres

CREATE SCHEMA my_schema AUTHORIZATION "postgres.hhjgxwbwdsvklegckumk";

GRANT CREATE, USAGE ON SCHEMA my_scheme TO "postgres.hhjgxwbwdsvklegckumk";

ALTER SCHEMA my_scheme OWNER TO "postgres.hhjgxwbwdsvklegckumk";