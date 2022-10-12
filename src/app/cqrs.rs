use async_trait::async_trait;
use sea_orm::DbErr;

#[derive(Clone,Debug)]
pub enum AppErr
{
    Bug (String)
}

impl From<DbErr> for AppErr
{
    fn from(err: DbErr) -> Self { 
        AppErr::Bug(err.to_string())
     }
}

pub type AppResult<TOk> = Result<TOk, AppErr>;

#[async_trait]
pub trait Command {
    type Out;
    async fn execute (&self) -> AppResult<Self::Out>;
}