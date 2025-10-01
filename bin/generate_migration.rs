#[cfg(not(feature = "dev-bin"))]
compile_error!("bin requires features: `dev-bin`");

use std::{fs::File, path::Path};

use chrono::DateTime;
use clap::Parser;

#[derive(Parser, Default, Debug)]
struct Arguments {
    database: String,
    name: String,
}

pub fn main() {
    let args = Arguments::parse();

    let database = &args.database;
    let migration_name = args.name.replace(" ", "_");
    let migration_name: &str = &migration_name;
    let current_timestamp = chrono::offset::Utc::now().timestamp_millis();
    let mut migration_name = format!("{current_timestamp:?}_{migration_name}");
    migration_name.push_str(".sql");
    println!("Creating migration: {migration_name}");
    let migration_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("migrations")
        .join(database)
        .join(migration_name);

    let migration_file = File::create(&migration_path).unwrap();
}
