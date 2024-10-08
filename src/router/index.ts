// import "@/utils/sso";
import NProgress from "@/utils/progress";
import { buildHierarchyTree } from "@/utils/tree";
import remainingRouter from "./modules/remaining";
import { usePermissionStoreHook } from "@/store/modules/permission";
import {
  ascending,
  getHistoryMode,
  formatTwoStageRoutes,
  formatFlatteningRoutes,
} from "./utils";
import {
  type Router,
  createRouter,
  type RouteRecordRaw,
  type RouteComponent,
} from "vue-router";

const modules: Record<string, any> = import.meta.glob(
  ["./modules/**/*.ts", "!./modules/**/remaining.ts"],
  {
    eager: true,
  },
);
const routes = [];

Object.keys(modules).forEach((key) => {
  routes.push(modules[key].default);
});
export const constantRoutes: Array<RouteRecordRaw> = formatTwoStageRoutes(
  formatFlatteningRoutes(buildHierarchyTree(ascending(routes.flat(Infinity)))),
);

export const constantMenus: Array<RouteComponent> = ascending(
  routes.flat(Infinity),
).concat(...remainingRouter);

export const remainingPaths = Object.keys(remainingRouter).map((v) => {
  return remainingRouter[v].path;
});

export const router: Router = createRouter({
  history: getHistoryMode(import.meta.env.VITE_ROUTER_HISTORY),
  routes: constantRoutes.concat(...(remainingRouter as any)),
  strict: true,
  scrollBehavior(to, from, savedPosition) {
    return new Promise((resolve) => {
      if (savedPosition) {
        return savedPosition;
      } else {
        if (from.meta.saveSrollTop) {
          const top: number =
            document.documentElement.scrollTop || document.body.scrollTop;
          resolve({ left: 0, top });
        }
      }
    });
  },
});

export function resetRouter() {
  router.getRoutes().forEach((route) => {
    const { name, meta } = route;
    if (name && router.hasRoute(name) && meta?.backstage) {
      router.removeRoute(name);
      router.options.routes = formatTwoStageRoutes(
        formatFlatteningRoutes(
          buildHierarchyTree(ascending(routes.flat(Infinity))),
        ),
      );
    }
  });
  usePermissionStoreHook().clearAllCachePage();
}

router.beforeEach((to: ToRouteType, _from, next) => {
  next();
});

router.afterEach(() => {
  NProgress.done();
});

export default router;
