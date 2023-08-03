-- SPDX-License-Identifier: BSD-3-Clause
-- Rollback IRC Server table and index

DROP INDEX IF EXISTS irc_server_index;
DROP TABLE IF EXISTS irc_servers;
