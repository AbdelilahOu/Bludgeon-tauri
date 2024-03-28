<script setup lang="ts">
import { useI18n } from "vue-i18n";
import UiPagination from "./ui/UiPagination.vue";
import { RouterLink } from "vue-router";
import { store } from "@/store";
import type { OrderProductT, OrderT } from "@/schemas/order.schema";
import { Badge } from "./ui/badge";
import { Button } from "./ui/button";
import { cn } from "@/utils/shadcn";
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover";
import { invoke } from "@tauri-apps/api";
import { error, info } from "tauri-plugin-log-api";
import { FilePenLine, Printer, Trash2 } from "lucide-vue-next";

const { updateQueryParams } = useUpdateRouteQueryParams();
const { t, d } = useI18n();

defineProps<{ orders: OrderT[]; orderProducts: OrderProductT[] }>();
defineEmits<{
  (e: "listOrderProducts", id?: string): void;
  (e: "cancelOrderProducts"): void;
}>();

const toggleThisOrders = (Order: OrderT, name: string) => {
  updateQueryParams({
    id: Order.id,
  });
  store.setters.updateStore({ key: "name", value: name });
  store.setters.updateStore({ key: "show", value: true });
};

const updateOrderStatus = async (order: any) => {
  try {
    await invoke("update_order", {
      order,
    });
    //
    info(`UPDATE ORDER STATUS: ${JSON.stringify(order)}`);
    // toggle refresh
    updateQueryParams({
      refresh: "refresh-update-" + Math.random() * 9999,
    });
  } catch (err: any) {
    error("UPDATE ORDER STATUS: " + err.error);
  }
};
</script>

<template>
  <div>
    <table>
      <thead>
        <tr>
          <th>{{ t("g.fields.fullname") }}</th>
          <th>{{ t("g.fields.items") }}</th>
          <th class="small">{{ t("g.fields.status") }}</th>
          <th>{{ t("g.fields.date") }}</th>
          <th>{{ t("g.fields.total") }}</th>
          <th class="small">{{ t("g.fields.actions") }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(order, index) in orders" v-fade="index" :key="order.id">
          <td class="p-2">
            <RouterLink
              class="font-medium"
              :to="{
                path: '/suppliers/' + order.supplierId,
              }"
            >
              {{ order.fullname }}
            </RouterLink>
          </td>
          <td class="p-2">
            <HoverCard v-if="order.products && order.products > 0">
              <HoverCardTrigger as-child>
                <Button
                  @mouseenter.passive="$emit('listOrderProducts', order.id)"
                  @mouseleave.passive="$emit('cancelOrderProducts')"
                  size="sm"
                  variant="link"
                  class="underline px-0"
                >
                  {{ t("g.plrz.p", { n: order.products }) }}
                </Button>
              </HoverCardTrigger>
              <HoverCardContent class="min-w-[13rem] p-2">
                <table class="w-full not-default">
                  <thead>
                    <tr>
                      <th v-for="index in 3" :key="index"></th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr
                      v-for="(orderProduct, index) in orderProducts"
                      :key="index"
                      class="space-y-1 text-sm flex justify-between w-full items-center"
                    >
                      <td class="underline w-1/2">{{ orderProduct.name }}</td>
                      <td class="w-1/4 text-end">
                        {{ orderProduct.price }} Dh
                      </td>
                      <td class="w-1/4 text-slate-700 text-end">
                        <i> x{{ orderProduct.quantity }} </i>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </HoverCardContent>
            </HoverCard>
            <template v-else>
              {{ t("g.plrz.p", { n: order.products }) }}
            </template>
          </td>
          <td class="p-2">
            <Popover>
              <PopoverTrigger as-child>
                <Badge
                  variant="outline"
                  :class="
                    cn(
                      'cursor-pointer whitespace-nowrap',
                      order?.status == 'CANCELED'
                        ? 'bg-red-100 border-red-500 text-red-900'
                        : order?.status == 'PENDING'
                          ? 'bg-yellow-100 border-yellow-500 text-yellow-900'
                          : order?.status == 'DELIVERED'
                            ? 'bg-green-100 border-green-500 text-green-900'
                            : '',
                    )
                  "
                >
                  {{ t(`g.status.${order.status.toLowerCase()}`) }}
                </Badge>
              </PopoverTrigger>
              <PopoverContent class="w-40 p-1 flex flex-col gap-1">
                <Button
                  type="button"
                  @click="
                    () =>
                      updateOrderStatus({
                        id: order.id,
                        supplier_id: order.supplierId,
                        status: 'DELIVERED',
                      })
                  "
                  variant="secondary"
                  size="sm"
                  class="border bg-green-100 w-full border-green-500 text-green-900"
                >
                  {{ t(`g.status.delivered`) }}
                </Button>
                <Button
                  type="button"
                  @click="
                    () =>
                      updateOrderStatus({
                        id: order.id,
                        supplier_id: order.supplierId,
                        status: 'PENDING',
                      })
                  "
                  variant="secondary"
                  size="sm"
                  class="border bg-yellow-100 w-full border-yellow-500 text-yellow-900"
                >
                  {{ t(`g.status.pending`) }}
                </Button>
                <Button
                  type="button"
                  @click="
                    () =>
                      updateOrderStatus({
                        id: order.id,
                        supplier_id: order.supplierId,
                        status: 'CANCELED',
                      })
                  "
                  variant="secondary"
                  size="sm"
                  class="border bg-red-100 w-full border-red-500 text-red-900"
                >
                  {{ t(`g.status.canceled`) }}
                </Button>
              </PopoverContent>
            </Popover>
          </td>
          <td class="p-2">
            {{ order.createdAt ? d(new Date(order.createdAt), "long") : "" }}
          </td>
          <td class="p-2">{{ order.total?.toFixed(2) }} DH</td>
          <td class="p-2">
            <div class="flex justify-center items-center gap-3">
              <Trash2
                @click="toggleThisOrders(order, 'OrderDelete')"
                :size="22"
              />
              <FilePenLine
                @click="toggleThisOrders(order, 'OrderUpdate')"
                :size="22"
              />
              <RouterLink
                :to="{
                  path: '/orders/' + order.id,
                }"
              >
                <Printer :size="22" />
              </RouterLink>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
    <UiPagination />
  </div>
</template>
