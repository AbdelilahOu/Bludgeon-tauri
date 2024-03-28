<script setup lang="ts">
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { FormControl, FormField, FormItem, FormLabel } from "./ui/form";
import { useI18n } from "vue-i18n";
import { ref } from "vue";
import { toTypedSchema } from "@vee-validate/zod";
import UiModalCard from "./ui/UiModalCard.vue";
import { invoke } from "@tauri-apps/api";
import { useForm } from "vee-validate";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { store } from "@/store";
import { z } from "zod";
import type { SupplierT } from "@/schemas/supplier.schema";
import { useRoute } from "vue-router";
import type { Res } from "@/types";
import { error, info } from "tauri-plugin-log-api";

const { updateQueryParams } = useUpdateRouteQueryParams();
const { t } = useI18n();
const route = useRoute();
const isLoading = ref<boolean>(false);

const supplierSchema = toTypedSchema(
  z.object({
    fullname: z
      .string()
      .min(2)
      .max(50)
      .default(route.query.fullname as string),
    email: z.string().default((route.query.email as string) ?? ""),
    phoneNumber: z.string().default((route.query.phoneNumber as string) ?? ""),
    address: z.string().default((route.query.address as string) ?? ""),
    image: z.string().default((route.query.image as string) ?? ""),
  }),
);

const form = useForm({
  validationSchema: supplierSchema,
});

const updateTheSupplier = async (supplier: SupplierT) => {
  try {
    await invoke<Res<any>>("update_supplier", {
      supplier: {
        id: route.query.id,
        full_name: supplier.fullname,
        email: supplier.email,
        phone_number: supplier.phoneNumber,
        address: supplier.address,
        image: supplier.image,
      },
    });
    //
    info(
      `UPDATE SUPPLIER: ${JSON.stringify({
        id: route.query.id,
        full_name: supplier.fullname,
        email: supplier.email,
        phone_number: supplier.phoneNumber,
        address: supplier.address,
        image: supplier.image,
      })}`,
    );
    // toggle refresh
    updateQueryParams({
      refresh: "refresh-update-" + Math.random() * 9999,
    });
  } catch (err: any) {
    error("UPDATE SUPPLIER: " + err.error);
  } finally {
    hideModal();
  }
};

const hideModal = () => {
  store.setters.updateStore({ key: "show", value: false });
};

const onSubmit = form.handleSubmit((values) => {
  updateTheSupplier(values);
});
</script>

<template>
  <UiModalCard>
    <template #title>
      {{ t("s.u.title") }}
    </template>
    <template #content>
      <form class="h-full w-full flex flex-col gap-2" @submit="onSubmit">
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
            {{ t("g.b.u", { name: $route.query.fullname }) }}
          </Button>
        </div>
      </form>
    </template>
  </UiModalCard>
</template>
