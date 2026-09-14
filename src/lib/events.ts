// 全局事件类型
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import type { StatusSnapshot, SetupProgress } from "./tauri";

export interface LogEvent {
  source: string;
  file: string;
  text: string;
}

export type LogListener = (e: LogEvent) => void;
export type StatusListener = (e: StatusSnapshot) => void;

export async function onLog(listener: LogListener): Promise<UnlistenFn> {
  return listen<LogEvent>("log:append", (e) => listener(e.payload));
}

export async function onStatus(listener: StatusListener): Promise<UnlistenFn> {
  return listen<StatusSnapshot>("status:update", (e) => listener(e.payload));
}

export async function onStartOk(listener: () => void): Promise<UnlistenFn> {
  return listen("start:ok", () => listener());
}

export async function onSetupProgress(
  listener: (progress: SetupProgress) => void,
): Promise<UnlistenFn> {
  return listen<SetupProgress>("setup:progress", (e) => listener(e.payload));
}

export async function onSetupComplete(listener: () => void): Promise<UnlistenFn> {
  return listen("setup:complete", () => listener());
}

export async function onSetupFailed(
  listener: (msg: string) => void,
): Promise<UnlistenFn> {
  return listen<string>("setup:failed", (e) => listener(e.payload));
}

export async function onStartFailed(
  listener: (msg: string) => void,
): Promise<UnlistenFn> {
  return listen<string>("start:failed", (e) => listener(e.payload));
}

export async function onStopOk(listener: () => void): Promise<UnlistenFn> {
  return listen("stop:ok", () => listener());
}

export async function onStopFailed(
  listener: (msg: string) => void,
): Promise<UnlistenFn> {
  return listen<string>("stop:failed", (e) => listener(e.payload));
}
