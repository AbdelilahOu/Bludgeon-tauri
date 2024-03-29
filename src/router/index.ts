import { createRouter, createWebHistory } from "vue-router";
import NotificationsView from "@/views/NotificationsView.vue";
import InvoiceDetails from "@/views/InvoiceDetails.vue";
import ClientDetails from "@/views/ClientDetails.vue";
import SellerDetails from "@/views/SellerDetails.vue";
import OrdersDetails from "@/views/OrderDetails.vue";
import ProductsView from "@/views/ProductsView.vue";
import InvoicesView from "@/views/InvoicesView.vue";
import CommenLayout from "@/layouts/CommenLayout.vue";
import SellersView from "@/views/SellersView.vue";
import ClientsView from "@/views/ClientsView.vue";
import OrdersView from "@/views/OrdersView.vue";
import InventoryView from "@/views/InventoryView.vue";
import StatsView from "@/views/StatsView.vue";
import HomeView from "@/views/HomeView.vue";
import RootLayout from "@/layouts/RootLayout.vue";
import AuthView from "@/views/AuthView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "Index",
      component: RootLayout,
      children: [
        {
          path: "Products",
          name: "Products",
          component: ProductsView,
        },
        {
          path: "Home",
          name: "Home",
          component: HomeView,
        },
        {
          path: "Clients",
          name: "Clients",
          component: CommenLayout,
          children: [
            {
              path: "all",
              name: "Clients",
              component: ClientsView,
            },
            {
              path: "client/:id",
              name: "ClientDetails",
              component: ClientDetails,
            },
          ],
        },
        {
          path: "Sellers",
          name: "Sellers",
          component: CommenLayout,
          children: [
            {
              path: "all",
              name: "Sellers",
              component: SellersView,
            },
            {
              path: "seller/:id",
              name: "SellerDetails",
              component: SellerDetails,
            },
          ],
        },
        {
          path: "Inventory",
          name: "Inventory",
          component: InventoryView,
        },
        {
          path: "Orders/",
          name: "Orders",
          component: CommenLayout,
          children: [
            {
              path: "all",
              name: "Orders",
              component: OrdersView,
            },
            {
              path: "order/:id",
              name: "OrdersDetails",
              component: OrdersDetails,
            },
          ],
        },
        {
          path: "Stats",
          name: "Stats",
          component: StatsView,
        },
        {
          path: "Invoices/",
          name: "Invoice",
          component: CommenLayout,
          children: [
            {
              path: "all",
              name: "Invoices",
              component: InvoicesView,
            },
            {
              path: "invoice/:id",
              name: "InvoiceDetails",
              component: InvoiceDetails,
            },
          ],
        },
        {
          path: "Notifications",
          name: "Notifications",
          component: NotificationsView,
        },
        // {
        //   path: "Payment",
        //   name: "Payment",
        //   component: PaymentView,
        // },
        // {
        //   path: "Upcomings",
        //   name: "Upcomings",
        //   component: UpcomingsView,
        // },
      ],
    },
    {
      path: "/Auth",
      name: "Auth",
      component: AuthView,
    },
  ],
});

export default router;
