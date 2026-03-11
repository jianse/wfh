use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FK_USER: &str = "FK_user_refresh_token";
const IDX_USER_ID: &str = "IDX_user_id";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RefreshToken::Table)
                    .if_not_exists()
                    .col(pk_uuid(RefreshToken::Id))
                    .col(integer(RefreshToken::UserId))
                    .col(string_len(RefreshToken::DeviceId, 50))
                    .col(string_len_uniq(RefreshToken::TokenHash, 50))
                    .col(string_len_null(RefreshToken::BrowserFamily, 50))
                    .col(string_len_null(RefreshToken::BrowserVersion, 50))
                    .col(string_len_null(RefreshToken::OsFamily, 50))
                    .col(string_len_null(RefreshToken::OsVersion, 50))
                    .col(string_len_null(RefreshToken::DeviceFamily, 50))
                    .col(string_len(RefreshToken::IpAddress, 45))
                    .col(timestamp(RefreshToken::CreateTime))
                    .col(timestamp(RefreshToken::ExpiresTime))
                    .to_owned(),
            )
            .await?;
        match manager.get_database_backend() {
            sea_orm::DatabaseBackend::MySql | sea_orm::DatabaseBackend::Postgres => {
                manager
                    .create_foreign_key(
                        sea_query::ForeignKey::create()
                            .name(FK_USER)
                            .from(RefreshToken::Table, RefreshToken::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .await?;
            }
            sea_orm::DatabaseBackend::Sqlite => {
                manager
                    .create_index(
                        Index::create()
                            .name(IDX_USER_ID)
                            .col(RefreshToken::UserId)
                            .table(RefreshToken::Table)
                            .to_owned(),
                    )
                    .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            sea_orm::DatabaseBackend::MySql | sea_orm::DatabaseBackend::Postgres => {
                manager
                    .drop_foreign_key(
                        ForeignKey::drop()
                            .name(FK_USER)
                            .table(RefreshToken::Table)
                            .to_owned(),
                    )
                    .await?;
            }
            sea_orm::DatabaseBackend::Sqlite => {
                manager
                    .drop_index(
                        Index::drop()
                            .name(IDX_USER_ID)
                            .table(RefreshToken::Table)
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .drop_table(Table::drop().table(RefreshToken::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RefreshToken {
    Table,
    Id,
    UserId,
    DeviceId,

    TokenHash,

    BrowserFamily,
    BrowserVersion,
    OsFamily,
    OsVersion,
    DeviceFamily,

    IpAddress,
    CreateTime,

    ExpiresTime,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
