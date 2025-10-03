#[cfg(feature = "adapter-mig-refinery")]
mod refinery;
use std::path::{Path, PathBuf};

#[cfg(feature = "adapter-mig-refinery")]
pub use refinery::Migrator;

pub trait Migration {
    fn up(&self);
    fn down(&self);
}

fn get_migrations_path(database: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("migrations")
        .join(database)
}
