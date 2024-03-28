<script setup lang="ts">
import type { ComboboxEmptyProps } from "radix-vue";
import { ComboboxEmpty } from "radix-vue";
import { cn } from "@/utils/shadcn";

const props = defineProps<ComboboxEmptyProps>();
</script>

<template>
  <ComboboxEmpty
    v-bind="props"
    :class="cn('py-6 text-center text-sm', ($attrs.class as String) ?? '')"
  >
    <slot />
  </ComboboxEmpty>
</template>
