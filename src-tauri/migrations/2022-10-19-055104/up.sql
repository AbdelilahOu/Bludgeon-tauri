CREATE TABLE IF NOT EXISTS users  (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  email TEXT NOT NULL,
  role TEXT NOT NULL DEFAULT "user"
);

CREATE TABLE IF NOT EXISTS clients (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  fullname TEXT NOT NULL,
  phone TEXT NOT NULL DEFAULT "",
  email TEXT NOT NULL DEFAULT "",
  address TEXT NOT NULL DEFAULT "",
  image TEXT NOT NULL DEFAULT ""
);

CREATE TABLE IF NOT EXISTS products (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  image TEXT NOT NULL DEFAULT "",
  description TEXT NOT NULL DEFAULT "",
  price REAL NOT NULL,
  tva REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS sellers (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  phone TEXT NOT NULL DEFAULT "",
  email TEXT NOT NULL DEFAULT "",
  address TEXT NOT NULL DEFAULT "",
  image TEXT NOT NULL DEFAULT ""
);

CREATE TABLE IF NOT EXISTS invoices (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  status TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  client_id INTEGER NOT NULL,
  CONSTRAINT invoices_client_id_fkey FOREIGN KEY (client_id) REFERENCES clients (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS invoice_items (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  product_id INTEGER NOT NULL,
  invoice_id INTEGER NOT NULL,
  quantity BIGINT NOT NULL,
  inventory_id INTEGER NOT NULL,
  CONSTRAINT invoice_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT invoice_items_invoice_id_fkey FOREIGN KEY (invoice_id) REFERENCES invoices (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT invoice_items_inventory_id_fkey FOREIGN KEY (inventory_id) REFERENCES inventory_mouvements (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS orders (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  status TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  seller_id INTEGER NOT NULL,
  CONSTRAINT orders_seller_id_fkey FOREIGN KEY (seller_id) REFERENCES sellers (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS order_items (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  product_id INTEGER NOT NULL,
  price REAl,
  order_id INTEGER NOT NULL,
  inventory_id INTEGER NOT NULL,
  quantity BIGINT NOT NULL,
  CONSTRAINT order_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT order_items_order_id_fkey FOREIGN KEY (order_id) REFERENCES orders (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT order_items_inventory_id_fkey FOREIGN KEY (inventory_id) REFERENCES inventory_mouvements (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS inventory_mouvements (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  model TEXT NOT NULL,
  quantity BIGINT NOT NULL,
  product_id INTEGER NOT NULL,
  CONSTRAINT inventory_mouvements_product_id_fkey FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS order_items_inventory_id_key ON order_items (inventory_id);
CREATE UNIQUE INDEX IF NOT EXISTS order_items_id_key ON order_items (id);
CREATE UNIQUE INDEX IF NOT EXISTS invoice_items_inventory_id_key ON invoice_items (inventory_id);
CREATE UNIQUE INDEX IF NOT EXISTS products_name_key ON products (name);
CREATE UNIQUE INDEX IF NOT EXISTS clients_fullname_key ON clients (fullname);
CREATE UNIQUE INDEX IF NOT EXISTS sellers_name_key ON sellers (name);
  