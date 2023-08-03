-- SPDX-License-Identifier: BSD-3-Clause
-- Create IRC User table and index

CREATE TABLE irc_users (
	id SERIAL,
	nickname VARCHAR(50) NOT NULL,
	username VARCHAR(50),
	first_seen TIMESTAMP NOT NULL,
	PRIMARY KEY(id)
);

CREATE INDEX irc_user_index ON irc_users (nickname);
