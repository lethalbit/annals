-- SPDX-License-Identifier: BSD-3-Clause
-- Rollback IRC Log table and indices

DROP INDEX IF EXISTS irc_log_ts_index;
DROP INDEX IF EXISTS irc_log_chan_ts_index;
DROP TABLE IF EXISTS irc_logs;
