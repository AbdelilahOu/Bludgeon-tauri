<script setup lang="ts">
import { computed, inject, type Ref } from "vue";
import { useRouter } from "vue-router";

import {
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev,
} from "@/components/ui/pagination";

import { Button } from "./button";
import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";

const router = useRouter();

const defaultPage = computed(() =>
  Number(router.currentRoute.value.query.page),
);

const { updateQueryParams } = useUpdateRouteQueryParams();

const rowsCount = inject<Ref<number>>("count");
const itemsCount = inject<Ref<number>>("itemsCount");

const goBackward = () => {
  if (defaultPage.value > 1) {
    updateQueryParams({ page: defaultPage.value - 1 });
  }
};

const goForward = () => {
  if (defaultPage.value < Math.ceil((rowsCount?.value ?? 1) / 17)) {
    updateQueryParams({ page: defaultPage.value + 1 });
  }
};

const goToPage = (page: number) => {
  updateQueryParams({ page });
};

const goToFirst = () => {
  updateQueryParams({ page: 1 });
};

const goToLast = () => {
  if (rowsCount?.value) {
    updateQueryParams({ page: Math.ceil(rowsCount?.value / 17) });
  }
};
</script>

<template>
  <div class="w-full h-fit flex items-center justify-center">
    <div class="w-fit m-auto fixed bottom-2 bg-white rounded-md shadow-md p-1">
      <Pagination
        v-slot="{ page }"
        :total="rowsCount"
        :sibling-count="1"
        show-edges
        :default-page="defaultPage"
        :items-per-page="itemsCount ?? 1"
      >
        <PaginationList v-slot="{ items }" class="flex items-center gap-1">
          <PaginationFirst @click="goToFirst" />
          <PaginationPrev @click="goBackward" />

          <template v-for="(item, index) in items">
            <PaginationListItem
              v-if="item.type === 'page'"
              :key="index"
              :value="item.value"
              as-child
            >
              <Button
                @click="goToPage(item.value)"
                class="w-10 h-10 p-0"
                :variant="item.value === page ? 'default' : 'outline'"
              >
                {{ item.value }}
              </Button>
            </PaginationListItem>
            <PaginationEllipsis v-else :key="item.type" :index="index" />
          </template>

          <PaginationNext @click="goForward" />
          <PaginationLast @click="goToLast" />
        </PaginationList>
      </Pagination>
    </div>
  </div>
</template>
