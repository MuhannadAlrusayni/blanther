CREATE TABLE users (
	id BIGSERIAL PRIMARY KEY NOT NULL,
	first_name TEXT,
	last_name TEXT,
	email TEXT NOT NULL UNIQUE,
	phone_number TEXT UNIQUE,
	username TEXT,
	email_verified BOOLEAN NOT NULL DEFAULT FALSE,
	phone_Verified BOOLEAN NOT NULL DEFAULT FALSE
);
