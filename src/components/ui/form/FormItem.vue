<script lang="ts">
// import { type InjectionKey } from 'vue'

export const FORM_ITEM_INJECTION_KEY = Symbol() as InjectionKey<string>;
</script>

<script lang="ts" setup>
import { provide, useAttrs, type InjectionKey } from "vue";
import { useId } from "radix-vue";
import { cn } from "@/utils/shadcn";

defineOptions({
  inheritAttrs: false,
});

const id = useId();
provide(FORM_ITEM_INJECTION_KEY, id);

const { class: className, ...rest } = useAttrs();
</script>

<template>
  <div :class="cn('space-y-2', (className as string) ?? '')" v-bind="rest">
    <slot />
  </div>
</template>
