pub use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{
    ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, QueryResult, Statement,
};

mod m20220101_000001_rooms;
mod m20221011_174636_mesages;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_rooms::Migration),
            Box::new(m20221011_174636_mesages::Migration),
        ]
    }
}

impl Migrator {
    pub async fn set_up_db(
        host: &str,
        login: &str,
        password: &str,
        database: &str,
        drop: bool,
    ) -> Result<DatabaseConnection, DbErr> {
        let url = format!(
            "postgres://{}:{}@{}",
            login.to_owned(),
            password.to_owned(),
            host.to_owned()
        );
        let db = Database::connect(&url).await?;

        if drop {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", database.to_owned()),
            ))
            .await?;

            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE\"{}\";", database.to_owned()),
            ))
            .await?;
        } else {
            let query_res: Option<QueryResult> = db
                .query_one(Statement::from_string(
                    DatabaseBackend::Postgres,
                    format!(
                        "SELECT EXISTS(SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower('{}')) as exists;",
                        database.to_owned()
                    ),
                ))
                .await?;
            let query_res = query_res.unwrap();
            let exists: bool = query_res.try_get("", "exists")?;

            if !exists {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE\"{}\";", database.to_owned()),
                ))
                .await?;
            }
        }

        let url = format!("{}/{}", url, database.to_owned());
        Database::connect(&url).await?;

        Ok(db)
    }
}
