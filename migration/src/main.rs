use sea_orm_migration::prelude::*;
use std::env;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}