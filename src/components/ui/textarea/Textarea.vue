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
  <textarea
    v-model="modelValue"
    :class="
      cn(
        'min-h-[80px] flex h-10 w-full border p-2 focus-visible:border-2 focus-visible:border-black focus:outline-0 rounded-md border-input bg-background py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 transform transition-color duration-100',
        ($attrs.class as string) ?? '',
      )
    "
  />
</template>
