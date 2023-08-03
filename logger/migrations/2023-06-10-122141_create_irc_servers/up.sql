-- SPDX-License-Identifier: BSD-3-Clause
-- Create IRC Server table and index

CREATE TABLE irc_servers (
	id SERIAL,
	server_name VARCHAR(50) NOT NULL,
	PRIMARY KEY(id)
);

CREATE INDEX irc_server_index ON irc_servers (server_name);
