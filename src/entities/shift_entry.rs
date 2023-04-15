//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "shift_entry")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub shift_id: i32,
    pub time_clock_in: DateTime,
    pub time_clock_out: Option<DateTime>,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::shifts::Entity",
        from = "Column::ShiftId",
        to = "super::shifts::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Shifts,
}

impl Related<super::shifts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Shifts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}