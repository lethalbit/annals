-- SPDX-License-Identifier: BSD-3-Clause
-- Create IRC Log table and indices

CREATE TABLE irc_logs (
	id SERIAL,
	channel VARCHAR(50) NOT NULL,
	ts TIMESTAMP NOT NULL,
	user_id INT NOT NULL,
	server_id INT NOT NULL,
	raw_line TEXT NOT NULL,

	PRIMARY KEY(id),

	CONSTRAINT fk_user
		FOREIGN KEY(user_id)
			REFERENCES irc_users(id),
	CONSTRAINT fk_server
		FOREIGN KEY(server_id)
			REFERENCES irc_servers(id)
);

CREATE INDEX irc_log_ts_index ON irc_logs (ts);
CREATE INDEX irc_log_chan_ts_index ON irc_logs (channel, ts);
