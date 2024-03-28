import { z } from "zod";

export const CreateProductSchema = z.object({
  id: z.string().optional(),
  name: z.string().min(2).max(50),
  price: z.number().min(0),
  image: z.string().optional(),
  description: z.string().optional(),
  minQuantity: z.number().min(0),
  stock: z.number().optional(),
  createdAt: z.number().optional(),
});

export type ProductT = z.infer<typeof CreateProductSchema>;
