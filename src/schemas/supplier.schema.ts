import { z } from "zod";

export const CreateSupplierSchema = z.object({
  id: z.string().optional(),
  fullname: z.string().min(2).max(50),
  email: z.string().optional(),
  phoneNumber: z.string().optional(),
  address: z.string().optional(),
  image: z.string().optional(),
});

export type SupplierT = z.infer<typeof CreateSupplierSchema>;
