use dotenv::dotenv;
use migration::Migrator;
use sea_orm_migration::prelude::*;
use settings::Settings;

#[async_std::main]
async fn main() {
    dotenv().ok();
    let settings = Settings::new().expect("Ошибка при загрузке конфига");

    let db = Migrator::set_up_db(
        &settings.database.host,
        &settings.database.login,
        &settings.database.password,
        &settings.database.name,
        false,
    )
    .await
    .expect("Ошибка подключения к базе данных");

    cli::run_migrate(Migrator, &db, None, true)
        .await
        .expect("Ошибка при примененнии миграций");
}
