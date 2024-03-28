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
import type { InvoiceForUpdateT } from "@/schemas/invoice.schema";
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

const clients = ref<{ label: string; value: string }[]>([]);
const products = ref<{ label: string; value: string }[]>([]);
const invoice = reactive<InvoiceForUpdateT>({
  id: "",
  clientId: "",
  paidAmount: 0,
  fullname: "",
  createdAt: "",
  status: "",
  items: [],
});

onBeforeMount(async () => {
  // @ts-ignore
  const res = await invoke<Res<InvoiceForUpdateT>>("get_invoice", {
    id: route.query.id,
  });

  if (!res.error) {
    invoice.id = res.data.id;
    invoice.clientId = res.data.clientId;
    invoice.paidAmount = res.data.paidAmount;
    invoice.createdAt = res.data.createdAt;
    invoice.status = res.data.status;
    invoice.fullname = res.data.fullname;
    invoice.items = res.data.items;
  }
});

const searchSuppliers = async (search: string | number) => {
  const res = await invoke<Res<{ label: string; value: string }[]>>(
    "search_clients",
    {
      search,
    },
  );
  if (!res.error) {
    clients.value = res.data;
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

const addInvoiceItem = () => {
  invoice.items?.push({
    product_id: undefined,
    quantity: undefined,
    price: undefined,
  });
};

const updateTheInvoices = async () => {
  try {
    await invoke<Res<String>>("update_invoice", {
      invoice: {
        id: invoice.id,
        client_id: invoice.clientId,
        status: invoice.status,
        paid_amount: invoice.paidAmount,
      },
    });
    //
    for await (const item of invoice.items) {
      if (!item.id) {
        const invRes = await invoke<Res<string>>("create_inventory", {
          mvm: {
            mvm_type: "OUT",
            product_id: item.product_id,
            quantity: item.quantity,
          },
        });
        await invoke<Res<string>>("create_invoice_item", {
          item: {
            invoice_id: invoice.id,
            inventory_id: invRes.data,
            price: item.price,
          },
        });
      } else {
        await invoke<Res<string>>("update_inventory", {
          mvm: {
            id: item.inventory_id,
            mvm_type: "OUT",
            product_id: item.product_id,
            quantity: item.quantity,
          },
        });
        await invoke<Res<string>>("update_invoice_item", {
          item: {
            id: item.id,
            invoice_id: invoice.id,
            inventory_id: item.inventory_id,
            price: item.price,
          },
        });
      }
    }
    //
    info(`UPDATE INVOICE: ${JSON.stringify(invoice)}`);
    // toggle refresh
    updateQueryParams({
      refresh: "refresh-update-" + Math.random() * 9999,
    });
  } catch (err: any) {
    error("UPDATE INVOICE: " + err.error);
  } finally {
    hideModal();
  }
};

const hideModal = () => {
  store.setters.updateStore({ key: "show", value: false });
};

async function deleteOneInvoiceItem(id: string) {
  try {
    await invoke("delete_invoice_item", { id });
  } catch (err: any) {
    error("Error creating client : " + err.error);
  }
}

const deleteInvoiceItem = (index: number) => {
  const item = invoice.items?.splice(index, 1)[0];
  if (item?.id) deleteOneInvoiceItem(item.id);
};
</script>

<template>
  <UiModalCard
    class="w-5/6 lg:w-1/2 relative h-fit rounded-md z-50 gap-3 flex flex-col bg-white p-2 min-w-[350px]"
  >
    <template #title> {{ t("i.u.title") }} N° {{ invoice?.id }} </template>
    <template #content>
      <div class="h-full w-full grid grid-cols-1 gap-2">
        <div class="flex w-full h-fit gap-1">
          <div class="w-full h-full flex flex-col gap-1">
            <Label for="client_id">
              {{ t("i.u.d.c.title") }}
            </Label>
            <SearchableItems
              v-if="invoice.fullname"
              :defaultValue="invoice.fullname"
              :items="clients"
              @update:items="(s) => searchSuppliers(s)"
              @on:select="(id) => (invoice.clientId = id)"
            />
          </div>
          <div class="w-full h-full flex flex-col gap-1">
            <Label for="status">
              {{ t("i.u.d.c.title") }}
            </Label>
            <Select v-model="invoice.status">
              <SelectTrigger>
                <SelectValue placeholder="Select a status" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="PAID"> Paid </SelectItem>
                <SelectItem value="CANCELED"> Cancelled </SelectItem>
                <SelectItem value="PENDING"> Pending </SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
        <div class="w-full h-full flex flex-col gap-1">
          <Label for="status">
            {{ t("i.u.d.c.title") }}
          </Label>
          <Input v-model="invoice.paidAmount" place-holder="" type="number" />
        </div>
        <Separator />
        <div class="w-full h-full flex flex-col gap-1">
          <Button @click="addInvoiceItem">
            {{ t("i.u.d.i.add") }}
          </Button>
          <div
            class="w-full pt-1 grid items-center grid-cols-[1fr_1fr_1fr_36px] overflow-auto scrollbar-thin scrollbar-thumb-transparent max-h-64 gap-1"
          >
            <template v-for="(item, index) in invoice.items" :key="index">
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
                @click="deleteInvoiceItem(index)"
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
        <Button class="col-span-2" @click="updateTheInvoices">
          {{ t("g.b.d") }}
        </Button>
      </div>
    </template>
  </UiModalCard>
</template>
