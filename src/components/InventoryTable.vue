<script setup lang="ts">
import { useI18n } from "vue-i18n";
import UiPagination from "./ui/UiPagination.vue";
import type { InventoryT } from "@/schemas/inventory.schema";
import { cn } from "@/utils/shadcn";
import { Badge } from "./ui/badge";

const { t, d } = useI18n();

defineProps<{
  inventory: InventoryT[];
}>();
</script>

<template>
  <div>
    <table>
      <thead>
        <tr>
          <th>{{ t("g.fields.name") }}</th>
          <th>{{ t("g.fields.price") }}</th>
          <th>{{ t("g.fields.quantity") }}</th>
          <th>{{ t("g.fields.status") }}</th>
          <th>{{ t("g.fields.date") }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-fade="index" v-for="(mvm, index) in inventory" :key="mvm.id">
          <td class="p-2 font-medium">
            {{ mvm?.name }}
          </td>
          <td class="p-2">{{ mvm?.price?.toFixed(2) }} DH</td>
          <td class="p-2">
            {{ t("g.plrz.i", { n: mvm.quantity }) }}
          </td>
          <td class="p-2">
            <Badge
              variant="outline"
              :class="
                cn(
                  'cursor-pointer whitespace-nowrap',
                  mvm?.mvmType == 'OUT'
                    ? 'bg-green-100 border-green-500 text-green-900'
                    : 'bg-sky-100 border-sky-500 text-sky-900',
                )
              "
            >
              {{ t("g.status." + mvm?.mvmType.toLowerCase()) }}
            </Badge>
          </td>
          <td class="p-2">
            {{ d(new Date(mvm.createdAt), "long") }}
          </td>
        </tr>
      </tbody>
    </table>
    <UiPagination />
  </div>
</template>
