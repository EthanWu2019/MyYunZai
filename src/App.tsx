// 海绵酱控制台 · 主面板
import { useEffect, useState, useCallback } from "react";
import { AnimatePresence } from "framer-motion";
import {
  Activity,
  Settings,
  Power,
  PowerOff,
  ExternalLink,
  FolderOpen,
  X,
} from "lucide-react";
import {
  api,
  type AppConfig,
  type StatusSnapshot,
  type ComponentStatus,
} from "./lib/tauri";
import {
  onStatus,
  onLog,
  onStartOk,
  onStartFailed,
  onStopOk,
  onStopFailed,
  type LogEvent,
} from "./lib/events";
import StatusCard from "./components/StatusCard";
import LogPanel from "./components/LogPanel";
import SettingsModal from "./components/SettingsModal";
import NapcatWebview from "./components/NapcatWebview";

const DEFAULT_SNAPSHOT: StatusSnapshot = {
  redis: { running: false, pid: null },
  yunzai: { running: false, pid: null },
  napcat: { running: false, pid: null },
  qq: { running: false, pid: null },
  webui_connected: false,
  websocket_connected: false,
  timestamp: 0,
};

export default function App() {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [status, setStatus] = useState<StatusSnapshot>(DEFAULT_SNAPSHOT);
  const [logs, setLogs] = useState<LogEvent[]>([]);
  const [busy, setBusy] = useState<"start" | "stop" | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [activeTab, setActiveTab] = useState<"overview" | "webview">(
    "overview",
  );

  // 初始加载
  useEffect(() => {
    void (async () => {
      try {
        const cfg = await api.getConfig();
        setConfig(cfg);
        const s = await api.getStatus();
        setStatus(s);
      } catch (e) {
        setError(`初始化失败: ${e}`);
      }
    })();
  }, []);

  // 订阅后端事件
  useEffect(() => {
    const unStat = onStatus((s) => setStatus(s));
    const unLog = onLog((e) =>
      setLogs((prev) => [...prev.slice(-500), e]),
    );
    const unStartOk = onStartOk(() => {
      setBusy(null);
      setError(null);
    });
    const unStartFail = onStartFailed((msg) => {
      setBusy(null);
      setError(`启动失败: ${msg}`);
    });
    const unStopOk = onStopOk(() => {
      setBusy(null);
      setError(null);
    });
    const unStopFail = onStopFailed((msg) => {
      setBusy(null);
      setError(`关闭失败: ${msg}`);
    });
    return () => {
      void unStat.then((f) => f());
      void unLog.then((f) => f());
      void unStartOk.then((f) => f());
      void unStartFail.then((f) => f());
      void unStopOk.then((f) => f());
      void unStopFail.then((f) => f());
    };
  }, []);

  const onStart = useCallback(async () => {
    setBusy("start");
    setError(null);
    try {
      await api.startAll();
    } catch (e) {
      setBusy(null);
      setError(String(e));
    }
  }, []);

  const onStop = useCallback(async () => {
    setBusy("stop");
    setError(null);
    try {
      await api.stopBot();
    } catch (e) {
      setBusy(null);
      setError(String(e));
    }
  }, []);

  const overallRunning =
    status.redis.running && status.yunzai.running && status.napcat.running;

  return (
    <div className="h-full flex flex-col bg-transparent text-zinc-100">
      {/* Header */}
      <header className="glass border-b border-white/5 px-6 py-3 flex items-center justify-between drag-region">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-accent to-accent-dim flex items-center justify-center font-bold">
            🧽
          </div>
          <div>
            <h1 className="text-lg font-semibold leading-none">海绵酱控制台</h1>
            <p className="text-xs text-zinc-400 mt-1">
              TRSS Yunzai · NapCat · Redis
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <button
            className={`btn-ghost px-3 py-1.5 rounded-lg text-sm flex items-center gap-1.5 ${
              activeTab === "overview" ? "bg-white/10" : ""
            }`}
            onClick={() => setActiveTab("overview")}
          >
            <Activity size={14} /> 概览
          </button>
          <button
            className={`btn-ghost px-3 py-1.5 rounded-lg text-sm flex items-center gap-1.5 ${
              activeTab === "webview" ? "bg-white/10" : ""
            }`}
            onClick={() => setActiveTab("webview")}
          >
            <ExternalLink size={14} /> NapCat WebUI
          </button>
          <button
            className="btn-ghost px-3 py-1.5 rounded-lg text-sm flex items-center gap-1.5"
            onClick={() => setShowSettings(true)}
          >
            <Settings size={14} /> 设置
          </button>
        </div>
      </header>

      {/* Body */}
      <main className="flex-1 overflow-hidden">
        {activeTab === "overview" ? (
          <div className="h-full grid grid-rows-[auto_1fr_auto] gap-4 p-6">
            {/* Status grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <StatusCard
                name="Redis"
                desc="127.0.0.1:6379"
                status={status.redis}
                port={6379}
                color="amber"
              />
              <StatusCard
                name="Yunzai"
                desc="node app.js · :2536"
                status={status.yunzai}
                port={2536}
                color="violet"
              />
              <StatusCard
                name="NapCat"
                desc="WebUI :6099"
                status={status.napcat}
                port={6099}
                color="fuchsia"
              />
              <StatusCard
                name="QQ"
                desc="OneBotv11"
                status={status.qq}
                color="cyan"
              />
            </div>

            {/* Logs */}
            <div className="min-h-0">
              <LogPanel logs={logs} />
            </div>

            {/* Footer: actions + error */}
            <div className="flex flex-col gap-3">
              {error && (
                <div className="glass border border-bad/40 rounded-lg px-4 py-2 text-sm text-bad flex items-start gap-2">
                  <X
                    size={16}
                    className="cursor-pointer mt-0.5 shrink-0"
                    onClick={() => setError(null)}
                  />
                  <span className="font-mono">{error}</span>
                </div>
              )}
              <div className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-2 text-sm text-zinc-400">
                  <ComponentStatusLine
                    label="Redis"
                    status={status.redis}
                  />
                  <span className="text-zinc-700">→</span>
                  <ComponentStatusLine
                    label="Yunzai"
                    status={status.yunzai}
                  />
                  <span className="text-zinc-700">→</span>
                  <ComponentStatusLine
                    label="NapCat"
                    status={status.napcat}
                  />
                  {status.webui_connected && (
                    <span className="ml-3 text-good">
                      ● WebUI 就绪 (扫码登录:{" "}
                      <a
                        className="underline cursor-pointer"
                        onClick={() => void api.openNapcatWebui()}
                      >
                        localhost:6099
                      </a>
                      )
                    </span>
                  )}
                </div>
                <div className="flex gap-2">
                  <button
                    className="btn-ghost px-4 py-2 rounded-lg text-sm flex items-center gap-1.5"
                    onClick={() => {
                      if (config) void api.revealInExplorer(config.paths.yunzai_dir);
                    }}
                    disabled={!config}
                  >
                    <FolderOpen size={14} /> 打开 Yunzai
                  </button>
                  <button
                    className="btn-primary px-5 py-2 rounded-lg text-sm font-medium flex items-center gap-1.5"
                    onClick={onStart}
                    disabled={busy !== null || overallRunning}
                  >
                    <Power size={14} />
                    {busy === "start" ? "启动中..." : "一键启动"}
                  </button>
                  <button
                    className="btn-danger px-5 py-2 rounded-lg text-sm font-medium flex items-center gap-1.5"
                    onClick={onStop}
                    disabled={busy !== null || !overallRunning}
                  >
                    <PowerOff size={14} />
                    {busy === "stop" ? "关闭中..." : "一键关闭"}
                  </button>
                </div>
              </div>
            </div>
          </div>
        ) : (
          <NapcatWebview />
        )}
      </main>

      {/* Settings modal */}
      <AnimatePresence>
        {showSettings && config && (
          <SettingsModal
            config={config}
            onClose={() => setShowSettings(false)}
            onSaved={(c) => {
              setConfig(c);
              setShowSettings(false);
            }}
          />
        )}
      </AnimatePresence>
    </div>
  );
}

function ComponentStatusLine({
  label,
  status,
}: {
  label: string;
  status: ComponentStatus;
}) {
  return (
    <span
      className={
        status.running
          ? "text-good"
          : "text-zinc-500"
      }
    >
      ● {label}
      {status.pid && (
        <span className="text-zinc-600 font-mono ml-1">
          ({status.pid})
        </span>
      )}
    </span>
  );
}
