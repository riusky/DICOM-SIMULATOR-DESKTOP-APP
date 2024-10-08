export {};
/**
 * 全局类型声明，无需引入直接在 `.vue` 、`.ts` 、`.tsx` 文件使用即可获得类型提示
 */
declare global {
  /**
   * 全局自定义环境变量的类型声明
   * @see {@link https://pure-admin.github.io/pure-admin-doc/pages/config/#%E5%85%B7%E4%BD%93%E9%85%8D%E7%BD%AE}
   */
  interface ViteEnv {
    VITE_PORT: number;
    VITE_PUBLIC_PATH: string;
    VITE_ROUTER_HISTORY: string;
    VITE_CDN: boolean;
    VITE_HIDE_HOME: string;
    VITE_COMPRESSION: ViteCompression;
  }
  /**
   * 对应 `public/platform-config.json` 文件的类型声明
   * @see {@link https://pure-admin.github.io/pure-admin-doc/pages/config/#platform-config-json}
   */
  interface PlatformConfigs {
    Version?: string;
    Title?: string;
    FixedHeader?: boolean;
    HiddenSideBar?: boolean;
    MultiTagsCache?: boolean;
    MaxTagsLevel?: number;
    KeepAlive?: boolean;
    Locale?: string;
    Layout?: string;
    Theme?: string;
    DarkMode?: boolean;
    OverallStyle?: string;
    Grey?: boolean;
    Weak?: boolean;
    HideTabs?: boolean;
    HideFooter?: boolean;
    Stretch?: boolean | number;
    SidebarStatus?: boolean;
    EpThemeColor?: string;
    ShowLogo?: boolean;
    ShowModel?: string;
    MenuArrowIconNoTransition?: boolean;
    CachingAsyncRoutes?: boolean;
    TooltipEffect?: Effect;
    ResponsiveStorageNameSpace?: string;
    MenuSearchHistory?: number;
  }
  /**
   * 与 `PlatformConfigs` 类型不同，这里是缓存到浏览器本地存储的类型声明
   * @see {@link https://pure-admin.github.io/pure-admin-doc/pages/config/#platform-config-json}
   */
  interface StorageConfigs {
    version?: string;
    title?: string;
    fixedHeader?: boolean;
    hiddenSideBar?: boolean;
    multiTagsCache?: boolean;
    keepAlive?: boolean;
    locale?: string;
    layout?: string;
    theme?: string;
    darkMode?: boolean;
    grey?: boolean;
    weak?: boolean;
    hideTabs?: boolean;
    hideFooter?: boolean;
    sidebarStatus?: boolean;
    epThemeColor?: string;
    themeColor?: string;
    overallStyle?: string;
    showLogo?: boolean;
    showModel?: string;
    menuSearchHistory?: number;
    username?: string;
  }
  interface GlobalPropertiesApi {
    $echarts: ECharts;
    $storage: ResponsiveStorage;
    $config: PlatformConfigs;
  }

  // 定义 ApiResponse 类型
  interface ApiResponse<T> {
    success: boolean;
    message: string;
    data: T | null;
    error: string | null;
  }
  interface WorklistEntry {
    id?: {
      tb: string;
      id: {
        String: string;
      };
    };
    name: string;
    calling_ae_title: string;
    worklist_ae_title: string;
    worklist_ip: string;
    worklist_port: string;
  }

  interface MppsEntry {
    id?: {
      tb: string;
      id: {
        String: string;
      };
    };
    name: string;
    calling_ae_title: string;
    worklist_ae_title: string;
    worklist_ip: string;
    worklist_port: string;
  }

  interface MppsRecordEntry {
    id: {
      tb: string;
      id: {
        String: string;
      };
    };
    AccessionNumber: string;
    RequestedProcedureDescription: string;
    PatientName: string;
    PatientID: string;
    PatientBirthDate: string;
    PatientSex: string;
    Modality: string;
    ScheduledStationAETitle: string;
    ScheduledProcedureStepStartDate: string;
    ScheduledPerformingPhysicianName: string;
    StudyInstanceUID: string;
    status: string;
  }
}
