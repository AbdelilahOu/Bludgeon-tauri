import { useUpdateRouteQueryParams } from "@/composables/useUpdateQuery";
import { InvoicesTable } from "@/components/InvoicesTable";
import { globalTranslate } from "@/utils/globalTranslate";
import { Button } from "@/components/ui/button";
import type { invoiceT, withCount } from "@/types";
import { Input } from "@/components/ui/input";
import UiIcon from "@/components/ui/UiIcon.vue";
import { invoke } from "@tauri-apps/api";
import { useRouter } from "vue-router";
import { store } from "@/store";
import {
  type WatchStopHandle,
  defineComponent,
  onBeforeMount,
  onUnmounted,
  Transition,
  onMounted,
  computed,
  provide,
  watch,
  ref,
} from "vue";

export const InvoicesView = defineComponent({
  name: "Invoices",
  components: {
    InvoicesTable,
    Button,
    Input,
    UiIcon,
  },
  setup() {
    //

    const router = useRouter();
    const { updateQueryParams } = useUpdateRouteQueryParams();

    const invoices = ref<invoiceT[]>([]);
    const searchQuery = ref<string>("");
    const page = computed(() => Number(router.currentRoute.value.query.page));
    const refresh = computed(() => router.currentRoute.value.query.refresh);

    const totalRows = ref<number>(0);

    let unwatch: WatchStopHandle | null = null;
    //
    provide("count", totalRows);

    onBeforeMount(() => getInvoices(page.value));

    onMounted(() => {
      unwatch = watch([page, refresh], ([p]) => {
        if (p && p > 0) getInvoices(p);
      });
    });

    onUnmounted(() => {
      if (unwatch) unwatch();
    });

    async function getInvoices(page: number = 1) {
      try {
        const res = await invoke<withCount<invoiceT[]>>("get_invoices", {
          page,
        });
        if (res.data.length) {
          invoices.value = res.data;
          totalRows.value = res.count;
        }
      } catch (error) {
        console.log(error);
      }
    }

    const uploadCSV = () => {
      store.setters.updateStore({ key: "name", value: "CsvUploader" });
      store.setters.updateStore({ key: "show", value: true });
      updateQueryParams({ table: "invoices" });
    };
    //
    const updateModal = (name: string) => {
      store.setters.updateStore({ key: "show", value: true });
      store.setters.updateStore({ key: "name", value: name });
    };
    //

    return () => (
      <main class="w-full h-full">
        <div class="w-full h-full flex flex-col items-start justify-start">
          <Transition appear>
            <div class="flex justify-between w-full gap-9 mb-1">
              <div class="w-1/3">
                <Input
                  IsEmpty={false}
                  OnInputChange={(value) =>
                    (searchQuery.value =
                      typeof value !== "string"
                        ? JSON.stringify(value)
                        : value.toLocaleLowerCase())
                  }
                  type="text"
                  placeHolder={globalTranslate("Global.search")}
                >
                  <UiIcon
                    class=" fill-gray-400 cursor-default hover:bg-white"
                    name="search"
                  />
                </Input>
              </div>
              <div class="w-1/3 grid grid-cols-[60px_1fr] gap-1">
                <Button colorTheme="primary" onClick={() => uploadCSV()}>
                  <span
                    class={
                      "fill-sky-400 transition-all duration-200 scale-[0.8] group-hover:fill-sky-600"
                    }
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="32"
                      height="32"
                      viewBox="0 0 256 256"
                    >
                      <path d="m217.5 170.3l-20 48a5.9 5.9 0 0 1-11 0l-20-48a6 6 0 0 1 11-4.6l14.5 34.7l14.5-34.7a6 6 0 1 1 11 4.6ZM76 206.1a15.1 15.1 0 0 1-10 3.9c-8.8 0-16-8.1-16-18s7.2-18 16-18a15.1 15.1 0 0 1 10 3.9a5.9 5.9 0 0 0 8.5-.4a6 6 0 0 0-.5-8.5a26.9 26.9 0 0 0-18-7c-15.4 0-28 13.5-28 30s12.6 30 28 30a26.9 26.9 0 0 0 18-7a6 6 0 0 0 .5-8.5a5.9 5.9 0 0 0-8.5-.4Zm53.2-20.4c-7.8-2-11.2-3.3-11.2-5.7c0-6.1 5.6-7 9-7a19.7 19.7 0 0 1 11.2 3.6a6 6 0 0 0 7.6-9.2A30 30 0 0 0 127 161c-12.4 0-21 7.8-21 19s11.6 15.1 20.1 17.3S138 201 138 204s0 7-11 7a20 20 0 0 1-11.2-3.6a6 6 0 1 0-7.6 9.2A30 30 0 0 0 127 223c14.4 0 23-7.1 23-19s-12.5-16.1-20.8-18.3ZM202 94h-50a6 6 0 0 1-6-6V38H56a2 2 0 0 0-2 2v88a6 6 0 0 1-12 0V40a14 14 0 0 1 14-14h96a5.6 5.6 0 0 1 4.2 1.8l56 55.9A6 6 0 0 1 214 88v40a6 6 0 0 1-12 0Zm-44-12h35.5L158 46.5Z" />
                    </svg>
                  </span>
                </Button>
                <Button
                  colorTheme="a"
                  onClick={() => updateModal("InvoiceCreate")}
                >
                  <UiIcon
                    class=" fill-gray-900 cursor-default hover:bg-transparent"
                    name="add"
                  />{" "}
                  {globalTranslate("Invoices.index.addButton")}
                </Button>
              </div>
            </div>
          </Transition>
          <Transition appear>
            <InvoicesTable Invoices={invoices.value} />
          </Transition>
        </div>
      </main>
    );
  },
});
