use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Client::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Client::Fullname).string().not_null())
                    .col(
                        ColumnDef::new(Client::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Client::Phone).string())
                    .col(ColumnDef::new(Client::Email).string())
                    .col(ColumnDef::new(Client::Address).string())
                    .col(ColumnDef::new(Client::Image).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Supplier::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Supplier::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Supplier::Fullname).string().not_null())
                    .col(
                        ColumnDef::new(Supplier::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Supplier::Phone).string())
                    .col(ColumnDef::new(Supplier::Email).string())
                    .col(ColumnDef::new(Supplier::Address).string())
                    .col(ColumnDef::new(Supplier::Image).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Product::Description).string())
                    .col(
                        ColumnDef::new(Product::Price)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(Product::MinQuantity)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(ColumnDef::new(Product::Image).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InventoryMouvement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InventoryMouvement::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InventoryMouvement::MvmType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InventoryMouvement::Quantity)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(
                        ColumnDef::new(InventoryMouvement::ProductId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_mvm_product_id")
                            .from(InventoryMouvement::Table, InventoryMouvement::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Order::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Order::SupplierId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_supplier_id")
                            .from(Order::Table, Order::SupplierId)
                            .to(Supplier::Table, Supplier::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Order::Status).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrderItem::Price)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(ColumnDef::new(OrderItem::OrderId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_item_order_id")
                            .from(OrderItem::Table, OrderItem::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrderItem::InventoryId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_item_inventory_id")
                            .from(OrderItem::Table, OrderItem::InventoryId)
                            .to(InventoryMouvement::Table, InventoryMouvement::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Invoice::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invoice::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Invoice::ClientId).string().not_null())
                    .col(
                        ColumnDef::new(Invoice::PaidAmount)
                            .float()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_client_id")
                            .from(Invoice::Table, Invoice::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Invoice::Status).string().not_null())
                    .col(
                        ColumnDef::new(Invoice::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InvoiceItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InvoiceItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InvoiceItem::Price)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(ColumnDef::new(InvoiceItem::InvoiceId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_item_invoice_id")
                            .from(InvoiceItem::Table, InvoiceItem::InvoiceId)
                            .to(Invoice::Table, Invoice::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(InvoiceItem::InventoryId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_invoice_item_inventory_id")
                            .from(InvoiceItem::Table, InvoiceItem::InventoryId)
                            .to(InventoryMouvement::Table, InventoryMouvement::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Quote::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Quote::ClientId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_client_id")
                            .from(Quote::Table, Quote::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Quote::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuoteItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuoteItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(QuoteItem::Price)
                            .float()
                            .not_null()
                            .default(0.0f32),
                    )
                    .col(ColumnDef::new(QuoteItem::ProductId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_item_product_id")
                            .from(QuoteItem::Table, QuoteItem::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(QuoteItem::QuoteId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_item_quote_id")
                            .from(QuoteItem::Table, QuoteItem::QuoteId)
                            .to(Quote::Table, Quote::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Client {
    #[sea_orm(iden = "clients")]
    Table,
    Id,
    #[sea_orm(iden = "full_name")]
    Fullname,
    #[sea_orm(iden = "phone_number")]
    Phone,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "image")]
    Image,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "address")]
    Address,
}

#[derive(DeriveIden)]
pub enum Supplier {
    #[sea_orm(iden = "suppliers")]
    Table,
    Id,
    #[sea_orm(iden = "full_name")]
    Fullname,
    #[sea_orm(iden = "phone_number")]
    Phone,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "image")]
    Image,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "address")]
    Address,
}

#[derive(DeriveIden)]
pub enum Product {
    #[sea_orm(iden = "products")]
    Table,
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "image")]
    Image,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "price")]
    Price,
    #[sea_orm(iden = "min_quantity")]
    MinQuantity,
}

#[derive(DeriveIden)]
pub enum InventoryMouvement {
    #[sea_orm(iden = "inventory_mouvements")]
    Table,
    Id,
    #[sea_orm(iden = "mvm_type")]
    MvmType,
    #[sea_orm(iden = "quantity")]
    Quantity,
    #[sea_orm(iden = "product_id")]
    ProductId,
}

#[derive(DeriveIden)]
pub enum Order {
    #[sea_orm(iden = "orders")]
    Table,
    Id,
    #[sea_orm(iden = "supplier_id")]
    SupplierId,
    // status: delivered, cancel, ongoing
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum OrderItem {
    #[sea_orm(iden = "order_items")]
    Table,
    Id,
    #[sea_orm(iden = "order_id")]
    OrderId,
    #[sea_orm(iden = "inventory_id")]
    InventoryId,
    #[sea_orm(iden = "price")]
    Price,
}

#[derive(DeriveIden)]
pub enum Invoice {
    #[sea_orm(iden = "invoices")]
    Table,
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    // status: paid, cancel, ongoing
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "paid_amount")]
    PaidAmount,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum InvoiceItem {
    #[sea_orm(iden = "invoice_items")]
    Table,
    Id,
    #[sea_orm(iden = "invoice_id")]
    InvoiceId,
    #[sea_orm(iden = "inventory_id")]
    InventoryId,
    #[sea_orm(iden = "price")]
    Price,
}

#[derive(DeriveIden)]
pub enum Quote {
    #[sea_orm(iden = "quotes")]
    Table,
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum QuoteItem {
    #[sea_orm(iden = "quote_items")]
    Table,
    Id,
    #[sea_orm(iden = "product_id")]
    ProductId,
    #[sea_orm(iden = "quote_id")]
    QuoteId,
    #[sea_orm(iden = "price")]
    Price,
}
