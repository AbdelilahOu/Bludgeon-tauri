use sea_orm_migration::prelude::*;

use crate::m20220101_000001_init_::{Client, Invoice, Order, Product, Supplier};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                sea_query::Index::create()
                    .table(Order::Table)
                    .col(Order::Status)
                    .name("idx_orders_status")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Invoice::Table)
                    .col(Invoice::Status)
                    .name("idx_invoices_status")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Client::Table)
                    .col(Client::Fullname)
                    .name("idx_clients_fullname")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Supplier::Table)
                    .col(Supplier::Fullname)
                    .name("idx_suppliers_fullname")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Product::Table)
                    .col(Product::Name)
                    .name("idx_products_name")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
