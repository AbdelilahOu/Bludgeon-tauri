SELECT orders.id, orders.status, orders.created_at, orders.supplier_id, suppliers.full_name, COALESCE(COUNT(quantity), ?) AS products, COALESCE(SUM(inventory_mouvements.quantity * order_items.price), ?) AS total FROM orders LEFT JOIN order_items ON order_items.order_id = orders.id LEFT JOIN inventory_mouvements ON inventory_mouvements.id = order_items.inventory_id JOIN suppliers ON suppliers.id = orders.supplier_id WHERE suppliers.full_name LIKE ? AND (strftime('%Y-%m-%d', orders.created_at) = $1) GROUP BY orders.id ORDER BY orders.created_at DESC LIMIT ? OFFSET ?"