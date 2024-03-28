<template>
  <div
    :class="[
      'w-full h-full grid overflow-hidden bg-white gap-1 grid-cols-1 border rounded-md shadow-sm',
      $slots.title ? 'grid-rows-[50px_1fr]' : 'grid-rows-1',
    ]"
  >
    <div
      v-if="$slots.title"
      class="w-full h-full flex items-center bg-slate-50"
    >
      <slot name="title"></slot>
    </div>
    <div class="w-full h-full p-2 flex">
      <slot></slot>
    </div>
  </div>
</template>
