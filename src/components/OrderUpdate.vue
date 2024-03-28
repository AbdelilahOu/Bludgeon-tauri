<script setup lang="ts">
import { ref, reactive, onBeforeMount } from "vue";
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api";
import { Button } from "./ui/button";
import { Trash2 } from "lucide-vue-next";
import { Input } from "./ui/input";
import { store } from "@/store";
import UiModalCard from "./ui/UiModalCard.vue";
import { Label } from "./ui/label";
import { Separator } from "./ui/separator";
import { useRoute } from "vue-router";
import type { Res } from "@/types";
import type { OrderForUpdateT } from "@/schemas/order.schema";
import {
  SelectContent,
  SelectTrigger,
  SelectValue,
  SelectItem,
  Select,
} from "@/components/ui/select";
import SearchableItems from "./ui/UISearchableItems.vue";
import { error, info } from "tauri-plugin-log-api";

const { updateQueryParams } = useUpdateRouteQueryParams();
const { t } = useI18n();
const route = useRoute();

const suppliers = ref<{ label: string; value: string }[]>([]);
const products = ref<{ label: string; value: string }[]>([]);
const order = reactive<OrderForUpdateT>({
  id: "",
  supplierId: "",
  fullname: "",
  createdAt: "",
  status: "",
  items: [],
});

onBeforeMount(async () => {
  // @ts-ignore
  const res = await invoke<Res<OrderForUpdateT>>("get_order", {
    id: route.query.id,
  });

  if (!res.error) {
    order.id = res.data.id;
    order.supplierId = res.data.supplierId;
    order.createdAt = res.data.createdAt;
    order.status = res.data.status;
    order.fullname = res.data.fullname;
    order.items = res.data.items;
  }
});

const searchSuppliers = async (search: string | number) => {
  const res = await invoke<Res<{ label: string; value: string }[]>>(
    "search_suppliers",
    {
      search,
    },
  );
  if (!res.error) {
    suppliers.value = res.data;
  }
};

const searchProducts = async (search: string | number) => {
  const res = await invoke<Res<{ label: string; value: string }[]>>(
    "search_products",
    {
      search,
    },
  );
  if (!res.error) {
    products.value = res.data;
  }
};

const addOrderItem = () => {
  order.items?.push({
    product_id: undefined,
    quantity: undefined,
    price: undefined,
  });
};

const updateTheOrders = async () => {
  try {
    await invoke<Res<String>>("update_order", {
      order: {
        id: order.id,
        supplier_id: order.supplierId,
        status: order.status,
      },
    });
    for await (const item of order.items) {
      if (!item.id) {
        const invRes = await invoke<Res<string>>("create_inventory", {
          mvm: {
            mvm_type: "IN",
            product_id: item.product_id,
            quantity: item.quantity,
          },
        });
        await invoke<Res<string>>("create_order_item", {
          item: {
            order_id: order.id,
            inventory_id: invRes.data,
            price: item.price,
          },
        });
      } else {
        await invoke<Res<string>>("update_inventory", {
          mvm: {
            id: item.inventory_id,
            mvm_type: "IN",
            product_id: item.product_id,
            quantity: item.quantity,
          },
        });
        await invoke<Res<string>>("update_order_item", {
          item: {
            id: item.id,
            order_id: order.id,
            inventory_id: item.inventory_id,
            price: item.price,
          },
        });
      }
    }
    //
    info(`UPDATE ORDER: ${JSON.stringify(order)}`);
    // toggle refresh
    updateQueryParams({
      refresh: "refresh-update-" + Math.random() * 9999,
    });
  } catch (err: any) {
    error("UPDATE ORDER: " + err.error);
  } finally {
    hideModal();
  }
};

const hideModal = () => {
  store.setters.updateStore({ key: "show", value: false });
};

async function deleteOneOrderItem(id: string) {
  try {
    await invoke("delete_order_item", { id });
  } catch (err: any) {
    error("Error creating client : " + err.error);
  }
}

const deleteOrderItem = (index: number) => {
  const item = order.items?.splice(index, 1)[0];
  if (item?.id) deleteOneOrderItem(item.id);
};
</script>

<template>
  <UiModalCard
    class="w-5/6 lg:w-1/2 relative h-fit rounded-md z-50 gap-3 flex flex-col bg-white p-2 min-w-[350px]"
  >
    <template #title> {{ t("o.u.title") }} N° {{ order?.id }} </template>
    <template #content>
      <div class="h-full w-full grid grid-cols-1 gap-2">
        <div class="flex w-full h-fit gap-1">
          <div class="w-full h-full flex flex-col gap-1">
            <Label for="supplier_id">
              {{ t("o.u.d.o.title") }}
            </Label>
            <SearchableItems
              v-if="order.fullname"
              :defaultValue="order.fullname"
              :items="suppliers"
              @update:items="(s) => searchSuppliers(s)"
              @on:select="(id) => (order.supplierId = id)"
            />
          </div>
          <div class="w-full h-full flex flex-col gap-1">
            <Label for="status">
              {{ t("o.u.d.o.title") }}
            </Label>
            <Select v-model="order.status">
              <SelectTrigger>
                <SelectValue placeholder="Select a status" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="DELIVERED"> Delivered </SelectItem>
                <SelectItem value="CANCELED"> Cancelled </SelectItem>
                <SelectItem value="PENDING"> Pending </SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
        <Separator />
        <div class="w-full h-full flex flex-col gap-1">
          <Button @click="addOrderItem">
            {{ t("o.u.d.o.add") }}
          </Button>
          <div
            class="w-full pt-1 grid grid-cols-[1fr_1fr_1fr_36px] items-center overflow-auto scrollbar-thin scrollbar-thumb-transparent max-h-64 gap-1"
          >
            <template v-for="(item, index) in order.items" :key="index">
              <SearchableItems
                :defaultValue="item.name"
                :items="products"
                @update:items="(s) => searchProducts(s)"
                @on:select="
                  (id, price) => ((item.product_id = id), (item.price = price))
                "
              />
              <Input
                v-model="item.quantity"
                class="border-r-0"
                :placeHolder="t('o.c.d.o.placeholder[0]')"
                type="number"
              >
                <template #unite> Item </template>
              </Input>
              <Input
                v-model="item.price"
                class="border-r-0"
                :placeHolder="t('o.c.d.o.placeholder[1]')"
                type="number"
              >
                <template #unite> DH </template>
              </Input>
              <Trash2
                @click="deleteOrderItem(index)"
                class="cursor-pointer"
                :size="22"
              />
            </template>
          </div>
        </div>
      </div>
    </template>
    <template #footer>
      <div class="grid grid-cols-3 gap-2">
        <Button variant="outline" @click="hideModal">
          {{ t("g.b.no") }}
        </Button>
        <Button class="col-span-2" @click="updateTheOrders">
          {{ t("g.b.d") }}
        </Button>
      </div>
    </template>
  </UiModalCard>
</template>
