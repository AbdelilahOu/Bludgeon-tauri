# Inventory and Invoicing Desktop App

This project is a desktop application for managing inventory and generating invoices, built using the following technologies:

- Frontend: Vue.js (https://vuejs.org/)
- Framework: Tauri (https://tauri.app/) with Rust backend (https://www.rust-lang.org/)
- Database: SQLite (https://www.sqlite.org/)
- ORM: seaORM (https://github.com/SeaQL/sea-orm)
- PDF Generation: pdf-lib (https://www.npmjs.com/package/pdf-lib?activeTab=readme)

## Features

- Manage inventory items:
  - Add, edit, and delete products.
  - Track stock levels and reorder points.
  - View product details, including images.
- Create and manage invoices:
  - Generate invoices.
- Order processing:
  - Create and manage orders.
  - Track order status.
  - Generate PDF reports for orders and invoices (using pdf-lib).
