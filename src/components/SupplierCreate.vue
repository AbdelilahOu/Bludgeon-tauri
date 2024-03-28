<script setup lang="ts">
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { FormControl, FormField, FormItem, FormLabel } from "./ui/form";
import { useI18n } from "vue-i18n";
import { toTypedSchema } from "@vee-validate/zod";
import UiModalCard from "./ui/UiModalCard.vue";
import UiUploader from "./ui/UiUploader.vue";
import { invoke } from "@tauri-apps/api";
import { useForm } from "vee-validate";
import { getFileBytes } from "@/utils/fs";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { store } from "@/store";
import { ref } from "vue";
import {
  CreateSupplierSchema,
  type SupplierT,
} from "@/schemas/supplier.schema";
import { error, info } from "tauri-plugin-log-api";
import type { Res } from "@/types";

const { updateQueryParams } = useUpdateRouteQueryParams();
const { t } = useI18n();
const isLoading = ref<boolean>(false);

const supplierSchema = toTypedSchema(CreateSupplierSchema);

const imagePath = ref<string>();

const form = useForm({
  validationSchema: supplierSchema,
});

const createNewSupplier = async (supplier: SupplierT) => {
  isLoading.value = true;
  try {
    let imageBase64 = await getFileBytes(imagePath.value);
    await invoke<Res<string>>("create_supplier", {
      supplier: {
        full_name: supplier.fullname,
        email: supplier.email,
        phone_number: supplier.phoneNumber,
        address: supplier.address,
        image: `data:image/png;base64,${imageBase64}`,
      },
    });
    //
    info(
      `CREATE SUPPLIER: ${JSON.stringify({
        ...supplier,
        image: `data:image/png;base64,${imageBase64}`,
      })}`,
    );
    // toggle refresh
    updateQueryParams({
      refresh: "refresh-create-" + Math.random() * 9999,
    });
  } catch (err: any) {
    error("CREATE SUPPLIER: " + err.error);
  } finally {
    isLoading.value = false;
    hideModal();
  }
};

const hideModal = () => {
  store.setters.updateStore({ key: "show", value: false });
};

const onSubmit = form.handleSubmit((values) => {
  createNewSupplier(values);
});

const saveImage = (image: string) => {
  imagePath.value = image;
};
</script>

<template>
  <UiModalCard>
    <template #title>
      {{ t("s.c.title") }}
    </template>
    <template #content>
      <form class="h-full w-full flex flex-col gap-2" @submit="onSubmit">
        <UiUploader
          name="Image"
          :extensions="['png', 'jpeg', 'webp']"
          @on:save="saveImage"
        />
        <FormField v-slot="{ componentField }" name="fullname">
          <FormItem>
            <FormLabel>{{ t("g.fields.fullname") }}</FormLabel>
            <FormControl>
              <Input
                :placeholder="t('g.fields.fullname')"
                v-bind="componentField"
              />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="email">
          <FormItem>
            <FormLabel>{{ t("g.fields.email") }}</FormLabel>
            <FormControl>
              <Input placeholder="example@gmail.com" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="phoneNumber">
          <FormItem>
            <FormLabel>{{ t("g.fields.phone") }}</FormLabel>
            <FormControl>
              <Input placeholder="+2126********" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="address">
          <FormItem>
            <FormLabel>{{ t("g.fields.address") }}</FormLabel>
            <FormControl>
              <Input
                :placeholder="t('g.fields.address')"
                v-bind="componentField"
              />
            </FormControl>
          </FormItem>
        </FormField>
        <div class="w-full grid grid-cols-3 gap-2">
          <Button
            @click="hideModal"
            type="button"
            :disabled="isLoading"
            variant="outline"
          >
            {{ t("g.b.no") }}
          </Button>
          <Button :disabled="isLoading" type="submit" class="w-full col-span-2">
            {{ t("g.b.c") }}
          </Button>
        </div>
      </form>
    </template>
  </UiModalCard>
</template>
