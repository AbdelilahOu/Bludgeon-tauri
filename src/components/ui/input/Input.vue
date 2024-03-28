<script setup lang="ts">
import { useVModel } from "@vueuse/core";
import { cn } from "@/utils/shadcn";

const props = defineProps<{
  defaultValue?: string | number;
  modelValue?: string | number;
}>();

const emits = defineEmits<{
  (e: "update:modelValue", payload: string | number): void;
}>();

const modelValue = useVModel(props, "modelValue", emits, {
  passive: true,
  defaultValue: props.defaultValue,
});
</script>
<template>
  <div :class="cn('w-full h-fit flex')">
    <input
      v-bind="$attrs"
      v-model="modelValue"
      :class="
        cn(
          'flex h-10 w-full border px-2 focus-visible:border-2 focus-visible:border-black focus:outline-0 rounded-md border-input bg-background py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 transform transition-color duration-100',
          $slots.unite
            ? 'rounded-r-none border-r-0 focus-visible:border-r-0'
            : '',
        )
      "
    />
    <div
      v-if="$slots.unite"
      class="w-fit h-10 transform transition-color border duration-100 border-l-0 flex rounded-r-md items-center justify-center"
    >
      <span
        class="h-full text-gray-400 rounded-md px-2 flex items-center justify-center"
      >
        <slot name="unite"></slot>
      </span>
    </div>
  </div>
</template>

<style>
div:has(> input:focus-visible) > div {
  border: 2px black solid;
  border-left: 0px;
}
</style>
