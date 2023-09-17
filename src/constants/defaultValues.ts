export const CLIENT_CREATE = {
  fullname: String(),
  phone: String(),
  email: String(),
  address: String(),
  image: String(),
};

export const CLIENT_UPDATE = {
  id: undefined,
  fullname: undefined,
  email: undefined,
  phone: undefined,
  address: undefined,
};

export const INVENTORY_CREATE = {
  productId: 0,
  quantity: 0,
  model: "IN",
};

export const INVOICE_CREATE = {
  client_id: 0,
  invoice_items: [],
  status: String(),
};

export const INVOICE_UPDATE = {
  id: undefined,
  total: undefined,
  client_id: undefined,
  invoice_items: [],
};

export const ORDER_CREATE = {
  status: String(),
  seller_id: undefined,
  order_items: [],
};

export const ORDER_UPDATE = {
  id: undefined,
  status: undefined,
  seller_id: undefined,
  order_items: [],
};

export const PRODUCT_CREATE = {
  name: String(),
  price: 0,
  quantity: 0,
  description: String(),
  tva: 0,
  image: String(),
};

export const PRODUCT_UPDATE = {
  id: undefined,
  name: undefined,
  price: undefined,
  quantity: undefined,
  description: undefined,
  tva: undefined,
};

export const SELLER_CREATE = {
  name: String(),
  email: String(),
  phone: String(),
  address: String(),
};

export const SELLER_UPDATE = {
  id: undefined,
  name: undefined,
  email: undefined,
  phone: undefined,
  address: undefined,
  image: undefined,
};
