<script setup lang="ts">
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { uploadCSVfiles } from "@/utils/fs";
import { useDropZone } from "@vueuse/core";
import { invoke } from "@tauri-apps/api";
import { useRoute } from "vue-router";
import { Trash2 } from "lucide-vue-next";
import { Button } from "./ui/button";
import { store } from "@/store";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import UiModalCard from "./ui/UiModalCard.vue";
import { error } from "tauri-plugin-log-api";

const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();

const { t } = useI18n();

const dropZone = ref<HTMLDivElement>();

const filesData = ref<File[]>([]);

async function onDrop(files: File[] | null) {
  filesData.value = [];
  if (files) {
    filesData.value = files.filter((file) => file.type === "text/csv");
  }
}
const { isOverDropZone } = useDropZone(dropZone, onDrop);

const upload = async () => {
  for await (const file of filesData.value) {
    try {
      await invoke("upload_csv_to_db", {
        csvPath: await uploadCSVfiles({ file: file }),
        table: route.query.table,
      });
    } catch (err: any) {
      error("Error upload csv : " + err.error);
    } finally {
      updateQueryParams({
        refresh: "refresh-upload-" + Math.random() * 9999,
      });
      store.setters.updateStore({ key: "show", value: false });
    }
  }
};
</script>

<template>
  <UiModalCard>
    <template #title>
      {{ t("csv.title") }}
    </template>
    <template #content>
      <div class="h-full w-full flex flex-col gap-2">
        <div
          ref="dropZone"
          :class="[
            'w-full relative rounded-md transition-all duration-200 transform z-50 h-28 border-2 border-dashed border-spacing-4 flex items-center justify-center',

            isOverDropZone
              ? 'fill-sky-500 border-sky-500 bg-sky-200'
              : 'fill-gray-400 border-gray-300 bg-white',
          ]"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="32"
            height="32"
            viewBox="0 0 256 256"
          >
            <path
              d="m217.5 170.3l-20 48a5.9 5.9 0 0 1-11 0l-20-48a6 6 0 0 1 11-4.6l14.5 34.7l14.5-34.7a6 6 0 1 1 11 4.6ZM76 206.1a15.1 15.1 0 0 1-10 3.9c-8.8 0-16-8.1-16-18s7.2-18 16-18a15.1 15.1 0 0 1 10 3.9a5.9 5.9 0 0 0 8.5-.4a6 6 0 0 0-.5-8.5a26.9 26.9 0 0 0-18-7c-15.4 0-28 13.5-28 30s12.6 30 28 30a26.9 26.9 0 0 0 18-7a6 6 0 0 0 .5-8.5a5.9 5.9 0 0 0-8.5-.4Zm53.2-20.4c-7.8-2-11.2-3.3-11.2-5.7c0-6.1 5.6-7 9-7a19.7 19.7 0 0 1 11.2 3.6a6 6 0 0 0 7.6-9.2A30 30 0 0 0 127 161c-12.4 0-21 7.8-21 19s11.6 15.1 20.1 17.3S138 201 138 204s0 7-11 7a20 20 0 0 1-11.2-3.6a6 6 0 1 0-7.6 9.2A30 30 0 0 0 127 223c14.4 0 23-7.1 23-19s-12.5-16.1-20.8-18.3ZM202 94h-50a6 6 0 0 1-6-6V38H56a2 2 0 0 0-2 2v88a6 6 0 0 1-12 0V40a14 14 0 0 1 14-14h96a5.6 5.6 0 0 1 4.2 1.8l56 55.9A6 6 0 0 1 214 88v40a6 6 0 0 1-12 0Zm-44-12h35.5L158 46.5Z"
            />
          </svg>
        </div>
      </div>
      <div v-if="filesData.length" class="w-full h-fit flex flex-col gap-2">
        <div
          v-for="(file, i) in filesData"
          :key="i"
          class="grid grid-cols-[30px_1fr_80px_30px] w-full gap-3 h-10 items-center px-1 rounded-md bg-sky-100 text-black fill-black border-sky-300 border-2"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="32"
            height="32"
            viewBox="0 0 256 256"
          >
            <path
              d="m217.5 170.3l-20 48a5.9 5.9 0 0 1-11 0l-20-48a6 6 0 0 1 11-4.6l14.5 34.7l14.5-34.7a6 6 0 1 1 11 4.6ZM76 206.1a15.1 15.1 0 0 1-10 3.9c-8.8 0-16-8.1-16-18s7.2-18 16-18a15.1 15.1 0 0 1 10 3.9a5.9 5.9 0 0 0 8.5-.4a6 6 0 0 0-.5-8.5a26.9 26.9 0 0 0-18-7c-15.4 0-28 13.5-28 30s12.6 30 28 30a26.9 26.9 0 0 0 18-7a6 6 0 0 0 .5-8.5a5.9 5.9 0 0 0-8.5-.4Zm53.2-20.4c-7.8-2-11.2-3.3-11.2-5.7c0-6.1 5.6-7 9-7a19.7 19.7 0 0 1 11.2 3.6a6 6 0 0 0 7.6-9.2A30 30 0 0 0 127 161c-12.4 0-21 7.8-21 19s11.6 15.1 20.1 17.3S138 201 138 204s0 7-11 7a20 20 0 0 1-11.2-3.6a6 6 0 1 0-7.6 9.2A30 30 0 0 0 127 223c14.4 0 23-7.1 23-19s-12.5-16.1-20.8-18.3ZM202 94h-50a6 6 0 0 1-6-6V38H56a2 2 0 0 0-2 2v88a6 6 0 0 1-12 0V40a14 14 0 0 1 14-14h96a5.6 5.6 0 0 1 4.2 1.8l56 55.9A6 6 0 0 1 214 88v40a6 6 0 0 1-12 0Zm-44-12h35.5L158 46.5Z"
            />
          </svg>
          <span>{{ file.name }}</span>
          <div class="w-full text-end">{{ file.size }} bytes</div>
          <Trash2 class="cursor-pointer" @click="filesData.splice(i, 1)" />
        </div>
        <Button @click="upload">
          {{ t("g.b.csv", { table: route.query.table }) }}
        </Button>
      </div>
    </template>
  </UiModalCard>
</template>
