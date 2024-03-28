<script setup lang="ts">
import { onBeforeMount, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { useRoute } from "vue-router";
import { store } from "@/store";
import ChartHolder from "@/components/ChartHolder.vue";
import { useI18n } from "vue-i18n";
import type { ClientT } from "@/schemas/client.schema";
import { error } from "tauri-plugin-log-api";

const { t, d } = useI18n();
const { id } = useRoute().params;
const client = ref<ClientT | null>(null);
</script>

<template>
  <main class="w-full h-full px-3 py-1"></main>
</template>
