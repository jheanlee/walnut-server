use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Password::Table)
          .if_not_exists()
          .col(pk_auto(Password::Id))
          .col(string(Password::Master))
          .col(string(Password::Website))
          .col(string_null(Password::Username))
          .col(string_null(Password::Email))
          .col(string(Password::HashedPassword))
          .col(string_null(Password::Notes))
          .col(string(Password::Salt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Password::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Password {
  Table,
  Id,
  Master,
  Website,
  Username,
  Email,
  HashedPassword,
  Notes,
  Salt,
}
