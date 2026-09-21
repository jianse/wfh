use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FK_USER: &str = "FK_user_refresh_token";

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
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_USER)
                            .from(RefreshToken::Table, RefreshToken::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshToken::Table).to_owned())
            .await
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
