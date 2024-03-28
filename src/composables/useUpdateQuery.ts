import { useRouter } from "vue-router";

export function useUpdateRouteQueryParams() {
  const router = useRouter();
  const route = router.currentRoute.value;

  const updateQueryParams = (query: Record<any, any>) => {
    router.push({
      path: route.path,
      query: { ...route.query, ...query },
    });
  };

  return {
    updateQueryParams,
  };
}
