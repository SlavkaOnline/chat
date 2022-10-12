//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user: Json,
    pub to: Option<Json>,
    #[sea_orm(column_type = "Custom(\"array\".to_owned())")]
    pub tags: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub text: Option<String>,
    pub date_time: Option<DateTime>,
    pub room_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::rooms::Entity",
        from = "Column::RoomId",
        to = "super::rooms::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Rooms,
}

impl Related<super::rooms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rooms.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
