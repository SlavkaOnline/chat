pub use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{Database, DatabaseConnection, ConnectionTrait, Statement};

mod m20220101_000001_create_rooma_and_messages;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_rooma_and_messages::Migration)]
    }
}

impl Migrator {
    pub async fn set_up_db(
        login: &str,
        password: &str,
        host: &str,
        database: &str,
        drop: bool
    ) -> Result<DatabaseConnection, DbErr> {
    
        let url = format!("postgres://{}:{}@{}", login.to_owned(), password.to_owned(), host.to_owned());
        let db = Database::connect(&url).await?;
    
        if drop {

            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", database.to_owned()),
            ))
            .await?;
        }
        
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("CREATE DATABASE \"{}\";", database.to_owned()),
        ))
        .await?;
        
        let url = format!("{}/{}", url, database.to_owned());
        
        Database::connect(&url).await?;
        
        Migrator::up(&db, None).await?;

        Ok(db)
    }
}

