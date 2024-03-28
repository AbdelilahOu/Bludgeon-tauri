<script setup lang="ts">
import { useI18n } from "vue-i18n";
import UiPagination from "./ui/UiPagination.vue";
import { FilePenLine, BookUser, Trash2 } from "lucide-vue-next";
import { store } from "@/store";
import type { ClientT } from "@/schemas/client.schema";
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { SheetTrigger } from "@/components/ui/sheet";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

defineProps<{
  clients: ClientT[];
}>();

const { t } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();

const toggleThisClient = (client: ClientT, name: string) => {
  updateQueryParams({
    id: client.id,
    fullname: client.fullname,
    email: client.email,
    phoneNumber: client.phoneNumber,
    address: client.address,
  });
  store.setters.updateStore({ key: "name", value: name });
  store.setters.updateStore({ key: "show", value: true });
};

const toggleCLientProfile = (client: ClientT) => {
  updateQueryParams({
    id: client.id,
    fullname: client.fullname,
    email: client.email,
    phoneNumber: client.phoneNumber,
    address: client.address,
    // image: client.image,
  });
};
</script>

<template>
  <div>
    <table>
      <thead>
        <tr>
          <th class="small"></th>
          <th>{{ t("g.fields.fullname") }}</th>
          <th>{{ t("g.fields.email") }}</th>
          <th>{{ t("g.fields.phone") }}</th>
          <th>{{ t("g.fields.address") }}</th>
          <th>{{ t("g.fields.credit") }}</th>
          <th class="small">{{ t("g.fields.actions") }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(client, index) in clients" v-fade="index" :key="client.id">
          <td class="p-2 flex justify-center">
            <Avatar>
              <AvatarImage :src="client.image ?? ''" />
              <AvatarFallback class="text-xs">
                {{ client.fullname.substring(0, 5) }}
              </AvatarFallback>
            </Avatar>
          </td>
          <td class="p-2 whitespace-nowrap font-medium">
            {{ client?.fullname }}
          </td>
          <td class="p-2">
            {{ client.email }}
          </td>
          <td class="p-2">
            {{ client.phoneNumber }}
          </td>
          <td class="p-2">
            {{ client.address }}
          </td>
          <td class="p-2 whitespace-nowrap">{{ client.credi }} DH</td>
          <td class="p-2">
            <div class="flex space-x-3">
              <Trash2
                @click="toggleThisClient(client, 'ClientDelete')"
                class="cursor-pointer text-gray-800"
                :size="22"
              />
              <FilePenLine
                @click="toggleThisClient(client, 'ClientUpdate')"
                class="cursor-pointer text-gray-800"
                :size="22"
              />
              <!-- <SheetTrigger @click="(e) => toggleCLientProfile(client)">
                <BookUser class="text-gray-800" :size="22" />
              </SheetTrigger> -->
            </div>
          </td>
        </tr>
      </tbody>
    </table>
    <UiPagination />
  </div>
</template>
