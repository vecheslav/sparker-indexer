//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::OrderStatus;
use super::sea_orm_active_enums::OrderType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tx_id: String,
    #[sea_orm(unique)]
    pub order_id: String,
    pub order_type: OrderType,
    pub user: String,
    pub asset: String,
    pub amount: i64,
    pub price: i64,
    pub status: OrderStatus,
    pub block_number: i64,
    pub timestamp: DateTime,
    pub market_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::trade::Entity")]
    Trade,
}

impl Related<super::trade::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Trade.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
