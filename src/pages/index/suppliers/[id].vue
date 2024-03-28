<script setup lang="ts">
import { onBeforeMount, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { useRoute } from "vue-router";
import { store } from "@/store";
import ChartHolder from "@/components/ChartHolder.vue";
import { useI18n } from "vue-i18n";
import type { SupplierT } from "@/schemas/supplier.schema";
import { error } from "tauri-plugin-log-api";

const { t, d } = useI18n();
const { id } = useRoute().params;
const supplier = ref<SupplierT | null>(null);
</script>

<template>
  <main class="w-full h-full px-3 py-1">
    <div
      class="w-full h-full text-black grid gap-4 xl:grid-cols-[400px_2px_1fr] xl:grid-rows-1 grid-rows-[260px_2px_1fr] grid-cols-1 print:pr-12"
    >
      <div
        class="w-full grid-cols-[400px_1fr] xl:grid-rows-[258px_1fr] xl:grid-cols-1 items-start justify-start gap-3 grid"
      >
        <div class="w-full flex items-end xl:items-start h-fit"></div>
      </div>
      <div class="xl:border-l-2 border-b-2"></div>
      <div class="w-full">
        <ChartHolder>
          <template #default> </template>
          <template #title>
            <h1 class="m-2 w-full text-center text-base font-medium">
              <i>
                {{ t("dashboard.de.title") }}
              </i>
            </h1>
          </template>
        </ChartHolder>
      </div>
    </div>
  </main>
</template>
