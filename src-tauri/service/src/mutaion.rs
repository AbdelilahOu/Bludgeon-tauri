use ::entity::prelude::*;
use sea_orm::*;

use crate::{models::*, Order};
pub struct MutationsService;

impl MutationsService {
    pub async fn create_product(db: &DbConn, product: NewProduct) -> Result<String, DbErr> {
        let product = ProductActiveModel {
            name: ActiveValue::Set(product.name),
            price: ActiveValue::Set(product.price),
            image: ActiveValue::Set(product.image),
            description: ActiveValue::Set(product.description),
            min_quantity: ActiveValue::Set(product.min_quantity),
            ..Default::default()
        };
        match product.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_product(db: &DbConn, product: Product) -> Result<(), DbErr> {
        let product_model = Products::find_by_id(product.id).one(db).await?;
        let mut product_active: ProductActiveModel = product_model.unwrap().into();
        product_active.name = ActiveValue::Set(product.name);
        product_active.price = ActiveValue::Set(product.price);
        product_active.image = ActiveValue::Set(product.image);
        product_active.description = ActiveValue::Set(product.description);
        product_active.min_quantity = ActiveValue::Set(product.min_quantity);
        match product_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_product(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let product_model = Products::find_by_id(id).one(db).await?;
        match product_model {
            Some(product_model) => {
                let product = product_model.delete(db).await?;
                Ok(product.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_client(db: &DbConn, client: NewClient) -> Result<String, DbErr> {
        let client = ClientActiveModel {
            full_name: ActiveValue::Set(client.full_name),
            email: ActiveValue::Set(client.email),
            phone_number: ActiveValue::Set(client.phone_number),
            address: ActiveValue::Set(client.address),
            image: ActiveValue::Set(client.image),
            ..Default::default()
        };
        match client.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_client(db: &DbConn, client: Client) -> Result<(), DbErr> {
        let client_model = Clients::find_by_id(client.id).one(db).await?;
        let mut client_active: ClientActiveModel = client_model.unwrap().into();
        client_active.full_name = ActiveValue::Set(client.full_name);
        client_active.email = ActiveValue::Set(client.email);
        client_active.phone_number = ActiveValue::Set(client.phone_number);
        client_active.address = ActiveValue::Set(client.address);
        client_active.image = ActiveValue::Set(client.image);
        match client_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_client(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let client_model = Clients::find_by_id(id).one(db).await?;
        match client_model {
            Some(client_model) => {
                let client = client_model.delete(db).await?;
                Ok(client.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_supplier(db: &DbConn, supplier: NewSupplier) -> Result<String, DbErr> {
        let supplier = SupplierActiveModel {
            full_name: ActiveValue::Set(supplier.full_name),
            email: ActiveValue::Set(supplier.email),
            phone_number: ActiveValue::Set(supplier.phone_number),
            address: ActiveValue::Set(supplier.address),
            image: ActiveValue::Set(supplier.image),
            ..Default::default()
        };
        match supplier.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_supplier(db: &DbConn, supplier: Supplier) -> Result<(), DbErr> {
        let supplier_model = Suppliers::find_by_id(supplier.id).one(db).await?;
        let mut supplier_active: SupplierActiveModel = supplier_model.unwrap().into();
        supplier_active.full_name = ActiveValue::Set(supplier.full_name);
        supplier_active.email = ActiveValue::Set(supplier.email);
        supplier_active.phone_number = ActiveValue::Set(supplier.phone_number);
        supplier_active.address = ActiveValue::Set(supplier.address);
        supplier_active.image = ActiveValue::Set(supplier.image);
        match supplier_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_supplier(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let supplier_model = Suppliers::find_by_id(id).one(db).await?;
        match supplier_model {
            Some(supplier_model) => {
                let supplier = supplier_model.delete(db).await?;
                Ok(supplier.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_inv_mvm(db: &DbConn, mvm: NewInventory) -> Result<String, DbErr> {
        let in_mvm = InventoryActiveModel {
            mvm_type: ActiveValue::Set(mvm.mvm_type),
            quantity: ActiveValue::Set(mvm.quantity),
            product_id: ActiveValue::Set(mvm.product_id),
            ..Default::default()
        };
        match in_mvm.insert(db).await {
            Ok(im) => Ok(im.id),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_inv_mvm(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let city_model = InventoryMouvements::find_by_id(id).one(db).await?;
        match city_model {
            Some(city_model) => {
                let city = city_model.delete(db).await?;
                Ok(city.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_inv_mvm(db: &DbConn, mvm: Inventory) -> Result<(), DbErr> {
        let inventory_model = InventoryMouvements::find_by_id(mvm.id).one(db).await?;
        let mut inventory_active: InventoryActiveModel = inventory_model.unwrap().into();
        inventory_active.mvm_type = ActiveValue::Set(mvm.mvm_type);
        inventory_active.quantity = ActiveValue::Set(mvm.quantity);
        inventory_active.product_id = ActiveValue::Set(mvm.product_id);
        match inventory_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    //
    pub async fn create_order(db: &DbConn, order: NewOrder) -> Result<String, DbErr> {
        let order = OrderActiveModel {
            supplier_id: ActiveValue::Set(order.supplier_id),
            status: ActiveValue::Set(order.status),
            ..Default::default()
        };
        match order.insert(db).await {
            Ok(o) => Ok(o.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_order(db: &DbConn, order: Order) -> Result<(), DbErr> {
        let order_model = Orders::find_by_id(order.id).one(db).await?;
        let mut order_active: OrderActiveModel = order_model.unwrap().into();
        order_active.supplier_id = ActiveValue::Set(order.supplier_id);
        order_active.status = ActiveValue::Set(order.status);
        match order_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_order(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let order_model = Orders::find_by_id(id).one(db).await?;
        match order_model {
            Some(order_model) => {
                let order = order_model.delete(db).await?;
                Ok(order.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_order_item(db: &DbConn, item: NewOrderItem) -> Result<String, DbErr> {
        let order_item = OrderItemActiveModel {
            order_id: ActiveValue::Set(item.order_id),
            inventory_id: ActiveValue::Set(item.inventory_id),
            price: ActiveValue::Set(item.price),
            ..Default::default()
        };
        match order_item.insert(db).await {
            Ok(o) => Ok(o.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_order_item(db: &DbConn, item: OrderItem) -> Result<(), DbErr> {
        let order_item_model = OrderItems::find_by_id(item.id).one(db).await?;
        let mut order_item_active: OrderItemActiveModel = order_item_model.unwrap().into();
        order_item_active.order_id = ActiveValue::Set(item.order_id);
        order_item_active.inventory_id = ActiveValue::Set(item.inventory_id);
        order_item_active.price = ActiveValue::Set(item.price);
        match order_item_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_order_item(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let order_item_model = OrderItems::find_by_id(id).one(db).await?;
        match order_item_model {
            Some(order_item_model) => {
                let order_item = order_item_model.delete(db).await?;
                Ok(order_item.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    //
    pub async fn create_invoice(db: &DbConn, invoice: NewInvoice) -> Result<String, DbErr> {
        let invoice = InvoiceActiveModel {
            client_id: ActiveValue::Set(invoice.client_id),
            status: ActiveValue::Set(invoice.status),
            paid_amount: ActiveValue::Set(invoice.paid_amount),
            ..Default::default()
        };
        match invoice.insert(db).await {
            Ok(o) => Ok(o.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_invoice(db: &DbConn, invoice: Invoice) -> Result<(), DbErr> {
        let invoice_model = Invoices::find_by_id(invoice.id).one(db).await?;
        let mut invoice_active: InvoiceActiveModel = invoice_model.unwrap().into();
        invoice_active.client_id = ActiveValue::Set(invoice.client_id);
        invoice_active.status = ActiveValue::Set(invoice.status);
        invoice_active.paid_amount = ActiveValue::Set(invoice.paid_amount);
        match invoice_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_invoice(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let invoice_model = Invoices::find_by_id(id).one(db).await?;
        match invoice_model {
            Some(invoice_model) => {
                let invoice = invoice_model.delete(db).await?;
                Ok(invoice.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_invoice_item(db: &DbConn, item: NewInvoiceItem) -> Result<String, DbErr> {
        let invoice_item = InvoiceItemActiveModel {
            invoice_id: ActiveValue::Set(item.invoice_id),
            inventory_id: ActiveValue::Set(item.inventory_id),
            price: ActiveValue::Set(item.price),
            ..Default::default()
        };
        match invoice_item.insert(db).await {
            Ok(o) => Ok(o.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_invoice_item(db: &DbConn, item: InvoiceItem) -> Result<(), DbErr> {
        let invoice_item_model = InvoiceItems::find_by_id(item.id).one(db).await?;
        let mut invoice_item_active: InvoiceItemActiveModel = invoice_item_model.unwrap().into();
        invoice_item_active.invoice_id = ActiveValue::Set(item.invoice_id);
        invoice_item_active.inventory_id = ActiveValue::Set(item.inventory_id);
        invoice_item_active.price = ActiveValue::Set(item.price);
        match invoice_item_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_invoice_item(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let invoice_item_model = InvoiceItems::find_by_id(id).one(db).await?;
        match invoice_item_model {
            Some(invoice_item_model) => {
                let invoice_item = invoice_item_model.delete(db).await?;
                Ok(invoice_item.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
}
