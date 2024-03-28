import { z } from "zod";

export const CreateInvoiceSchema = z.object({
  id: z.string().optional(),
  status: z.string().min(2).max(50),
  paidAmount: z.number(),
  clientId: z.string().optional(),
});

export type InvoiceT = z.infer<typeof CreateInvoiceSchema> & {
  fullname?: string;
  products?: number;
  total?: number;
  createdAt?: string;
};

export type InvoiceForUpdateT = {
  id: string;
  clientId: string;
  fullname: string;
  paidAmount: number;
  createdAt: string;
  status: string;
  items: {
    id?: string;
    inventory_id?: string;
    name?: string;
    product_id?: string;
    quantity?: number;
    price?: number;
  }[];
};

export type InvoiceForCreateT = {
  clientId: string;
  status: string;
  paidAmount: number;
  items: {
    product_id?: string;
    quantity?: number;
    price?: number;
  }[];
};

export type InvoiceProductT = {
  price: number;
  name: string;
  quantity: number;
};
