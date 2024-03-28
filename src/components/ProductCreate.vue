<script setup lang="ts">
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { FormControl, FormField, FormItem, FormLabel } from "./ui/form";
import { toTypedSchema } from "@vee-validate/zod";
import UiModalCard from "./ui/UiModalCard.vue";
import UiUploader from "./ui/UiUploader.vue";
import { invoke } from "@tauri-apps/api";
import { Textarea } from "./ui/textarea";
import { useForm } from "vee-validate";
import { getFileBytes } from "@/utils/fs";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { useI18n } from "vue-i18n";
import { store } from "@/store";
import { ref } from "vue";
import type { Res } from "@/types";
import { CreateProductSchema, type ProductT } from "@/schemas/products.schema";
import { error, info } from "tauri-plugin-log-api";

const { t } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();

const isCreating = ref<boolean>(false);
const imagePath = ref<string>();
const quantity = ref<string>("");

const productSchema = toTypedSchema(CreateProductSchema);
const form = useForm({
  validationSchema: productSchema,
});

const createNewProduct = async (product: ProductT) => {
  isCreating.value = true;
  try {
    let imageBase64 = await getFileBytes(imagePath.value);
    let createRes = await invoke<Res<string>>("create_product", {
      product: {
        name: product.name,
        price: Number(product.price),
        description: product.description,
        min_quantity: product.minQuantity,
        image: `data:image/png;base64,${imageBase64}`,
      },
    });
    await invoke<Res<string>>("create_inventory", {
      mvm: {
        mvm_type: "IN",
        product_id: createRes.data,
        quantity: Number(quantity.value),
      },
    });
    info(
      `CREATE PRODUCT: ${JSON.stringify({
        ...product,
        image: `data:image/png;base64,${imageBase64}`,
        quantity: quantity.value,
      })}`,
    );
    // toggle refresh
    updateQueryParams({
      refresh: "refresh-create-" + Math.random() * 9999,
    });
  } catch (err: any) {
    error("CREATE PRODUCT: " + err.error);
  } finally {
    isCreating.value = false;
    hideModal();
  }
};

const hideModal = () => {
  store.setters.updateStore({ key: "show", value: false });
};

const onSubmit = form.handleSubmit((values) => {
  createNewProduct(values);
});

const setImage = (image: string) => {
  imagePath.value = image;
};
</script>

<template>
  <UiModalCard>
    <template #title>
      {{ t("p.c.title") }}
    </template>
    <template #content>
      <form class="h-full w-full flex flex-col gap-2" @submit="onSubmit">
        <UiUploader
          name="Image"
          :extensions="['png', 'jpeg', 'webp']"
          @on:save="setImage"
        />
        <FormField v-slot="{ componentField }" name="name">
          <FormItem>
            <FormLabel>{{ t("g.fields.name") }}</FormLabel>
            <FormControl>
              <Input
                :placeholder="t('g.fields.name')"
                v-bind="componentField"
              />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="price">
          <FormItem>
            <FormLabel>{{ t("g.fields.price") }}</FormLabel>
            <FormControl>
              <Input
                type="number"
                :placeholder="t('g.fields.price')"
                v-bind="componentField"
              >
                <template #unite> DH </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField name="">
          <FormItem>
            <FormLabel>{{ t("g.fields.init-quantity") }}</FormLabel>
            <FormControl>
              <Input
                type="number"
                :placeholder="t('g.fields.init-quantity')"
                v-model="quantity"
              >
                <template #unite> Item </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="minQuantity">
          <FormItem>
            <FormLabel>{{ t("g.fields.min-quantity") }}</FormLabel>
            <FormControl>
              <Input
                type="number"
                :placeholder="t('g.fields.min-quantity')"
                v-bind="componentField"
              >
                <template #unite> Item </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="description">
          <FormItem>
            <FormLabel>
              {{ t("g.fields.description") }}
            </FormLabel>
            <FormControl>
              <Textarea
                :placeholder="t('g.fields.description')"
                v-bind="componentField"
              ></Textarea>
            </FormControl>
          </FormItem>
        </FormField>
        <div class="w-full grid grid-cols-3 gap-2">
          <Button
            @click="hideModal"
            type="button"
            :disabled="isCreating"
            variant="outline"
          >
            {{ t("g.b.no") }}
          </Button>
          <Button
            :disabled="isCreating"
            type="submit"
            class="w-full col-span-2"
          >
            {{ t("g.b.c") }}
          </Button>
        </div>
      </form>
    </template>
  </UiModalCard>
</template>
