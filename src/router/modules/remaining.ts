import { $t } from "@/plugins/i18n";
const Layout = () => import("@/layout/index.vue");

export default [
  {
    path: "/dicom",
    component: Layout,
    redirect: "/worklist",
    meta: {
      title: $t("status.pureLoad"),
      showLink: false,
      rank: 102,
    },
  },
] satisfies Array<RouteConfigsTable>;
