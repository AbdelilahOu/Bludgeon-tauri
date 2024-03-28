<script setup lang="ts">
import { cn } from "@/utils/shadcn";
</script>

<template>
  <div
    :class="[
      cn(
        ($attrs.class as string) ??
          'w-1/2 h-fit z-50 gap-3 rounded-md flex flex-col bg-white p-2 min-w-[350px]',
      ),
    ]"
  >
    <h1
      class="font-semibold text-lg text-gray-800 border-b-2 border-b-gray-500 pb-2 uppercase text-center"
    >
      <slot name="title"></slot>
    </h1>
    <slot name="content"></slot>
    <slot name="footer"></slot>
  </div>
</template>
