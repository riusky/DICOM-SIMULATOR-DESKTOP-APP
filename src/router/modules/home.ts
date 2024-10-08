import { $t } from "@/plugins/i18n";
const Layout = () => import("@/layout/index.vue");

export default {
  path: "/",
  name: "Home",
  component: Layout,
  redirect: "/dicom",
  meta: {
    icon: "ep:home-filled",
    title: $t("menus.pureHome"),
    rank: 0,
  },
  children: [
    {
      path: "/dicom",
      name: "dicom",
      component: () => import("@/views/dicom/index.vue"),
      meta: {
        title: $t("dicom.dicom"),
      },
    },
    {
      path: "/hl7",
      name: "hl7",
      component: () => import("@/views/hl7/index.vue"),
      meta: {
        title: $t("dicom.dicom"),
      },
    },
    {
      path: "/dicom/mpps",
      name: "mpps",
      component: () => import("@/views/dicom/mpps/index.vue"),
      meta: {
        title: $t("dicom.mpps"),
        showBackButton: true,
      },
    },
    {
      path: "/dicom/worklist",
      name: "worklist",
      component: () => import("@/views/dicom/worklist/index.vue"),
      meta: {
        title: $t("dicom.worklist"),
        showBackButton: true,
      },
    },
    {
      path: "/worklistsetting",
      name: "worklistsetting",
      component: () => import("@/views/dicom/setting/WorklistSetting.vue"),
      meta: {
        title: $t("dicom.worklist"),
      },
    },
    {
      path: "/mimSetting",
      name: "setting",
      component: () => import("@/views/dicom/setting/MimSetting.vue"),
      meta: {
        title: $t("dicom.worklist"),
      },
    },
    {
      path: "/settinghl7",
      name: "hl7setting",
      component: () => import("@/views/dicom/setting/HL7Setting.vue"),
      meta: {
        title: $t("dicom.worklist"),
      },
    },
    {
      path: "/settingmessage",
      name: "settingmessage",
      component: () => import("@/views/dicom/setting/Hl7MessageSetting.vue"),
      meta: {
        title: $t("dicom.worklist"),
      },
    },
    {
      path: "/dicom/cstore",
      name: "cstore",
      component: () => import("@/views/dicom/mpps/PatientHeadless.vue"),
      meta: {
        title: $t("dicom.worklist"),
      },
    },
  ],
} satisfies RouteConfigsTable;
