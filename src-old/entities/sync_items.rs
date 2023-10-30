//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sync_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub sync_dir_id: i32,
    /// The local item being synced, as an absolute path with no '/' at the end.
    pub local_path: String,
    /// The remote path being synced, relative to the directory of the matching
    /// `SyncDirs::sync_dir` specified by `Self::sync_dir_id`.
    pub remote_path: String,
    /// The local UNIX timestamp of the item when last synced.
    pub last_local_timestamp: i32,
    /// The remote UNIX timestamp of the item when last synced.
    pub last_remote_timestamp: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sync_dirs::Entity",
        from = "Column::SyncDirId",
        to = "super::sync_dirs::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SyncDirs,
}

impl Related<super::sync_dirs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SyncDirs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
