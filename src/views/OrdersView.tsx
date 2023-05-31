import { defineComponent, onBeforeMount, ref, Transition } from "vue";
import { OrdersTable } from "@/components/OrdersTable";
import { useOrdersStore } from "@/stores/orderStore";
import { UiButton } from "@/components/ui/UiButton";
import { useModalStore } from "@/stores/modalStore";
import { UiInput } from "@/components/ui/UiInput";
import UiIcon from "@/components/ui/UiIcon.vue";
import { storeToRefs } from "pinia";
import { globalTranslate } from "@/utils/globalTranslate";
import { useProductStore } from "@/stores/productStore";
import { useSellerStore } from "@/stores/sellerStore";

export const OrdersView = defineComponent({
  name: "Orders",
  components: {
    OrdersTable,
    UiButton,
    UiInput,
    UiIcon,
  },
  setup() {
    //
    const modalStore = useModalStore();
    const OrdersStore = useOrdersStore();
    const { orders } = storeToRefs(OrdersStore);
    //
    const searchQuery = ref<string>("");
    //
    onBeforeMount(() => {
      if (!orders.value.length) {
        OrdersStore.getAllOrders();
        useProductStore().getAllProducts();
        useSellerStore().getAllSellers();
      }
    });
    //
    const updateModal = (name: string) => {
      modalStore.updateModal({ key: "show", value: true });
      modalStore.updateModal({ key: "name", value: name });
    };
    //

    return () => (
      <main class="w-full h-full px-3">
        <div class="w-full h-full flex flex-col items-start justify-start">
          <Transition appear>
            <div class="flex justify-between w-full gap-9 my-1">
              <div class="w-1/3">
                <UiInput
                  IsEmpty={false}
                  OnInputChange={(value) =>
                    (searchQuery.value =
                      typeof value !== "string"
                        ? JSON.stringify(value)
                        : value.toLocaleLowerCase())
                  }
                  Type="text"
                  PlaceHolder={globalTranslate("Global.search")}
                >
                  <UiIcon
                    class=" fill-gray-400 cursor-default hover:bg-white"
                    name="search"
                  />
                </UiInput>
              </div>
              <div class="w-1/4 flex gap-2">
                <UiButton
                  colorTheme="a"
                  Click={() => updateModal("OrderCreate")}
                >
                  <UiIcon
                    class=" fill-gray-900 cursor-default hover:bg-transparent"
                    name="add"
                  />{" "}
                  {globalTranslate("Orders.index.addButton")}
                </UiButton>
              </div>
            </div>
          </Transition>
          <Transition appear>
            <OrdersTable
              FilterParam={searchQuery.value}
              Orders={orders.value}
            />
          </Transition>
        </div>
      </main>
    );
  },
});
