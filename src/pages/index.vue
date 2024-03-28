<script setup lang="ts">
import { ref, computed } from "vue";
import { RouterView } from "vue-router";
import ModalComponentsRenderer from "@/components/ModalComponentsRenderer.vue";
import Navigation from "@/components/Navigation.vue";
import SideBar from "@/components/SideBar.vue";
import { store } from "@/store";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/utils/shadcn";
import { Toaster } from "@/components/ui/sonner";

const show = computed(() => store.getters.getModalVisibility());
const name = computed(() => store.getters.getModalName());
const isCollapse = ref<boolean>(true);
</script>

<template>
  <main class="w-screen h-screen fixed top-0 left-0 bg-[#FFFAFA] flex">
    <div
      :class="
        cn(
          'transition-all print:hidden sticky h-screen top-0 border-r border-slate-100 z-50 flex justify-center duration-200',
          isCollapse ? 'w-12 min-w-[48px]' : 'w-52 min-w-[12rem]',
        )
      "
    >
      <SideBar v-model:collapse="isCollapse" />
    </div>
    <div class="grid grid-rows-[50px_1fr] w-full">
      <Navigation />
      <ScrollArea
        class="w-full scroll-smooth h-[calc(100vh-50px)] flex flex-col border-t border-slate-100"
      >
        <div
          v-if="show"
          class="w-full h-full flex items-center justify-center fixed bg-black z-50 top-0 left-0 bg-opacity-30"
        >
          <ModalComponentsRenderer :name="name" />
        </div>
        <div class="w-full bg-[#FFFAFA] p-2 rounded-md h-full">
          <RouterView />
        </div>
      </ScrollArea>
    </div>
    <ModalComponentsRenderer name="ClientProfile" />
    <Toaster position="top-center" />
  </main>
</template>
