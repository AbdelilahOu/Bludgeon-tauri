import { chartOptions, optionsWoTicks } from "@/constants/chartOptions";
import { defineComponent, onBeforeMount, reactive, ref } from "vue";
import { ChartHolder } from "@/components/ChartHolder";
import { generateColor } from "@/utils/generateColor";
import { useStatsStore } from "@/stores/statsStore";
import { useModalStore } from "@/stores/modalStore";
import { ChartLine } from "@/components/ChartLine";
import { ChartBar } from "@/components/ChartBar";
import { UiCard } from "@/components/ui/UiCard";
import type { clientT } from "@/types";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";
import { getWeekDay } from "@/utils/formatDate";
import _ from "lodash";

export const ClientDetails = defineComponent({
  name: "ClientDetails",
  components: { UiCard, ChartHolder, ChartBar, ChartLine },
  setup() {
    const id = useRoute().params.id;
    const statsStore = useStatsStore();

    const client = ref<clientT | null>(null);

    const ProductsStats = reactive({
      products: [] as string[],
      dates: [] as string[],
      data: {} as { [key: string]: number[] },
    });

    const DailyStats = reactive({
      data: [] as number[],
      keys: [] as string[],
    });

    async function getProductPerMonth(id: number) {
      const data: any[] = await invoke("get_c_product_month", { id });

      const existingDates = _.keys(_.groupBy(data, "month"));
      const existingProducts = _.keys(_.groupBy(data, "name"));
      const dataPerProduct = _.mapValues(_.groupBy(data, "name"), (value) =>
        _.reduce(
          value,
          (pr, cr) => {
            if (!pr) pr = [];
            pr.push(cr.quantity);
            return pr;
          },
          [] as number[]
        )
      );

      return {
        data: dataPerProduct,
        dates: existingDates,
        products: existingProducts,
      };
    }

    async function getDailyExpenses(id: number) {
      const result: { day: string; expense: number }[] = await invoke(
        "get_c_week_expenses",
        { id }
      );
      // date related
      const nextDay = new Date().getDay() == 6 ? 0 : new Date().getDay() + 1;
      const resultMap = new Map<string, number>();
      const weekDays = [0, 1, 2, 3, 4, 5, 6];

      for (const index of weekDays) {
        resultMap.set(getWeekDay(index), 0);
      }

      for (const { day, expense } of result) {
        console.log(
          day,
          new Date(day).toLocaleDateString("en-us", {
            weekday: "short",
          })
        );
        resultMap.set(
          new Date(day).toLocaleDateString("en-us", {
            weekday: "short",
          }),
          expense
        );
      }

      // @ts-ignore
      const K = _.keys(Object.fromEntries(resultMap));
      // @ts-ignore
      const V = _.values(Object.fromEntries(resultMap));
      const rearrangedKeys = K.slice(nextDay).concat(K.slice(0, nextDay));
      const rearrangedValues = V.slice(nextDay).concat(V.slice(0, nextDay));

      return {
        keys: rearrangedKeys,
        values: rearrangedValues,
      };
    }

    onBeforeMount(async () => {
      const productStats = await getProductPerMonth(Number(id));
      const dailyStats = await getDailyExpenses(Number(id));

      DailyStats.keys = dailyStats.keys;
      DailyStats.data = dailyStats.values;

      console.log(dailyStats);

      ProductsStats.data = productStats.data;
      ProductsStats.dates = productStats.dates;
      ProductsStats.products = productStats.products;
    });

    const toggleThisClient = (client: clientT | null, name: string) => {
      useModalStore().updateModal({ key: "show", value: true });
      useModalStore().updateModal({ key: "name", value: name });
      useModalStore().updateClientRow(client);
    };

    onBeforeMount(async () => {
      try {
        const res = await invoke<clientT>("get_client", { id: Number(id) });
        if (res.id) {
          client.value = res;
        }
      } catch (error) {
        console.log(error);
      }
    });

    return () => (
      <main class="w-full h-full px-3 py-1">
        <div class="w-full h-full text-black grid gap-4 xl:grid-cols-[400px_2px_1fr] xl:grid-rows-1 grid-rows-[260px_2px_1fr] grid-cols-1 print:pr-12">
          <div class="w-full grid-cols-[400px_1fr] xl:grid-rows-[258px_1fr] xl:grid-cols-1 items-start justify-start gap-3 grid">
            <UiCard
              title="Client information"
              updateItem={() => {
                toggleThisClient(client.value, "ClientUpdate");
              }}
              item={client.value}
            />
            <div class="w-full flex items-end xl:items-start h-full">
              <ChartLine
                id="sjdlsdksd"
                chartData={{
                  labels: DailyStats.keys,
                  datasets: [
                    {
                      label: "daily expenses",
                      backgroundColor: generateColor(),
                      borderColor: generateColor().replace("0.2", "0.5"),
                      data: DailyStats.data,
                      borderWidth: 2,
                      lineTension: 0.4,
                      pointRadius: 1,
                    },
                  ],
                }}
                chartOptions={optionsWoTicks}
              />
            </div>
          </div>
          <div class="xl:border-l-2 border-b-2"></div>
          <div class="w-full">
            <ChartHolder>
              {{
                default: () => (
                  <ChartBar
                    id="inventory-mouvements-for-past-three-months"
                    chartData={{
                      labels: ProductsStats.dates,
                      datasets: ProductsStats.products.map((product) => {
                        const color = generateColor();
                        return {
                          label: product,
                          backgroundColor: color,
                          borderColor: color.replace("0.2", "0.5"),
                          data: ProductsStats.data[product],
                          borderWidth: 2,
                        };
                      }),
                    }}
                    chartOptions={chartOptions}
                  />
                ),
                title: () => (
                  <h1 class="m-2 w-full text-center text-base font-medium">
                    <i>Products baught last three months</i>
                  </h1>
                ),
              }}
            </ChartHolder>
          </div>
        </div>
      </main>
    );
  },
});
