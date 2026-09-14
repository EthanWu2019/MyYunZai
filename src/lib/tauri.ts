// Tauri IPC 封装 + 类型

import { invoke } from "@tauri-apps/api/core";

export interface AppPaths {
  redis_exe: string;
  redis_conf: string;
  yunzai_dir: string;
  yunzai_node: string;
  napcat_dir: string;
  napcat_log_dir: string;
  yunzai_log_dir: string;
}

export interface AppSettings {
  close_action: string;
  autostart: boolean;
  auto_start_on_launch: boolean;
  napcat_elevated: boolean;
}

export interface AppConfig {
  paths: AppPaths;
  settings: AppSettings;
}

export interface ComponentStatus {
  running: boolean;
  pid: number | null;
}

export interface StatusSnapshot {
  redis: ComponentStatus;
  yunzai: ComponentStatus;
  napcat: ComponentStatus;
  qq: ComponentStatus;
  webui_connected: boolean;
  websocket_connected: boolean;
  timestamp: number;
}

export interface SetupProgress {
  current_step: number;
  total_steps: number;
  steps: SetupStep[];
  overall_status: 'Pending' | 'Running' | 'Completed' | 'Failed' | 'Skipped';
  error_message?: string;
}

export interface SetupStep {
  id: string;
  name: string;
  status: 'Pending' | 'Running' | 'Completed' | 'Failed' | 'Skipped';
  progress: number;
  message: string;
}

export interface PluginInfo {
  name: string;
  author: string;
  repo_url: string;
  category: 'Framework' | 'Featured' | 'Function' | 'Game' | 'WordGame' | 'JsPlugin';
  description: string;
  recommended: boolean;
}

export const api = {
  getConfig: () => invoke<AppConfig>("get_config"),
  updatePaths: (
    paths: AppPaths,
    settings?: AppSettings,
  ) => invoke<void>("update_paths", { paths, settings }),
  getStatus: () => invoke<StatusSnapshot>("get_status"),
  startAll: () => invoke<StatusSnapshot>("start_all"),
  stopBot: () => invoke<StatusSnapshot>("stop_bot"),
  openNapcatWebui: () => invoke<void>("open_napcat_webui"),
  revealInExplorer: (path: string) => invoke<void>("reveal_in_explorer", { path }),
  readLogTail: (source: "yunzai" | "napcat", lines = 200) =>
    invoke<string>("read_log_tail", { source, lines }),
  isAutostartEnabled: () => invoke<boolean>("is_autostart_enabled"),
  setAutostart: (enable: boolean) => invoke<void>("set_autostart", { enable }),
  minimizeToTray: () => invoke<void>("minimize_to_tray"),
  quitApp: () => invoke<void>("quit_app"),
  runSetup: (paths: AppPaths, selectedPlugins: string[]) =>
    invoke<void>("run_setup", { paths, selectedPlugins }),
  getAvailablePlugins: () => invoke<PluginInfo[]>("get_available_plugins"),
};
