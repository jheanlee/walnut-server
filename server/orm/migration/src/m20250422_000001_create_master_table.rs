use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Master::Table)
          .if_not_exists()
          .col(pk_auto(Master::Id))
          .col(string_uniq(Master::Username))
          .col(string(Master::HashedPassword))
          .col(string(Master::MasterSalt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Master::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Master {
  Table,
  Id,
  Username,
  HashedPassword,
  MasterSalt,
}
