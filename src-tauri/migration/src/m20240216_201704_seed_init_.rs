use std::ops::Range;

use crate::{
    m20220101_000001_init_::{
        Client, InventoryMouvement, Invoice, InvoiceItem, Order, OrderItem, Product, Quote,
        QuoteItem, Supplier,
    },
    utils::get_random_enum,
};
use fake::{
    faker::{
        address::en::SecondaryAddress,
        internet::en::FreeEmail,
        lorem::en::{Sentence, Word},
        name::en::Name,
        phone_number::en::PhoneNumber,
    },
    Fake, Faker,
};
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for _ in 0..200 {
            let id = uuid::Uuid::now_v7();
            let fullname: String = Name().fake();
            let address: String = SecondaryAddress().fake();
            let email: String = FreeEmail().fake();
            let phone: String = PhoneNumber().fake();
            let insert = Query::insert()
                .into_table(Client::Table)
                .columns([
                    Client::Id,
                    Client::Fullname,
                    Client::Address,
                    Client::Email,
                    Client::Phone,
                ])
                .values_panic([
                    id.to_string().into(),
                    fullname.into(),
                    address.into(),
                    email.into(),
                    phone.into(),
                ])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        for _ in 0..200 {
            let id = uuid::Uuid::now_v7();
            let fullname: String = Name().fake();
            let address: String = SecondaryAddress().fake();
            let email: String = FreeEmail().fake();
            let phone: String = PhoneNumber().fake();
            let insert = Query::insert()
                .into_table(Supplier::Table)
                .columns([
                    Supplier::Id,
                    Supplier::Fullname,
                    Supplier::Address,
                    Supplier::Email,
                    Supplier::Phone,
                ])
                .values_panic([
                    id.to_string().into(),
                    fullname.into(),
                    address.into(),
                    email.into(),
                    phone.into(),
                ])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        for _ in 0..400 {
            let id = uuid::Uuid::now_v7();
            let name: String = Word().fake();
            let rand: u8 = Faker.fake();
            let address: String = Sentence(Range { start: 5, end: 10 }).fake();
            let price: u8 = Faker.fake();
            let quantity: u8 = Faker.fake();
            let insert = Query::insert()
                .into_table(Product::Table)
                .columns([
                    Product::Id,
                    Product::Name,
                    Product::Description,
                    Product::Price,
                    Product::MinQuantity,
                ])
                .values_panic([
                    id.to_string().into(),
                    format!("{}-{}", name, rand).into(),
                    address.into(),
                    price.into(),
                    quantity.into(),
                ])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        let db = manager.get_connection();

        let status = vec![
            String::from("DELIVERED"),
            String::from("CANCELED"),
            String::from("PENDING"),
        ];

        for _ in 0..100 {
            let id = uuid::Uuid::now_v7();
            let status_ = get_random_enum(status.clone());
            let insert_order = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    orders (id, status, supplier_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM suppliers ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [id.to_string().into(), status_.into()],
            );
            db.execute(insert_order).await?;
        }

        for _ in 0..1000 {
            let _id = uuid::Uuid::now_v7();
            let quantity: u8 = Faker.fake();
            let insert_inventory = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    inventory_mouvements (id, mvm_type, quantity, product_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        $3,
                        (SELECT id FROM products ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [
                    _id.to_string().into(),
                    String::from("IN").into(),
                    quantity.into(),
                ],
            );
            db.execute(insert_inventory).await?;
            //
            let id = uuid::Uuid::now_v7();
            let price: u8 = Faker.fake();
            let insert_order = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    order_items (id, price, order_id, inventory_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM orders ORDER BY RANDOM() LIMIT 1),
                        $3
                    )
                "#,
                [id.to_string().into(), price.into(), _id.to_string().into()],
            );
            db.execute(insert_order).await?;
        }

        let status = vec![
            String::from("PAID"),
            String::from("CANCELED"),
            String::from("PENDING"),
        ];

        for _ in 0..100 {
            let id = uuid::Uuid::now_v7();
            let status_ = get_random_enum(status.clone());
            let paid: u8 = Faker.fake();
            let insert_invoice = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    invoices (id, status, client_id, paid_amount)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1),
                        $3

                    )
                "#,
                [id.to_string().into(), status_.into(), paid.into()],
            );
            db.execute(insert_invoice).await?;
        }

        for _ in 0..1000 {
            let _id = uuid::Uuid::now_v7();
            let quantity: u8 = Faker.fake();
            let insert_inventory = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    inventory_mouvements (id, mvm_type, quantity, product_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        $3,
                        (SELECT id FROM products ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [
                    _id.to_string().into(),
                    String::from("OUT").into(),
                    quantity.into(),
                ],
            );
            db.execute(insert_inventory).await?;

            let id = uuid::Uuid::now_v7();
            let price: u8 = Faker.fake();
            let insert_invoice = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    invoice_items (id, price, invoice_id, inventory_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM invoices ORDER BY RANDOM() LIMIT 1),
                        $3
                    )
                "#,
                [id.to_string().into(), price.into(), _id.to_string().into()],
            );
            db.execute(insert_invoice).await?;
        }

        for _ in 0..150 {
            let id = uuid::Uuid::now_v7();
            let insert_quote = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    quotes (id, client_id)
                VALUES
                    (
                        $1, 
                        (SELECT id FROM clients ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [id.to_string().into()],
            );
            db.execute(insert_quote).await?;
        }

        for _ in 0..1000 {
            let id = uuid::Uuid::now_v7();
            let price: u8 = Faker.fake();
            let insert_quote = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                r#"
                INSERT INTO 
                    quote_items (id, price, product_id, quote_id)
                VALUES
                    (
                        $1, 
                        $2, 
                        (SELECT id FROM products ORDER BY RANDOM() LIMIT 1),
                        (SELECT id FROM quotes ORDER BY RANDOM() LIMIT 1)
                    )
                "#,
                [id.to_string().into(), price.into()],
            );
            db.execute(insert_quote).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_item = Query::delete().from_table(OrderItem::Table).to_owned();
        manager.exec_stmt(delete_item).await?;

        let delete_item = Query::delete().from_table(InvoiceItem::Table).to_owned();
        manager.exec_stmt(delete_item).await?;

        let delete_item: DeleteStatement = Query::delete().from_table(QuoteItem::Table).to_owned();
        manager.exec_stmt(delete_item).await?;

        let delete = Query::delete().from_table(Order::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Invoice::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Quote::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete()
            .from_table(InventoryMouvement::Table)
            .to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Product::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Client::Table).to_owned();
        manager.exec_stmt(delete).await?;

        let delete = Query::delete().from_table(Supplier::Table).to_owned();
        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
