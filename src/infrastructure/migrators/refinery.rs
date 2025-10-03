use refinery::Migration;
use refinery::embed_migrations;

mod embedded {}

pub struct Migrator;

impl Migrator {
    fn up_master() {
        embed_migrations!("./tests/sql_migrations");
    }
}

impl super::Migration for Migrator {
    fn up(&self) {
        embedded::migrations::runner()
            .run(&mut std::io::stdout())
            .unwrap();
    }

    fn down(&self) {
        // Implement down migration logic if needed
        unimplemented!("Down migration is not implemented");
    }
}
