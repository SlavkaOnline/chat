use super::m20220101_000001_rooms::Rooms;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Messages::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Messages::User).json_binary().not_null())
                    .col(ColumnDef::new(Messages::To).json_binary().null())
                    .col(ColumnDef::new(Messages::Text).text().not_null())
                    .col(
                        ColumnDef::new(Messages::DateTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Messages::RoomId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-rooms_id")
                            .from(Messages::Table, Messages::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                            Index::create()
                            .name("IX_messages_room_id_date_time")
                            .table(Messages::Table)
                            .col(Messages::RoomId)
                            .col(Messages::DateTime)
                            .unique()
                            .index_type(IndexType::BTree)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Messages {
    Table,
    Id,
    User,
    To,
    Text,
    DateTime,
    RoomId,
}
