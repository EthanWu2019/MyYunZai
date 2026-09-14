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
  RotateCw,
  Puzzle,
  Download,
} from "lucide-react";
import {
  api,
  type AppConfig,
  type StatusSnapshot,
  type ComponentStatus,
  type SetupVerification,
  type QqRecommendation,
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
import FirstRunWizard from "./components/FirstRunWizard";
import PluginManager from "./components/PluginManager";

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
  const [busy, setBusy] = useState<"start" | "stop" | "restart" | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [activeTab, setActiveTab] = useState<"overview" | "plugins" | "webview">(
    "overview",
  );
  const [needsSetup, setNeedsSetup] = useState<boolean | null>(null);
  const [setupStatus, setSetupStatus] = useState<SetupVerification | null>(null);
  const [qqRec, setQqRec] = useState<QqRecommendation | null>(null);

  // 初始加载
  useEffect(() => {
    void (async () => {
      try {
        const cfg = await api.getConfig();
        setConfig(cfg);
        const s = await api.getStatus();
        setStatus(s);
        // 检测是否需要 First-Run Wizard
        const yunzaiAppJs = cfg.paths.yunzai_dir + "\\app.js";
        const needs =
          !yunzaiAppJs || yunzaiAppJs.includes("<USERNAME>");
        setNeedsSetup(needs);

        // 加载 setup status + QQ 推荐 (即使 setup 完成也显示)
        const ss = await api.checkSetupStatus(cfg.paths.yunzai_dir.replace(/\\Yunzai-Bot$/, ""));
        setSetupStatus(ss);
        const qq = await api.getQqRecommendation();
        setQqRec(qq);
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

  const onRestart = useCallback(async () => {
    setBusy("restart");
    setError(null);
    try {
      await api.stopBot();
      // 等待 3 秒再启动
      setTimeout(async () => {
        try {
          await api.startAll();
        } catch (e) {
          setBusy(null);
          setError(String(e));
        }
      }, 3000);
    } catch (e) {
      setBusy(null);
      setError(String(e));
    }
  }, []);

  const overallRunning =
    status.redis.running && status.yunzai.running && status.napcat.running;

  // install_root (基础目录,不含 Yunzai-Bot 子目录)
  const installRoot = config
    ? config.paths.yunzai_dir.replace(/\\Yunzai-Bot$/, "").replace(/\\Yunzai-Bot\\$/, "")
    : "";

  return (
    <div className="h-full flex flex-col bg-transparent text-zinc-100">
      {/* First-Run Wizard 优先显示 */}
      {needsSetup && config && (
        <FirstRunWizard
          paths={config.paths}
          onComplete={() => {
            setNeedsSetup(false);
            // 重新加载 config + status
            void api.getConfig().then(setConfig);
            void api.getStatus().then(setStatus);
            void api
              .checkSetupStatus(installRoot)
              .then(setSetupStatus)
              .catch(() => {});
          }}
        />
      )}

      {/* Header */}
      <header className="glass border-b border-white/5 px-6 py-3 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-accent to-accent-dim flex items-center justify-center font-bold">
            🧽
          </div>
          <div>
            <h1 className="text-lg font-semibold leading-none">海绵酱控制台</h1>
            <p className="text-xs text-zinc-400 mt-1">TRSS Yunzai · NapCat · Redis</p>
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
              activeTab === "plugins" ? "bg-white/10" : ""
            }`}
            onClick={() => setActiveTab("plugins")}
          >
            <Puzzle size={14} /> 插件
          </button>
          <button
            className={`btn-ghost px-3 py-1.5 rounded-lg text-sm flex items-center gap-1.5 ${
              activeTab === "webview" ? "bg-white/10" : ""
            }`}
            onClick={() => setActiveTab("webview")}
          >
            <ExternalLink size={14} /> WebUI
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
        {activeTab === "overview" && (
          <div className="h-full grid grid-rows-[auto_auto_1fr_auto] gap-4 p-6 overflow-y-auto">
            {/* QQ install banner (if needed) */}
            {!status.qq.running && qqRec && (
              <div className="glass border border-warn/40 rounded-lg p-3 flex items-start gap-3 bg-warn/5">
                <Download size={18} className="text-warn shrink-0 mt-0.5" />
                <div className="flex-1 min-w-0">
                  <div className="font-medium text-sm">
                    需要安装 QQ 才能扫码登录
                  </div>
                  <div className="text-xs text-zinc-400 mt-1">
                    推荐版本: <span className="font-mono">{qqRec.version}</span>
                    <span className="text-zinc-500 ml-2">({qqRec.reason})</span>
                  </div>
                  <div className="mt-2 flex items-center gap-2">
                    <a
                      href={qqRec.download_url}
                      target="_blank"
                      rel="noopener"
                      className="btn-primary px-3 py-1 rounded text-xs inline-flex items-center gap-1"
                    >
                      <Download size={12} /> 下载 QQ
                    </a>
                    <span className="text-xs text-zinc-500">{qqRec.note}</span>
                  </div>
                </div>
              </div>
            )}

            {/* Setup status banner (if incomplete) */}
            {setupStatus && !setupStatus.all_ok && (
              <div className="glass border border-warn/40 rounded-lg p-3 flex items-start gap-3 bg-warn/5">
                <X size={18} className="text-warn shrink-0 mt-0.5" />
                <div className="flex-1 min-w-0">
                  <div className="font-medium text-sm">
                    setup 未完成 ({setupStatus.checks.filter((c) => !c.ok).length} 项异常)
                  </div>
                  <div className="text-xs text-zinc-400 mt-1 space-y-0.5">
                    {setupStatus.checks
                      .filter((c) => !c.ok)
                      .slice(0, 5)
                      .map((c) => (
                        <div key={c.name}>
                          <span className="text-bad">✗</span> {c.name}: {c.detail}
                        </div>
                      ))}
                  </div>
                </div>
              </div>
            )}

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
            <div className="min-h-96">
              <LogPanel logs={logs} />
            </div>

            {/* Footer actions */}
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
                  <ComponentStatusLine label="Redis" status={status.redis} />
                  <span className="text-zinc-700">→</span>
                  <ComponentStatusLine label="Yunzai" status={status.yunzai} />
                  <span className="text-zinc-700">→</span>
                  <ComponentStatusLine label="NapCat" status={status.napcat} />
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
                    <FolderOpen size={14} /> 打开目录
                  </button>
                  <button
                    className="btn-primary px-5 py-2 rounded-lg text-sm font-medium flex items-center gap-1.5"
                    onClick={onStart}
                    disabled={busy !== null || overallRunning}
                  >
                    <Power size={14} />
                    {busy === "start" ? "启动中..." : "启动"}
                  </button>
                  <button
                    className="btn-ghost px-4 py-2 rounded-lg text-sm flex items-center gap-1.5"
                    onClick={onRestart}
                    disabled={busy !== null || !overallRunning}
                  >
                    <RotateCw size={14} />
                    {busy === "restart" ? "重启中..." : "重启"}
                  </button>
                  <button
                    className="btn-danger px-5 py-2 rounded-lg text-sm font-medium flex items-center gap-1.5"
                    onClick={onStop}
                    disabled={busy !== null || !overallRunning}
                  >
                    <PowerOff size={14} />
                    {busy === "stop" ? "关闭中..." : "关闭"}
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === "plugins" && config && (
          <PluginManager
            installRoot={installRoot}
            onRefresh={() => {
              void api.getStatus().then(setStatus);
              void api
                .checkSetupStatus(installRoot)
                .then(setSetupStatus)
                .catch(() => {});
            }}
          />
        )}

        {activeTab === "webview" && <NapcatWebview />}
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
