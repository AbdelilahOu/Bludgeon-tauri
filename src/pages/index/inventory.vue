<script setup lang="ts">
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";
import InventoryTable from "@/components/InventoryTable.vue";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { store } from "@/store";
import { Transition } from "vue";
import {
  ref,
  computed,
  onUnmounted,
  onMounted,
  provide,
  watch,
  type WatchStopHandle,
} from "vue";
import type { InventoryT } from "@/schemas/inventory.schema";
import type { Res } from "@/types";
import {
  SelectContent,
  SelectTrigger,
  SelectValue,
  SelectItem,
  Select,
} from "@/components/ui/select";
import { Calendar as CalendarIcon } from "lucide-vue-next";
import { Calendar } from "@/components/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { cn } from "@/utils/shadcn";
import { error } from "tauri-plugin-log-api";

const { t } = useI18n();
const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
//
const inventoryMouvements = ref<InventoryT[]>([]);
const searchQuery = ref<string>("");
const page = computed(() => Number(route.query.page));
const refresh = computed(() => route.query.refresh);
const status = ref<string | undefined>(undefined);
const createdAt = ref<string | number | undefined>(undefined);
const totalRows = ref<number>(0);
//
provide("count", totalRows);
provide("itemsCount", 30);
//
let timer: any;
let unwatch: WatchStopHandle | null = null;
onMounted(() => {
  unwatch = watch(
    [searchQuery, page, refresh, createdAt, status],
    ([search, p], [oldSearch]) => {
      clearTimeout(timer);
      timer = setTimeout(
        () => {
          if (p && p > 0) getInventory(search, p);
        },
        search != oldSearch && oldSearch ? 500 : 0,
      );
    },
    {
      immediate: true,
    },
  );
});

onUnmounted(() => {
  if (unwatch) unwatch();
});

async function getInventory(search: string, page: number = 1) {
  try {
    const res = await invoke<Res<any>>("list_inventory", {
      args: {
        page,
        search,
        limit: 30,
        status: status.value,
        created_at: createdAt.value
          ? new Date(createdAt.value).toISOString().slice(0, 10)
          : null,
      },
    });
    //
    inventoryMouvements.value = res.data.inventory;
    totalRows.value = res.data.count;
  } catch (err: any) {
    error("LIST INVENTORY MOUVEMENTS: " + err.error);
  }
}

const uploadCSV = () => {
  store.setters.updateStore({ key: "name", value: "CsvUploader" });
  store.setters.updateStore({ key: "show", value: true });
  updateQueryParams({ table: "clients" });
};
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <Transition appear>
        <div class="flex justify-between w-full gap-9 mb-2">
          <div class="w-full lg:max-w-[50%] max-w-[70%] grid grid-cols-3 gap-2">
            <Input v-model="searchQuery" type="text" :placeHolder="t('g.s')" />

            <Popover>
              <PopoverTrigger as-child>
                <Button
                  variant="outline"
                  :class="
                    cn(
                      'w-full justify-start text-left font-normal',
                      !createdAt && 'text-muted-foreground',
                    )
                  "
                >
                  <CalendarIcon class="mr-2 h-4 w-4" />
                  <span>{{
                    createdAt
                      ? new Date(createdAt).toLocaleDateString("fr-fr")
                      : "Pick a date"
                  }}</span>
                </Button>
              </PopoverTrigger>
              <PopoverContent class="w-auto p-0">
                <Calendar v-model="createdAt" />
              </PopoverContent>
            </Popover>
            <Select v-model="status">
              <SelectTrigger>
                <SelectValue placeholder="Select a status" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="OUT"> Sold </SelectItem>
                <SelectItem value="IN"> Bought </SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div>
            <Button variant="ghost" @click="uploadCSV">
              <span
                class="text-sky-400 transition-all duration-200 scale-[0.8] group-hover:fill-sky-600"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="32"
                  height="32"
                  viewBox="0 0 256 256"
                >
                  <path
                    d="m217.5 170.3l-20 48a5.9 5.9 0 0 1-11 0l-20-48a6 6 0 0 1 11-4.6l14.5 34.7l14.5-34.7a6 6 0 1 1 11 4.6ZM76 206.1a15.1 15.1 0 0 1-10 3.9c-8.8 0-16-8.1-16-18s7.2-18 16-18a15.1 15.1 0 0 1 10 3.9a5.9 5.9 0 0 0 8.5-.4a6 6 0 0 0-.5-8.5a26.9 26.9 0 0 0-18-7c-15.4 0-28 13.5-28 30s12.6 30 28 30a26.9 26.9 0 0 0 18-7a6 6 0 0 0 .5-8.5a5.9 5.9 0 0 0-8.5-.4Zm53.2-20.4c-7.8-2-11.2-3.3-11.2-5.7c0-6.1 5.6-7 9-7a19.7 19.7 0 0 1 11.2 3.6a6 6 0 0 0 7.6-9.2A30 30 0 0 0 127 161c-12.4 0-21 7.8-21 19s11.6 15.1 20.1 17.3S138 201 138 204s0 7-11 7a20 20 0 0 1-11.2-3.6a6 6 0 1 0-7.6 9.2A30 30 0 0 0 127 223c14.4 0 23-7.1 23-19s-12.5-16.1-20.8-18.3ZM202 94h-50a6 6 0 0 1-6-6V38H56a2 2 0 0 0-2 2v88a6 6 0 0 1-12 0V40a14 14 0 0 1 14-14h96a5.6 5.6 0 0 1 4.2 1.8l56 55.9A6 6 0 0 1 214 88v40a6 6 0 0 1-12 0Zm-44-12h35.5L158 46.5Z"
                  />
                </svg>
              </span>
            </Button>
          </div>
        </div>
      </Transition>

      <Transition appear>
        <InventoryTable :inventory="inventoryMouvements" />
      </Transition>
    </div>
  </main>
</template>
