-- Your SQL goes here
CREATE TABLE IF NOT EXISTS discord_users(
	id SERIAL PRIMARY KEY,
	uname VARCHAR NOT NULL,
	discord_id VARCHAR NOT NULL
)