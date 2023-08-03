// SPDX-License-Identifier: BSD-3-Clause

use diesel_migrations::{
	embed_migrations, EmbeddedMigrations, MigrationHarness
};

use diesel::{
	PgConnection
};


pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!(
	"migrations"
);

pub fn run_migrations(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
