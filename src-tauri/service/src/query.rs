use entity::prelude::*;
use sea_orm::{
    sea_query::{
        Alias, Cond, Expr, Func, IntoCondition, Query, SimpleExpr, SqliteQueryBuilder,
        SubQueryStatement,
    },
    ColumnTrait, Condition, DatabaseConnection as DbConn, DbBackend, DbErr, EntityTrait,
    FromQueryResult, JoinType, JsonValue, Order, PaginatorTrait, QueryFilter, QuerySelect,
    QueryTrait, RelationTrait, Statement,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    SelectClients, SelectExpenses, SelectInventory, SelectInvoiceDetails, SelectInvoices,
    SelectInvoicesItems, SelectInvoicesItemsForUpdate, SelectMvm, SelectOrderDetails, SelectOrders,
    SelectOrdersItems, SelectOrdersItemsForUpdate, SelectProducts, SelectRevenue,
    SelectStatusCount, SelectSuppliers, SelectTops,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListArgs {
    pub page: u64,
    pub limit: u64,
    pub search: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
}

pub struct QueriesService;

impl QueriesService {
    pub async fn list_products(db: &DbConn, args: ListArgs) -> Result<JsonValue, DbErr> {
        let count = Products::find()
            .filter(
                Condition::any()
                    .add(products::Column::Name.like(format!("{}%", args.search)))
                    .add(products::Column::Description.like(format!("%{}%", args.search))),
            )
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Products)
            .exprs([
                Expr::col((Products, products::Column::Id)),
                Expr::col((Products, products::Column::Name)),
                Expr::col((Products, products::Column::Description)),
                Expr::col((Products, products::Column::Image)),
                Expr::col((Products, products::Column::CreatedAt)),
                Expr::col((Products, products::Column::Price)),
                Expr::col((Products, products::Column::MinQuantity)),
            ])
            .expr_as(
                SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(InventoryMouvements)
                            .expr(Func::coalesce([
                                Func::sum(Expr::col(inventory_mouvements::Column::Quantity)).into(),
                                Expr::val(0.0f64).into(),
                            ]))
                            .join(
                                JoinType::Join,
                                OrderItems,
                                Expr::col((OrderItems, order_items::Column::InventoryId)).equals((
                                    InventoryMouvements,
                                    inventory_mouvements::Column::Id,
                                )),
                            )
                            .join(
                                JoinType::Join,
                                Orders,
                                Expr::col((Orders, orders::Column::Id))
                                    .equals((OrderItems, order_items::Column::OrderId)),
                            )
                            .cond_where(
                                Cond::all().add(
                                    Expr::col((
                                        InventoryMouvements,
                                        inventory_mouvements::Column::ProductId,
                                    ))
                                    .equals((Products, products::Column::Id))
                                    .into_condition()
                                    .add(
                                        Cond::any()
                                            .add(orders::Column::Status.eq("PENDING"))
                                            .add(orders::Column::Status.eq("DELIVERED")),
                                    ),
                                ),
                            )
                            .to_owned(),
                    )),
                )
                .sub(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(InventoryMouvements)
                            .expr(Func::coalesce([
                                Func::sum(Expr::col(inventory_mouvements::Column::Quantity)).into(),
                                Expr::val(0.0f64).into(),
                            ]))
                            .join(
                                JoinType::Join,
                                InvoiceItems,
                                Expr::col((InvoiceItems, invoice_items::Column::InventoryId))
                                    .equals((
                                        InventoryMouvements,
                                        inventory_mouvements::Column::Id,
                                    )),
                            )
                            .join(
                                JoinType::Join,
                                Invoices,
                                Expr::col((Invoices, invoices::Column::Id))
                                    .equals((InvoiceItems, invoice_items::Column::InvoiceId)),
                            )
                            .cond_where(
                                Cond::all().add(
                                    Expr::col((
                                        InventoryMouvements,
                                        inventory_mouvements::Column::ProductId,
                                    ))
                                    .equals((Products, products::Column::Id))
                                    .into_condition()
                                    .add(
                                        Cond::any()
                                            .add(invoices::Column::Status.eq("PENDING"))
                                            .add(invoices::Column::Status.eq("PAID")),
                                    ),
                                ),
                            )
                            .to_owned(),
                    )),
                )),
                Alias::new("stock"),
            )
            .cond_where(
                Cond::any()
                    .add(
                        Expr::col((Products, products::Column::Name))
                            .like(format!("{}%", args.search))
                            .into_condition(),
                    )
                    .add(
                        Expr::col((Products, products::Column::Description))
                            .like(format!("%{}%", args.search))
                            .into_condition(),
                    ),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by(products::Column::CreatedAt, Order::Desc)
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectProducts::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "id": row.id,
                "name": row.name,
                "description": row.description,
                "image": row.image,
                "price": row.price,
                "minQuantity": row.min_quantity,
                "stock": row.stock,
                "createdAt": row.created_at,
            }));
        });

        Ok(json!({
            "count": count,
            "products": result
        }))
    }
    pub async fn search_products(db: &DbConn, search: String) -> Result<Vec<JsonValue>, DbErr> {
        let products = Products::find()
            .select_only()
            .expr_as_(Expr::col(products::Column::Name), "label")
            .expr_as_(Expr::col(products::Column::Id), "value")
            .expr(Expr::col(products::Column::Price))
            .filter(products::Column::Name.like(format!("{}%", search)))
            .into_json()
            .all(db)
            .await?;

        Ok(products)
    }
    //
    pub async fn list_clients(db: &DbConn, args: ListArgs) -> Result<JsonValue, DbErr> {
        let count = Clients::find()
            .filter(clients::Column::FullName.like(format!("{}%", args.search)))
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Clients)
            .exprs([
                Expr::col((Clients, clients::Column::Id)),
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Image)),
                Expr::col((Clients, clients::Column::Email)),
            ])
            .expr_as(
                SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(Invoices)
                            .expr(Func::coalesce([
                                Func::sum(
                                    Expr::col((
                                        InventoryMouvements,
                                        inventory_mouvements::Column::Quantity,
                                    ))
                                    .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                                )
                                .into(),
                                Expr::val(0.0f64).into(),
                            ]))
                            .inner_join(
                                InvoiceItems,
                                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                                    .equals((Invoices, invoices::Column::Id)),
                            )
                            .inner_join(
                                InventoryMouvements,
                                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                                    .equals((InvoiceItems, invoice_items::Column::InventoryId)),
                            )
                            .cond_where(
                                Cond::all().add(
                                    Expr::col((Invoices, invoices::Column::Status))
                                        .eq("PAID")
                                        .into_condition()
                                        .add(
                                            Expr::col((Invoices, invoices::Column::ClientId))
                                                .equals((Clients, clients::Column::Id))
                                                .into_condition(),
                                        ),
                                ),
                            )
                            .to_owned(),
                    )),
                ),
                Alias::new("credi"),
            )
            .cond_where(
                Expr::col((Clients, clients::Column::FullName))
                    .like(format!("{}%", args.search))
                    .into_condition(),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by(clients::Column::CreatedAt, Order::Desc)
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectClients::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "id": row.id,
                "fullname": row.full_name,
                "address": row.address,
                "image": row.image,
                "email": row.email,
                "phoneNumber": row.phone_number,
                "credi": row.credi,
            }));
        });

        Ok(json!({
            "count": count,
            "clients": result
        }))
    }
    pub async fn search_clients(db: &DbConn, search: String) -> Result<Vec<JsonValue>, DbErr> {
        let clients = Clients::find()
            .select_only()
            .expr_as_(Expr::col(clients::Column::FullName), "label")
            .expr_as_(Expr::col(clients::Column::Id), "value")
            .filter(clients::Column::FullName.like(format!("{}%", search)))
            .into_json()
            .all(db)
            .await?;

        Ok(clients)
    }
    //
    pub async fn list_suppliers(db: &DbConn, args: ListArgs) -> Result<JsonValue, DbErr> {
        let count = Suppliers::find()
            .filter(suppliers::Column::FullName.like(format!("{}%", args.search)))
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Suppliers)
            .exprs([
                Expr::col((Suppliers, suppliers::Column::Id)),
                Expr::col((Suppliers, suppliers::Column::FullName)),
                Expr::col((Suppliers, suppliers::Column::Address)),
                Expr::col((Suppliers, suppliers::Column::PhoneNumber)),
                Expr::col((Suppliers, suppliers::Column::Image)),
                Expr::col((Suppliers, suppliers::Column::Email)),
            ])
            .expr_as(
                SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(Orders)
                            .expr(Func::coalesce([
                                Func::sum(
                                    Expr::col((
                                        InventoryMouvements,
                                        inventory_mouvements::Column::Quantity,
                                    ))
                                    .mul(Expr::col((OrderItems, order_items::Column::Price))),
                                )
                                .into(),
                                Expr::val(0.0f64).into(),
                            ]))
                            .inner_join(
                                OrderItems,
                                Expr::col((OrderItems, order_items::Column::OrderId))
                                    .equals((Orders, orders::Column::Id)),
                            )
                            .inner_join(
                                InventoryMouvements,
                                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                                    .equals((OrderItems, order_items::Column::InventoryId)),
                            )
                            .cond_where(
                                Cond::all().add(
                                    Expr::col((Orders, orders::Column::Status))
                                        .eq("PAID")
                                        .into_condition()
                                        .add(
                                            Expr::col((Orders, orders::Column::SupplierId))
                                                .equals((Suppliers, suppliers::Column::Id))
                                                .into_condition(),
                                        ),
                                ),
                            )
                            .to_owned(),
                    )),
                ),
                Alias::new("credi"),
            )
            .cond_where(
                Expr::col((Suppliers, suppliers::Column::FullName))
                    .like(format!("{}%", args.search))
                    .into_condition(),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by(suppliers::Column::CreatedAt, Order::Desc)
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectSuppliers::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "id": row.id,
                "fullname": row.full_name,
                "address": row.address,
                "image": row.image,
                "email": row.email,
                "phoneNumber": row.phone_number,
                "credi": row.credi,
            }));
        });

        Ok(json!({
            "count": count,
            "suppliers": result
        }))
    }
    pub async fn search_suppliers(db: &DbConn, search: String) -> Result<Vec<JsonValue>, DbErr> {
        let suppliers = Suppliers::find()
            .select_only()
            .expr_as_(Expr::col(suppliers::Column::FullName), "label")
            .expr_as_(Expr::col(suppliers::Column::Id), "value")
            .filter(suppliers::Column::FullName.like(format!("{}%", search)))
            .into_json()
            .all(db)
            .await?;

        Ok(suppliers)
    }
    //
    pub async fn list_orders(db: &DbConn, args: ListArgs) -> Result<JsonValue, DbErr> {
        let count = Orders::find()
            .apply_if(Some(args.search.clone()), |query, v| {
                query.filter(
                    Expr::col((Suppliers, suppliers::Column::FullName)).like(format!("{}%", v)),
                )
            })
            .apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((Orders, orders::Column::Status)).eq(v))
            })
            .apply_if(args.created_at.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', orders.created_at) = ?",
                    [v],
                ))
            })
            .join(JoinType::Join, orders::Relation::Suppliers.def())
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Orders)
            .exprs([
                Expr::col((Orders, orders::Column::Id)),
                Expr::col((Orders, orders::Column::Status)),
                Expr::col((Orders, orders::Column::CreatedAt)),
                Expr::col((Orders, orders::Column::SupplierId)),
                Expr::col((Suppliers, suppliers::Column::FullName)),
            ])
            .expr_as(
                Func::coalesce([
                    Func::count(Expr::col(inventory_mouvements::Column::Quantity)).into(),
                    Expr::val(0i64).into(),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity))
                            .mul(Expr::col((OrderItems, order_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
                ]),
                Alias::new("total"),
            )
            .left_join(
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Orders, orders::Column::Id)),
            )
            .left_join(
                InventoryMouvements,
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .join(
                JoinType::Join,
                Suppliers,
                Expr::col((Suppliers, suppliers::Column::Id))
                    .equals((Orders, orders::Column::SupplierId)),
            )
            .cond_where(
                Expr::col((Suppliers, suppliers::Column::FullName))
                    .like(format!("{}%", args.search)),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((Orders, orders::Column::Status)).eq(args.status));
                },
                |_| {},
            )
            .conditions(
                args.created_at.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', orders.created_at) = ?",
                        args.created_at,
                    ));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by((Orders, orders::Column::CreatedAt), Order::Desc)
            .group_by_col((Orders, orders::Column::Id))
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectOrders::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "id": row.id,
                "supplierId": row.supplier_id,
                "createdAt": row.created_at,
                "fullname": row.full_name,
                "status": row.status,
                "products": row.products,
                "total": row.total,
            }));
        });

        Ok(json!({
            "count": count,
            "orders": result
        }))
    }
    pub async fn get_order(db: &DbConn, id: String) -> Result<JsonValue, DbErr> {
        let order = Orders::find_by_id(id.clone())
            .find_also_related(Suppliers)
            .one(db)
            .await?;

        match order {
            Some(order) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((OrderItems, order_items::Column::Id)),
                        Expr::col((OrderItems, order_items::Column::InventoryId)),
                        Expr::col((OrderItems, order_items::Column::Price)),
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .expr_as(
                        Expr::col((Products, products::Column::Id)),
                        Alias::new("product_id"),
                    )
                    .from(OrderItems)
                    .join(
                        JoinType::Join,
                        InventoryMouvements,
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                            .equals((OrderItems, order_items::Column::InventoryId)),
                    )
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((InventoryMouvements, inventory_mouvements::Column::ProductId)),
                    )
                    .cond_where(Expr::col((OrderItems, order_items::Column::OrderId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectOrdersItemsForUpdate::find_by_statement(
                    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
                )
                .all(db)
                .await?;

                let mut result = Vec::<JsonValue>::new();
                items.into_iter().for_each(|item| {
                    result.push(json!({
                        "id": item.id,
                        "inventory_id": item.inventory_id,
                        "product_id": item.product_id,
                        "price": item.price,
                        "quantity": item.quantity,
                        "name": item.name,
                    }));
                });

                Ok(json!({
                    "id": order.0.id,
                    "supplierId": order.0.supplier_id,
                    "createdAt": order.0.created_at,
                    "status": order.0.status,
                    "fullname": order.1.unwrap().full_name,
                    "items": result,
                }))
            }
            None => Err(DbErr::RecordNotFound(String::from("no order"))),
        }
    }
    pub async fn list_order_products(db: &DbConn, id: String) -> Result<Vec<JsonValue>, DbErr> {
        let order_products = OrderItems::find()
            .select_only()
            .columns([order_items::Column::Price])
            .exprs([
                Expr::col((Products, products::Column::Name)),
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
            ])
            .join(
                JoinType::Join,
                order_items::Relation::InventoryMouvements.def(),
            )
            .join(
                JoinType::Join,
                inventory_mouvements::Relation::Products.def(),
            )
            .filter(Expr::col((OrderItems, order_items::Column::OrderId)).eq(id))
            .into_json()
            .all(db)
            .await?;

        Ok(order_products)
    }
    pub async fn get_order_details(db: &DbConn, id: String) -> Result<JsonValue, DbErr> {
        let (sql, values) = Query::select()
            .from(Orders)
            .exprs([
                Expr::col((Suppliers, suppliers::Column::FullName)),
                Expr::col((Suppliers, suppliers::Column::Address)),
                Expr::col((Suppliers, suppliers::Column::PhoneNumber)),
                Expr::col((Suppliers, suppliers::Column::Email)),
                Expr::col((Orders, orders::Column::Id)),
                Expr::col((Orders, orders::Column::Status)),
                Expr::col((Orders, orders::Column::CreatedAt)),
            ])
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity))
                            .mul(Expr::col((OrderItems, order_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
                ]),
                Alias::new("total"),
            )
            .left_join(
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Orders, orders::Column::Id)),
            )
            .left_join(
                InventoryMouvements,
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .join(
                JoinType::Join,
                Suppliers,
                Expr::col((Suppliers, suppliers::Column::Id))
                    .equals((Orders, orders::Column::SupplierId)),
            )
            .cond_where(Expr::col((Orders, orders::Column::Id)).eq(id.clone()))
            .to_owned()
            .build(SqliteQueryBuilder);

        let order = SelectOrderDetails::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .one(db)
        .await?;

        match order {
            Some(order) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((OrderItems, order_items::Column::Price)),
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .from(OrderItems)
                    .join(
                        JoinType::Join,
                        InventoryMouvements,
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                            .equals((OrderItems, order_items::Column::InventoryId)),
                    )
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((InventoryMouvements, inventory_mouvements::Column::ProductId)),
                    )
                    .cond_where(Expr::col((OrderItems, order_items::Column::OrderId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectOrdersItems::find_by_statement(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    sql,
                    values,
                ))
                .all(db)
                .await?;

                let mut result = Vec::<JsonValue>::new();
                items.into_iter().for_each(|item| {
                    result.push(json!({
                        "price": item.price,
                        "quantity": item.quantity,
                        "name": item.name,
                    }));
                });

                Ok(json!({
                    "id": order.id,
                    "createdAt": order.created_at,
                    "status": order.status,
                    "total": order.total,
                    "supplier": json!({
                        "fullname": order.full_name,
                        "email": order.email,
                        "address":order.address,
                        "phoneNumber":order.phone_number,
                    }),
                    "items": result,
                }))
            }
            None => Err(DbErr::RecordNotFound(String::from("no order"))),
        }
    }
    //
    pub async fn list_invoices(db: &DbConn, args: ListArgs) -> Result<JsonValue, DbErr> {
        let count = Invoices::find()
            .apply_if(Some(args.search.clone()), |query, v| {
                query
                    .filter(Expr::col((Clients, clients::Column::FullName)).like(format!("{}%", v)))
            })
            .apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((Invoices, invoices::Column::Status)).eq(v))
            })
            .apply_if(args.created_at.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', invoices.created_at) = ?",
                    [v],
                ))
            })
            .join(JoinType::Join, invoices::Relation::Clients.def())
            .count(db)
            .await?;

        let (sql, values) = Query::select()
            .from(Invoices)
            .exprs([
                Expr::col((Invoices, invoices::Column::Id)),
                Expr::col((Invoices, invoices::Column::Status)),
                Expr::col((Invoices, invoices::Column::CreatedAt)),
                Expr::col((Invoices, invoices::Column::ClientId)),
                Expr::col((Invoices, invoices::Column::PaidAmount)),
                Expr::col((Clients, clients::Column::FullName)),
            ])
            .expr_as(
                Func::coalesce([
                    Func::count(Expr::col(inventory_mouvements::Column::Quantity)).into(),
                    Expr::val(0i64).into(),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
                ]),
                Alias::new("total"),
            )
            .left_join(
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                    .equals((Invoices, invoices::Column::Id)),
            )
            .left_join(
                InventoryMouvements,
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                    .equals((InvoiceItems, invoice_items::Column::InventoryId)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Invoices, invoices::Column::ClientId)),
            )
            .cond_where(
                Expr::col((Clients, clients::Column::FullName)).like(format!("{}%", args.search)),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((Invoices, invoices::Column::Status)).eq(args.status));
                },
                |_| {},
            )
            .conditions(
                args.created_at.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', invoices.created_at) = ?",
                        args.created_at,
                    ));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .order_by((Invoices, invoices::Column::CreatedAt), Order::Desc)
            .group_by_col((Invoices, invoices::Column::Id))
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectInvoices::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "id": row.id,
                "clientId": row.client_id,
                "paidAmount": row.paid_amount,
                "createdAt": row.created_at,
                "fullname": row.full_name,
                "status": row.status,
                "products": row.products,
                "total": row.total,
            }));
        });

        Ok(json!({
            "count": count,
            "invoices": result
        }))
    }
    pub async fn get_invoice(db: &DbConn, id: String) -> Result<JsonValue, DbErr> {
        let invoice = Invoices::find_by_id(id.clone())
            .find_also_related(Clients)
            .one(db)
            .await?;

        match invoice {
            Some(invoice) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((InvoiceItems, invoice_items::Column::Id)),
                        Expr::col((InvoiceItems, invoice_items::Column::InventoryId)),
                        Expr::col((InvoiceItems, invoice_items::Column::Price)),
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .expr_as(
                        Expr::col((Products, products::Column::Id)),
                        Alias::new("product_id"),
                    )
                    .from(InvoiceItems)
                    .join(
                        JoinType::Join,
                        InventoryMouvements,
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                            .equals((InvoiceItems, invoice_items::Column::InventoryId)),
                    )
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((InventoryMouvements, inventory_mouvements::Column::ProductId)),
                    )
                    .cond_where(Expr::col((InvoiceItems, invoice_items::Column::InvoiceId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectInvoicesItemsForUpdate::find_by_statement(
                    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
                )
                .all(db)
                .await?;

                let mut result = Vec::<JsonValue>::new();
                items.into_iter().for_each(|item| {
                    result.push(json!({
                        "id": item.id,
                        "inventory_id": item.inventory_id,
                        "product_id": item.product_id,
                        "price": item.price,
                        "quantity": item.quantity,
                        "name": item.name,
                    }));
                });

                Ok(json!({
                    "id": invoice.0.id,
                    "clientId": invoice.0.client_id,
                    "paidAmount": invoice.0.paid_amount,
                    "createdAt": invoice.0.created_at,
                    "status": invoice.0.status,
                    "fullname": invoice.1.unwrap().full_name,
                    "items": result,
                }))
            }
            None => Err(DbErr::RecordNotFound(String::from("no invoice"))),
        }
    }
    pub async fn list_invoice_products(db: &DbConn, id: String) -> Result<Vec<JsonValue>, DbErr> {
        let invoice_products = InvoiceItems::find()
            .select_only()
            .columns([invoice_items::Column::Price])
            .exprs([
                Expr::col((Products, products::Column::Name)),
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
            ])
            .join(
                JoinType::Join,
                invoice_items::Relation::InventoryMouvements.def(),
            )
            .join(
                JoinType::Join,
                inventory_mouvements::Relation::Products.def(),
            )
            .filter(Expr::col((InvoiceItems, invoice_items::Column::InvoiceId)).eq(id))
            .into_json()
            .all(db)
            .await?;

        Ok(invoice_products)
    }
    pub async fn get_invoice_details(db: &DbConn, id: String) -> Result<JsonValue, DbErr> {
        let (sql, values) = Query::select()
            .from(Invoices)
            .exprs([
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Invoices, invoices::Column::Id)),
                Expr::col((Invoices, invoices::Column::Status)),
                Expr::col((Invoices, invoices::Column::PaidAmount)),
                Expr::col((Invoices, invoices::Column::CreatedAt)),
            ])
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
                ]),
                Alias::new("total"),
            )
            .left_join(
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                    .equals((Invoices, invoices::Column::Id)),
            )
            .left_join(
                InventoryMouvements,
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                    .equals((InvoiceItems, invoice_items::Column::InventoryId)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Invoices, invoices::Column::ClientId)),
            )
            .cond_where(Expr::col((Invoices, invoices::Column::Id)).eq(id.clone()))
            .to_owned()
            .build(SqliteQueryBuilder);

        let invoice = SelectInvoiceDetails::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .one(db)
        .await?;

        match invoice {
            Some(invoice) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((InvoiceItems, invoice_items::Column::Price)),
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .from(InvoiceItems)
                    .join(
                        JoinType::Join,
                        InventoryMouvements,
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                            .equals((InvoiceItems, invoice_items::Column::InventoryId)),
                    )
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((InventoryMouvements, inventory_mouvements::Column::ProductId)),
                    )
                    .cond_where(Expr::col((InvoiceItems, invoice_items::Column::InvoiceId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectInvoicesItems::find_by_statement(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    sql,
                    values,
                ))
                .all(db)
                .await?;

                let mut result = Vec::<JsonValue>::new();
                items.into_iter().for_each(|item| {
                    result.push(json!({
                        "price": item.price,
                        "quantity": item.quantity,
                        "name": item.name,
                    }));
                });

                Ok(json!({
                    "id": invoice.id,
                    "paidAmount": invoice.paid_amount,
                    "createdAt": invoice.created_at,
                    "status": invoice.status,
                    "total": invoice.total,
                    "client": json!({
                        "fullname": invoice.full_name,
                        "email": invoice.email,
                        "address":invoice.address,
                        "phoneNumber":invoice.phone_number,
                    }),
                    "items": result,
                }))
            }
            None => Err(DbErr::RecordNotFound(String::from("no invoice"))),
        }
    }

    //
    pub async fn list_inventory(db: &DbConn, args: ListArgs) -> Result<JsonValue, DbErr> {
        let count = InventoryMouvements::find()
            .join(
                JoinType::Join,
                inventory_mouvements::Relation::Products.def(),
            )
            .join(
                JoinType::LeftJoin,
                inventory_mouvements::Relation::OrderItems.def(),
            )
            .join(
                JoinType::LeftJoin,
                inventory_mouvements::Relation::InvoiceItems.def(),
            )
            .join(JoinType::LeftJoin, invoice_items::Relation::Invoices.def())
            .join(JoinType::LeftJoin, order_items::Relation::Orders.def())
            .filter(
                Cond::all()
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Invoices, invoices::Column::CreatedAt)).into(),
                            Expr::col((Orders, orders::Column::CreatedAt)).into(),
                        ]))
                        .is_not_null(),
                    )
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Invoices, invoices::Column::Status)).into(),
                            Expr::col((Orders, orders::Column::Status)).into(),
                        ]))
                        .eq("CANCELED")
                        .not(),
                    ),
            )
            .apply_if(Some(args.search.clone()), |query, v| {
                query.filter(Expr::col((Products, products::Column::Name)).like(format!("{}%", v)))
            })
            .apply_if(args.status.clone(), |query, v| {
                query.filter(
                    Expr::col((InventoryMouvements, inventory_mouvements::Column::MvmType)).eq(v),
                )
            })
            .apply_if(args.created_at.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', COALESCE(invoices.created_at, orders.created_at)) = ?",
                    [v],
                ))
            })
            .count(db)
            .await?;
        //
        let (sql, values) = Query::select()
            .from(InventoryMouvements)
            .exprs([
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id)),
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
                Expr::col((Products, products::Column::Name)),
                Expr::col((InventoryMouvements, inventory_mouvements::Column::MvmType)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::col((Invoices, invoices::Column::CreatedAt)).into(),
                    Expr::col((Orders, orders::Column::CreatedAt)).into(),
                ]),
                Alias::new("created_at"),
            )
            .expr_as(
                Func::coalesce([
                    Expr::col((OrderItems, order_items::Column::Price)).into(),
                    Expr::col((InvoiceItems, invoice_items::Column::Price)).into(),
                ]),
                Alias::new("price"),
            )
            .join(
                JoinType::Join,
                Products,
                Expr::col((Products, products::Column::Id))
                    .equals((InventoryMouvements, inventory_mouvements::Column::ProductId)),
            )
            .join(
                JoinType::LeftJoin,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::InventoryId))
                    .equals((InventoryMouvements, inventory_mouvements::Column::Id)),
            )
            .join(
                JoinType::LeftJoin,
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InventoryId))
                    .equals((InventoryMouvements, inventory_mouvements::Column::Id)),
            )
            .join(
                JoinType::LeftJoin,
                Orders,
                Expr::col((Orders, orders::Column::Id))
                    .equals((OrderItems, order_items::Column::OrderId)),
            )
            .join(
                JoinType::LeftJoin,
                Invoices,
                Expr::col((Invoices, invoices::Column::Id))
                    .equals((InvoiceItems, invoice_items::Column::InvoiceId)),
            ).cond_where(
                Cond::all()
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Invoices, invoices::Column::CreatedAt)).into(),
                            Expr::col((Orders, orders::Column::CreatedAt)).into(),
                        ]))
                        .is_not_null(),
                    )
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Invoices, invoices::Column::Status)).into(),
                            Expr::col((Orders, orders::Column::Status)).into(),
                        ]))
                        .eq("CANCELED")
                        .not(),
                    )
            )
            .and_where(
                Expr::col((Products, products::Column::Name)).like(format!("{}%", args.search)),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((InventoryMouvements, inventory_mouvements::Column::MvmType)).eq(args.status));
                },
                |_| {},
            )
            .conditions(
                args.created_at.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', COALESCE(invoices.created_at, orders.created_at)) = ?",
                        args.created_at,
                    ));
                },
                |_| {},
            )
            .order_by_expr(
                Func::coalesce([
                    Expr::col((Invoices, invoices::Column::CreatedAt)).into(),
                    Expr::col((Orders, orders::Column::CreatedAt)).into(),
                ])
                .into(),
                Order::Desc,
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .to_owned()
            .build(SqliteQueryBuilder);
        //
        let res = SelectInventory::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "id": row.id,
                "name": row.name,
                "price": row.price,
                "createdAt": row.created_at,
                "quantity": row.quantity,
                "mvmType": row.mvm_type,
            }));
        });

        Ok(json!({
            "count": count,
            "inventory": result
        }))
    }
    //
    pub async fn list_mvm_stats(db: &DbConn) -> Result<Vec<JsonValue>, DbErr> {
        let (sql, values) = Query::select()
            .from(InventoryMouvements)
            .exprs([Expr::col((
                InventoryMouvements,
                inventory_mouvements::Column::MvmType,
            ))])
            .expr_as(
                Func::sum(Expr::col((
                    InventoryMouvements,
                    inventory_mouvements::Column::Quantity,
                ))),
                Alias::new("quantity"),
            )
            .expr_as(
                Expr::cust("strftime('%Y-%m-01', COALESCE(invoices.created_at, orders.created_at))"),
                Alias::new("created_at"),
            )
            .expr_as(
                Func::sum(
                    Expr::expr(Func::coalesce([
                        Expr::col((OrderItems, order_items::Column::Price)).into(),
                        Expr::col((InvoiceItems, invoice_items::Column::Price)).into(),
                    ])).mul(
                        Expr::col((
                        InventoryMouvements,
                        inventory_mouvements::Column::Quantity,
                    ))
                )),
                Alias::new("price"),
            )
            .join(
                JoinType::Join,
                Products,
                Expr::col((Products, products::Column::Id))
                    .equals((InventoryMouvements, inventory_mouvements::Column::ProductId)),
            )
            .join(
                JoinType::LeftJoin,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::InventoryId))
                    .equals((InventoryMouvements, inventory_mouvements::Column::Id)),
            )
            .join(
                JoinType::LeftJoin,
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InventoryId))
                    .equals((InventoryMouvements, inventory_mouvements::Column::Id)),
            )
            .join(
                JoinType::LeftJoin,
                Orders,
                Expr::col((Orders, orders::Column::Id))
                    .equals((OrderItems, order_items::Column::OrderId)),
            )
            .join(
                JoinType::LeftJoin,
                Invoices,
                Expr::col((Invoices, invoices::Column::Id))
                    .equals((InvoiceItems, invoice_items::Column::InvoiceId)),
            )
            .cond_where(
                Cond::all()
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Invoices, invoices::Column::CreatedAt)).into(),
                            Expr::col((Orders, orders::Column::CreatedAt)).into(),
                        ]))
                        .is_not_null(),
                    )
                    .add(
                        Expr::expr(Func::coalesce([
                            Expr::col((Invoices, invoices::Column::Status)).into(),
                            Expr::col((Orders, orders::Column::Status)).into(),
                        ]))
                        .eq("CANCELED")
                        .not(),
                    ).add(
                Expr::cust("COALESCE(invoices.created_at, orders.created_at) >= DATETIME('now', '-3 month')"),
                    ),
            )
            .add_group_by([
                Expr::cust("strftime('%Y-%m', COALESCE(invoices.created_at, orders.created_at))"),
                Expr::col((InventoryMouvements, inventory_mouvements::Column::MvmType)).into(),
            ])
            .to_owned()
            .build(SqliteQueryBuilder);
        //
        let res = SelectMvm::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "price": row.price,
                "createdAt": row.created_at,
                "quantity": row.quantity,
                "mvmType": row.mvm_type,
            }));
        });

        Ok(result)
    }
    pub async fn list_top_clients(db: &DbConn) -> Result<Vec<JsonValue>, DbErr> {
        let (sql, values) =
            Query::select()
                .from(Clients)
                .column((Clients, clients::Column::FullName))
                .expr_as(
                    Func::sum(Expr::col((
                        InventoryMouvements,
                        inventory_mouvements::Column::Quantity,
                    ))),
                    Alias::new("quantity"),
                )
                .expr_as(
                    Func::sum(Expr::col((InvoiceItems, invoice_items::Column::Price)).mul(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
                    )),
                    Alias::new("price"),
                )
                .join(
                    JoinType::Join,
                    Invoices,
                    Expr::col((Invoices, invoices::Column::ClientId))
                        .equals((Clients, clients::Column::Id)),
                )
                .join(
                    JoinType::Join,
                    InvoiceItems,
                    Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                        .equals((Invoices, invoices::Column::Id)),
                )
                .join(
                    JoinType::Join,
                    InventoryMouvements,
                    Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                        .equals((InvoiceItems, invoice_items::Column::InventoryId)),
                )
                .cond_where(
                    Cond::all().add(
                        Expr::expr(Expr::col((Invoices, invoices::Column::Status)))
                            .eq("CANCELED")
                            .not(),
                    ),
                )
                .add_group_by([Expr::col((Clients, clients::Column::Id)).into()])
                .order_by_expr(
                    Func::sum(Expr::col((InvoiceItems, invoice_items::Column::Price)).mul(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity)),
                    ))
                    .into(),
                    Order::Desc,
                )
                .limit(5)
                .to_owned()
                .build(SqliteQueryBuilder);
        //
        let res = SelectTops::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "price": row.price,
                "Fullname": row.full_name,
                "quantity": row.quantity,
            }));
        });

        Ok(result)
    }
    pub async fn list_top_suppliers(db: &DbConn) -> Result<Vec<JsonValue>, DbErr> {
        let (sql, values) = Query::select()
            .from(Suppliers)
            .column((Suppliers, suppliers::Column::FullName))
            .expr_as(
                Func::sum(Expr::col((
                    InventoryMouvements,
                    inventory_mouvements::Column::Quantity,
                ))),
                Alias::new("quantity"),
            )
            .expr_as(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price)).mul(Expr::col((
                        InventoryMouvements,
                        inventory_mouvements::Column::Quantity,
                    ))),
                ),
                Alias::new("price"),
            )
            .join(
                JoinType::Join,
                Orders,
                Expr::col((Orders, orders::Column::SupplierId))
                    .equals((Suppliers, suppliers::Column::Id)),
            )
            .join(
                JoinType::Join,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Orders, orders::Column::Id)),
            )
            .join(
                JoinType::Join,
                InventoryMouvements,
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .cond_where(
                Cond::all().add(
                    Expr::expr(Expr::col((Orders, orders::Column::Status)))
                        .eq("CANCELED")
                        .not(),
                ),
            )
            .add_group_by([Expr::col((Suppliers, suppliers::Column::Id)).into()])
            .order_by_expr(
                Func::sum(
                    Expr::col((OrderItems, order_items::Column::Price)).mul(Expr::col((
                        InventoryMouvements,
                        inventory_mouvements::Column::Quantity,
                    ))),
                )
                .into(),
                Order::Desc,
            )
            .limit(5)
            .to_owned()
            .build(SqliteQueryBuilder);
        //
        let res = SelectTops::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        let mut result = Vec::<JsonValue>::new();
        res.into_iter().for_each(|row| {
            result.push(json!({
                "price": row.price,
                "Fullname": row.full_name,
                "quantity": row.quantity,
            }));
        });

        Ok(result)
    }
    pub async fn list_status_count(db: &DbConn) -> Result<JsonValue, DbErr> {
        let (order_sql, order_values) = Query::select()
            .from(Orders)
            .column(orders::Column::Status)
            .expr_as(
                Func::count(Expr::col((Orders, orders::Column::Id))),
                Alias::new("status_count"),
            )
            .group_by_col(orders::Column::Status)
            .to_owned()
            .build(SqliteQueryBuilder);

        let (invoice_sql, invoice_values) = Query::select()
            .from(Invoices)
            .column(invoices::Column::Status)
            .expr_as(
                Func::count(Expr::col((Invoices, invoices::Column::Id))),
                Alias::new("status_count"),
            )
            .group_by_col(invoices::Column::Status)
            .to_owned()
            .build(SqliteQueryBuilder);

        let order_res = SelectStatusCount::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            order_sql,
            order_values,
        ))
        .all(db)
        .await?;

        let invoice_res = SelectStatusCount::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            invoice_sql,
            invoice_values,
        ))
        .all(db)
        .await?;

        Ok(json!({
            "orders": order_res,
            "invoices": invoice_res,
        }))
    }
    pub async fn list_revenue(db: &DbConn) -> Result<JsonValue, DbErr> {
        let (sql, values) = Query::select()
            .from(Invoices)
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0f64).into(),
                ]),
                Alias::new("current_revenue"),
            )
            .expr_as(
                Expr::expr(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(Invoices)
                            .expr(Func::coalesce([
                                Func::sum(
                                    Expr::col((
                                        InventoryMouvements,
                                        inventory_mouvements::Column::Quantity,
                                    ))
                                    .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                                )
                                .into(),
                                Expr::val(0.0f64).into(),
                            ]))
                            .join(
                                JoinType::Join,
                                InvoiceItems,
                                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                                    .equals((Invoices, invoices::Column::Id)),
                            )
                            .join(
                                JoinType::Join,
                                InventoryMouvements,
                                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                                    .equals((InvoiceItems, invoice_items::Column::InventoryId)),
                            )
                            .and_where(Expr::col((Invoices, invoices::Column::Status)).eq("PAID"))
                            .and_where(Expr::cust(
                                "invoices.created_at < strftime('%Y-%m-01', CURRENT_DATE)",
                            ))
                            .to_owned(),
                    )),
                )),
                Alias::new("last_month_revenue"),
            )
            .join(
                JoinType::Join,
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                    .equals((Invoices, invoices::Column::Id)),
            )
            .join(
                JoinType::Join,
                InventoryMouvements,
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                    .equals((InvoiceItems, invoice_items::Column::InventoryId)),
            )
            .and_where(Expr::col((Invoices, invoices::Column::Status)).eq("PAID"))
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectRevenue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(json!({
            "revenue": res.into_iter().map(|r| json!({
                "currentRevenue": r.current_revenue,
                "lastMonthRevenue": r.last_month_revenue,
            })).collect::<Vec<JsonValue>>()
        }))
    }
    pub async fn list_expenses(db: &DbConn) -> Result<JsonValue, DbErr> {
        let (sql, values) = Query::select()
            .from(Orders)
            .expr_as(
                Func::coalesce([
                    Func::sum(
                        Expr::col((InventoryMouvements, inventory_mouvements::Column::Quantity))
                            .mul(Expr::col((OrderItems, order_items::Column::Price))),
                    )
                    .into(),
                    Expr::val(0.0).into(),
                ]),
                Alias::new("current_expenses"),
            )
            .expr_as(
                Expr::expr(SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .from(Orders)
                            .expr(Func::coalesce([
                                Func::sum(
                                    Expr::col((
                                        InventoryMouvements,
                                        inventory_mouvements::Column::Quantity,
                                    ))
                                    .mul(Expr::col((OrderItems, order_items::Column::Price))),
                                )
                                .into(),
                                Expr::val(0.0).into(),
                            ]))
                            .join(
                                JoinType::Join,
                                OrderItems,
                                Expr::col((OrderItems, order_items::Column::OrderId))
                                    .equals((Orders, orders::Column::Id)),
                            )
                            .join(
                                JoinType::Join,
                                InventoryMouvements,
                                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                                    .equals((OrderItems, order_items::Column::InventoryId)),
                            )
                            .and_where(Expr::col((Orders, orders::Column::Status)).eq("DELIVERED"))
                            .and_where(Expr::cust(
                                "orders.created_at < strftime('%Y-%m-01', CURRENT_DATE)",
                            ))
                            .to_owned(),
                    )),
                )),
                Alias::new("last_month_expenses"),
            )
            .join(
                JoinType::Join,
                OrderItems,
                Expr::col((OrderItems, order_items::Column::OrderId))
                    .equals((Orders, orders::Column::Id)),
            )
            .join(
                JoinType::Join,
                InventoryMouvements,
                Expr::col((InventoryMouvements, inventory_mouvements::Column::Id))
                    .equals((OrderItems, order_items::Column::InventoryId)),
            )
            .and_where(Expr::col((Orders, orders::Column::Status)).eq("DELIVERED"))
            .to_owned()
            .build(SqliteQueryBuilder);

        let res = SelectExpenses::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(json!({
            "expenses": res.into_iter().map(|r| json!({
                "currentExpenses": r.current_expenses,
                "lastMonthExpenses": r.last_month_expenses,
            })).collect::<Vec<JsonValue>>()
        }))
    }
}
