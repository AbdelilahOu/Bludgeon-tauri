<script setup lang="ts">
import { Skeleton } from "@/components/ui/skeleton";
</script>

<template>
  <div
    class="w-1/2 h-fit z-50 gap-3 flex flex-col bg-white rounded-md p-2 min-w-[350px]"
  >
    <h1
      class="font-semibold text-lg h-10 text-gray-800 border-b-2 border-b-gray-500 pb-2 uppercase text-center"
    >
      <Skeleton class="w-full h-full" />
    </h1>
    <div class="h-40 w-full flex flex-col gap-2">
      <Skeleton class="w-full h-full" />
    </div>
  </div>
</template>
