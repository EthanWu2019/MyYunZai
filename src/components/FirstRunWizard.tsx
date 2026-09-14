// First Run Wizard - 首次启动时引导用户 setup
import { useEffect, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  CheckCircle2,
  Circle,
  Loader2,
  XCircle,
  AlertCircle,
  Settings as SettingsIcon,
  Plug,
} from "lucide-react";
import {
  api,
  type SetupProgress,
  type SetupStep,
  type PluginInfo,
  type AppPaths,
} from "../lib/tauri";
import {
  onSetupProgress,
  onSetupComplete,
  onSetupFailed,
} from "../lib/events";

interface Props {
  paths: AppPaths;
  onComplete: () => void;
}

const STEPS = ["welcome", "paths", "plugins", "installing", "done"] as const;
type WizardStep = (typeof STEPS)[number];

export default function FirstRunWizard({ paths: initialPaths, onComplete }: Props) {
  const [step, setStep] = useState<WizardStep>("welcome");
  const [paths, setPaths] = useState<AppPaths>(initialPaths);
  const [plugins, setPlugins] = useState<PluginInfo[]>([]);
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [progress, setProgress] = useState<SetupProgress | null>(null);
  const [error, setError] = useState<string | null>(null);

  // 加载可用插件
  useEffect(() => {
    void (async () => {
      try {
        const list = await api.getAvailablePlugins();
        setPlugins(list);
        // 默认全选
        setSelected(new Set(list.map((p) => p.name)));
      } catch (e) {
        setError(`加载插件列表失败: ${e}`);
      }
    })();
  }, []);

  // 订阅 setup 进度
  useEffect(() => {
    const u1 = onSetupProgress((p) => setProgress(p));
    const u2 = onSetupComplete(() => {
      setStep("done");
      setTimeout(() => onComplete(), 2000);
    });
    const u3 = onSetupFailed((msg) => setError(msg));
    return () => {
      void Promise.all([u1, u2, u3]).then((arr) => arr.forEach((f) => f()));
    };
  }, [onComplete]);

  const totalSelectedSize = plugins
    .filter((p) => selected.has(p.name))
    .reduce((acc, p) => {
      // 估算: 每个插件 ~5 MB + node_modules ~50 MB
      return acc + (p.category === "Framework" ? 100 : 5) + 50;
    }, 0);

  return (
    <div className="fixed inset-0 bg-bg-0/95 backdrop-blur-sm flex items-center justify-center z-50 p-6">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="glass rounded-2xl max-w-4xl w-full max-h-[90vh] overflow-y-auto p-8"
      >
        {/* Step indicator */}
        <div className="flex items-center gap-2 mb-6">
          {STEPS.map((s, i) => (
            <div key={s} className="flex items-center gap-2">
              <div
                className={`w-8 h-8 rounded-full flex items-center justify-center text-sm ${
                  STEPS.indexOf(step) >= i
                    ? "bg-accent text-white"
                    : "bg-bg-3 text-zinc-500"
                }`}
              >
                {i + 1}
              </div>
              {i < STEPS.length - 1 && (
                <div
                  className={`w-12 h-0.5 ${
                    STEPS.indexOf(step) > i ? "bg-accent" : "bg-bg-3"
                  }`}
                />
              )}
            </div>
          ))}
        </div>

        <AnimatePresence mode="wait">
          {step === "welcome" && (
            <motion.div
              key="welcome"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              <div className="text-center py-8">
                <div className="text-6xl mb-4">🧽</div>
                <h2 className="text-2xl font-semibold mb-3">
                  欢迎使用 YunZaiConsole
                </h2>
                <p className="text-zinc-400 mb-6 max-w-xl mx-auto">
                  这是一个 Yunzai 机器人一键部署器。点击下一步开始安装：
                </p>
                <ul className="text-left max-w-md mx-auto space-y-2 mb-8 text-sm">
                  <li className="flex items-start gap-2">
                    <CheckCircle2 size={16} className="text-good mt-0.5 shrink-0" />
                    <span>自动下载 Yunzai 主程序 + 23 个常用插件</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2 size={16} className="text-good mt-0.5 shrink-0" />
                    <span>自动下载 NapCat 协议端 + QQ 客户端</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2 size={16} className="text-good mt-0.5 shrink-0" />
                    <span>自动安装 Redis + Node.js 依赖</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2 size={16} className="text-good mt-0.5 shrink-0" />
                    <span>约 5-30 分钟,下载 ~1-3 GB</span>
                  </li>
                </ul>
                <button
                  className="btn-primary px-8 py-3 rounded-lg font-medium"
                  onClick={() => setStep("paths")}
                >
                  开始安装 →
                </button>
              </div>
            </motion.div>
          )}

          {step === "paths" && (
            <motion.div
              key="paths"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              <h2 className="text-xl font-semibold mb-2">
                <SettingsIcon className="inline mr-2" size={20} />
                选择安装目录
              </h2>
              <p className="text-zinc-400 text-sm mb-6">
                所有 Yunzai 组件将下载到此目录(默认 C:\YunZaiApp)
              </p>
              <div className="space-y-4">
                <PathField
                  label="安装根目录"
                  value={paths.yunzai_dir.replace(/\\Yunzai-Bot$/, "")}
                  onChange={(v) =>
                    setPaths({
                      ...paths,
                      yunzai_dir: `${v}\\Yunzai-Bot`,
                      yunzai_log_dir: `${v}\\Yunzai-Bot\\logs`,
                    })
                  }
                />
              </div>
              <div className="flex justify-between mt-8">
                <button
                  className="btn-ghost px-4 py-2 rounded-lg"
                  onClick={() => setStep("welcome")}
                >
                  ← 上一步
                </button>
                <button
                  className="btn-primary px-6 py-2 rounded-lg font-medium"
                  onClick={() => setStep("plugins")}
                >
                  下一步 →
                </button>
              </div>
            </motion.div>
          )}

          {step === "plugins" && (
            <motion.div
              key="plugins"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              <h2 className="text-xl font-semibold mb-2">
                <Plug className="inline mr-2" size={20} />
                选择要安装的插件 ({selected.size}/{plugins.length})
              </h2>
              <p className="text-zinc-400 text-sm mb-4">
                默认全选主人游戏本同款 23 个插件。取消勾选可跳过。
              </p>
              <div className="text-xs text-zinc-500 mb-4">
                预计下载: ~{totalSelectedSize} MB
              </div>

              <div className="max-h-96 overflow-y-auto space-y-2">
                {plugins.map((p) => (
                  <PluginRow
                    key={p.name}
                    plugin={p}
                    checked={selected.has(p.name)}
                    onChange={(v) => {
                      const next = new Set(selected);
                      if (v) next.add(p.name);
                      else next.delete(p.name);
                      setSelected(next);
                    }}
                  />
                ))}
              </div>

              <div className="flex justify-between mt-6">
                <button
                  className="btn-ghost px-4 py-2 rounded-lg"
                  onClick={() => setStep("paths")}
                >
                  ← 上一步
                </button>
                <button
                  className="btn-primary px-6 py-2 rounded-lg font-medium"
                  onClick={async () => {
                    setError(null);
                    setStep("installing");
                    try {
                      await api.runSetup(paths, Array.from(selected));
                    } catch (e) {
                      setError(String(e));
                      setStep("plugins");
                    }
                  }}
                >
                  开始安装 →
                </button>
              </div>
            </motion.div>
          )}

          {step === "installing" && (
            <motion.div
              key="installing"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              <h2 className="text-xl font-semibold mb-2">
                <Loader2 className="inline mr-2 animate-spin" size={20} />
                正在安装... ({progress?.current_step ?? 0}/{progress?.total_steps ?? "?"})
              </h2>
              <p className="text-zinc-400 text-sm mb-6">
                请勿关闭 App。这可能需要 5-30 分钟。
              </p>

              {error && (
                <div className="bg-bad/20 border border-bad/40 rounded-lg p-3 mb-4 text-sm text-bad">
                  {error}
                </div>
              )}

              <div className="space-y-2">
                {progress?.steps.map((s) => (
                  <SetupStepRow key={s.id} step={s} />
                ))}
              </div>
            </motion.div>
          )}

          {step === "done" && (
            <motion.div
              key="done"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              <div className="text-center py-8">
                <CheckCircle2 size={64} className="text-good mx-auto mb-4" />
                <h2 className="text-2xl font-semibold mb-3">安装完成!</h2>
                <p className="text-zinc-400 mb-6">
                  Yunzai 部署成功。正在打开主控制台...
                </p>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </motion.div>
    </div>
  );
}

function PathField({
  label,
  value,
  onChange,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
}) {
  return (
    <div>
      <label className="text-sm text-zinc-300 block mb-1">{label}</label>
      <input
        type="text"
        className="w-full bg-bg-2 border border-white/10 rounded-lg px-3 py-2 text-sm font-mono focus:outline-none focus:border-accent/50"
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
    </div>
  );
}

function PluginRow({
  plugin,
  checked,
  onChange,
}: {
  plugin: PluginInfo;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  const categoryColors: Record<string, string> = {
    Framework: "text-accent",
    Featured: "text-good",
    Function: "text-cyan-400",
    Game: "text-amber-400",
    WordGame: "text-fuchsia-400",
    JsPlugin: "text-zinc-400",
  };
  return (
    <label className="flex items-start gap-3 p-3 rounded-lg hover:bg-white/[0.04] cursor-pointer">
      <input
        type="checkbox"
        className="accent-accent w-4 h-4 mt-0.5 shrink-0"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
      />
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span className="font-medium">{plugin.name}</span>
          <span className={`text-xs ${categoryColors[plugin.category]}`}>
            {plugin.category}
          </span>
          {plugin.recommended && (
            <span className="text-xs bg-good/20 text-good px-1.5 rounded">
              推荐
            </span>
          )}
        </div>
        <div className="text-xs text-zinc-500 mt-0.5">{plugin.description}</div>
        <div className="text-xs text-zinc-600 font-mono mt-1 truncate">
          {plugin.repo_url}
        </div>
      </div>
    </label>
  );
}

function SetupStepRow({ step }: { step: SetupStep }) {
  const icons = {
    Pending: <Circle size={16} className="text-zinc-600" />,
    Running: <Loader2 size={16} className="text-accent animate-spin" />,
    Completed: <CheckCircle2 size={16} className="text-good" />,
    Failed: <XCircle size={16} className="text-bad" />,
    Skipped: <AlertCircle size={16} className="text-zinc-500" />,
  };
  return (
    <div className="flex items-start gap-3 p-3 rounded-lg bg-bg-2/50">
      <div className="mt-0.5">{icons[step.status]}</div>
      <div className="flex-1 min-w-0">
        <div className="text-sm font-medium">{step.name}</div>
        {step.message && (
          <div className="text-xs text-zinc-500 font-mono mt-0.5 truncate">
            {step.message}
          </div>
        )}
      </div>
    </div>
  );
}
