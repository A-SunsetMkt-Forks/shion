//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0-rc.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub color: String,
    pub sort: i64,
    pub plan_id: i64,
    pub hidden: bool,
    pub deleted_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::dimension_label::Entity")]
    DimensionLabel,
    #[sea_orm(has_many = "super::note::Entity")]
    Note,
    #[sea_orm(
        belongs_to = "super::plan::Entity",
        from = "Column::PlanId",
        to = "super::plan::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Plan,
}

impl Related<super::dimension_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DimensionLabel.def()
    }
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl Related<super::plan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plan.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
