-- SPDX-License-Identifier: BSD-3-Clause
-- Rollback IRC User table and index

DROP INDEX IF EXISTS irc_user_index;
DROP TABLE IF EXISTS irc_users;
