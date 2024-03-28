<script setup lang="ts">
import { useI18n } from "vue-i18n";
// import { RouteLinks } from "@/constants/RouteNames";
// import { useRoute } from "vue-router";
// import { computed } from "vue";
// import { store } from "@/store";
import { HomeIcon, ChevronLeft, ChevronRight } from "lucide-vue-next";

const { t } = useI18n();
// const route = useRoute();
// const ActiveLink = computed(() =>
//   RouteLinks.find((link) => link.path === route.path),
// );
// const User = computed(() => store.getters.getUser());
</script>

<template>
  <header class="w-full h-full print:hidden sticky top-0 z-50">
    <div
      class="w-full h-full flex items-center py-3 px-2 bg-white justify-between"
    >
      <div class="text-black flex items-center justify-center gap-2">
        <ChevronLeft class="cursor-pointer" @click="$router.back()" />
        <ChevronRight class="cursor-pointer" @click="$router.forward()" />
        <RouterLink to="/">
          <HomeIcon :size="22" class="bg-white" />
        </RouterLink>
        <!-- <span class="pb-[3px] text-gray-700 flex items-center"> -->
        <!-- <span v-if="route.fullPath !== '/'" class="flex items-center">
            <span class="flex items-center">
              {{ ActiveLink && " / " }}
              {{ ActiveLink && t(`g.r.${ActiveLink.name}`) }}
              {{ route.params.id ? `/ n° ${route.params.id}` : "" }}
            </span>
          </span> -->
        <!-- </span> -->
      </div>
      <!-- <div v-if="User" class="flex items-center gap-2">
        <img class="rounded-full w-8 h-8" :src="User.photoURL" alt="" />
        <span class="font-semibold">{{ User.displayName }}</span>
      </div> -->
    </div>
  </header>
</template>
