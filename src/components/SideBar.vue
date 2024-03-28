<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { RouterLink } from "vue-router";
import { store } from "@/store";
import { computed } from "vue";
import { cn } from "@/utils/shadcn";
import {
  Archive,
  BarChartBig,
  ChevronsLeft,
  Languages,
  NotepadText,
  Package,
  Settings,
  Truck,
  Contact,
} from "lucide-vue-next";

const collapse = defineModel<boolean>("collapse", { required: true });
const { t } = useI18n();

const locale = computed(() => store.getters.getCurrentLocale());

const openTranslationModal = () => {
  store.setters.updateStore({ key: "show", value: true });
  store.setters.updateStore({ key: "name", value: "TranslationModal" });
};
</script>

<template>
  <aside class="w-full h-screen sticky top-0 z-50 flex flex-col gap-3 bg-white">
    <div class="w-full h-[50px]">
      <div
        :class="
          cn(
            'font-medium flex h-full items-center px-1',
            collapse ? 'justify-around' : 'justify-between',
          )
        "
      >
        <span
          v-if="!collapse"
          class="whitespace-nowrap pl-2 text-gray-800 overflow-hidden"
        >
          The Inventoryer
        </span>
        <ChevronsLeft
          @click="collapse = !collapse"
          class="transition-all duration-200 cursor-pointer transform"
          :class="{ 'rotate-180': collapse }"
        />
      </div>
    </div>
    <div class="w-full px-1 flex flex-col justify-around h-full pb-2">
      <div class="w-full h-full flex flex-col gap-1">
        <RouterLink
          class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
          :to="{ path: '/clients/', query: { page: 1 } }"
        >
          <span class="w-8 min-w-[30px] max-w-[30px]">
            <Contact
              class="m-auto text-gray-700 group-hover:text-black"
              :size="22"
            />
          </span>
          <span
            v-if="!collapse"
            class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
          >
            {{ t("g.r.Clients") }}
          </span>
        </RouterLink>
        <RouterLink
          class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
          :to="{ path: '/suppliers/', query: { page: 1 } }"
        >
          <span class="w-8 min-w-[30px] max-w-[30px]">
            <Contact
              class="m-auto text-gray-700 group-hover:text-black"
              :size="22"
            />
          </span>
          <span
            v-if="!collapse"
            class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
          >
            {{ t("g.r.Suppliers") }}
          </span>
        </RouterLink>
        <RouterLink
          class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
          :to="{ path: '/products/', query: { page: 1 } }"
        >
          <span class="w-8 min-w-[30px] max-w-[30px]">
            <Package
              class="m-auto text-gray-700 group-hover:text-black"
              :size="22"
            />
          </span>
          <span
            v-if="!collapse"
            class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
          >
            {{ t("g.r.Products") }}
          </span>
        </RouterLink>
        <RouterLink
          class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
          :to="{ path: '/orders/', query: { page: 1 } }"
        >
          <span class="w-8 min-w-[30px] max-w-[30px]">
            <Truck
              class="m-auto text-gray-700 group-hover:text-black"
              :size="22"
            />
          </span>
          <span
            v-if="!collapse"
            class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
          >
            {{ t("g.r.Orders") }}
          </span>
        </RouterLink>
        <RouterLink
          class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
          :to="{ path: '/invoices/', query: { page: 1 } }"
        >
          <span class="w-8 min-w-[30px] max-w-[30px]">
            <NotepadText
              class="m-auto text-gray-700 group-hover:text-black"
              :size="22"
            />
          </span>
          <span
            v-if="!collapse"
            class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
          >
            {{ t("g.r.Invoices") }}
          </span>
        </RouterLink>
        <RouterLink
          class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
          :to="{ path: '/inventory/', query: { page: 1 } }"
        >
          <span class="w-8 min-w-[30px] max-w-[30px]">
            <Archive
              class="m-auto text-gray-700 group-hover:text-black"
              :size="22"
            />
          </span>
          <span
            v-if="!collapse"
            class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
          >
            {{ t("g.r.Inventory") }}
          </span>
        </RouterLink>
        <RouterLink
          class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
          :to="{ path: '/dashboard' }"
        >
          <span class="w-8 min-w-[30px] max-w-[30px]">
            <BarChartBig
              class="m-auto text-gray-700 group-hover:text-black"
              :size="22"
            />
          </span>
          <span
            v-if="!collapse"
            class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
          >
            {{ t("g.r.Dashboard") }}
          </span>
        </RouterLink>
      </div>
      <div
        class="cursor-pointer w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
        @click="openTranslationModal"
      >
        <span class="w-8 min-w-[30px] max-w-[30px]">
          <Languages
            class="m-auto text-gray-700 group-hover:text-black"
            :size="22"
          />
        </span>
        <span
          v-if="!collapse"
          class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
        >
          {{ locale.text }}
        </span>
      </div>
      <RouterLink
        class="w-full flex h-9 rounded-md items-center p-1 group transition-all duration-300"
        :to="{ path: '/settings' }"
      >
        <span class="w-8 min-w-[30px] max-w-[30px]">
          <Settings
            class="m-auto text-gray-700 group-hover:text-black"
            :size="22"
          />
        </span>
        <span
          v-if="!collapse"
          class="text-gray-500 ml-1 text-sm group-hover:text-primary overflow-hidden"
        >
          {{ t("g.r.Settings") }}
        </span>
      </RouterLink>
    </div>
  </aside>
</template>
