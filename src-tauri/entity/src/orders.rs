//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub seller_id: String,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::order_items::Entity")]
    OrderItems,
    #[sea_orm(
        belongs_to = "super::sellers::Entity",
        from = "Column::SellerId",
        to = "super::sellers::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Sellers,
}

impl Related<super::order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItems.def()
    }
}

impl Related<super::sellers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sellers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}