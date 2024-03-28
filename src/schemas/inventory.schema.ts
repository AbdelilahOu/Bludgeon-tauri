import { z } from "zod";

export const CreateInventorySchema = z.object({
  id: z.string(),
  name: z.string(),
  mvmType: z.enum(["IN", "OUT"]),
  quantity: z.number(),
  price: z.number(),
  createdAt: z.string(),
});

export type InventoryT = z.infer<typeof CreateInventorySchema>;
