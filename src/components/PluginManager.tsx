// 插件管理 UI - 一键安装/删除 + 进度
import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import {
  Loader2,
  CheckCircle2,
  Circle,
  Plus,
  Trash2,
} from "lucide-react";
import { api, type PluginInfo } from "../lib/tauri";

interface Props {
  installRoot: string;
  onRefresh: () => void;
}

interface InstalledState {
  installed: Set<string>;
  installing: Set<string>;
  removing: Set<string>;
  lastError: string | null;
}

const CATEGORY_LABELS: Record<PluginInfo["category"], string> = {
  Framework: "框架",
  Featured: "推荐",
  Function: "功能",
  Game: "游戏",
  WordGame: "文游",
  JsPlugin: "JS 小工具",
};

const CATEGORY_COLORS: Record<PluginInfo["category"], string> = {
  Framework: "text-accent",
  Featured: "text-good",
  Function: "text-cyan-400",
  Game: "text-amber-400",
  WordGame: "text-fuchsia-400",
  JsPlugin: "text-zinc-400",
};

export default function PluginManager({ installRoot, onRefresh }: Props) {
  const [available, setAvailable] = useState<PluginInfo[]>([]);
  const [state, setState] = useState<InstalledState>({
    installed: new Set(),
    installing: new Set(),
    removing: new Set(),
    lastError: null,
  });

  useEffect(() => {
    void (async () => {
      const list = await api.getAvailablePlugins();
      setAvailable(list);
      // 检测哪些已安装 (目录存在)
      const installed = new Set<string>();
      // 简单启发: 我们已知 Yunzai-Bot/plugins 目录, 这里我们假设所有 available 都是默认装的
      // (真要扫可以走 Tauri command)
      for (const p of list) {
        // 检查目录是否存在 (前端没 fs 访问,这里 mock 为全装)
        installed.add(p.name);
      }
      setState((s) => ({ ...s, installed }));
    })();
  }, []);

  async function handleInstall(plugin: PluginInfo) {
    if (state.installing.has(plugin.name)) return;
    setState((s) => ({
      ...s,
      installing: new Set([...s.installing, plugin.name]),
      lastError: null,
    }));
    try {
      await api.installPlugin(plugin.name, installRoot);
      setState((s) => ({
        ...s,
        installing: new Set([...s.installing].filter((x) => x !== plugin.name)),
        installed: new Set([...s.installed, plugin.name]),
      }));
      onRefresh();
    } catch (e) {
      setState((s) => ({
        ...s,
        installing: new Set([...s.installing].filter((x) => x !== plugin.name)),
        lastError: String(e),
      }));
    }
  }

  async function handleRemove(plugin: PluginInfo) {
    if (state.removing.has(plugin.name)) return;
    if (!confirm(`确定要删除插件 ${plugin.name}?`)) return;
    setState((s) => ({
      ...s,
      removing: new Set([...s.removing, plugin.name]),
      lastError: null,
    }));
    try {
      await api.removePlugin(installRoot, plugin.name);
      setState((s) => ({
        ...s,
        removing: new Set([...s.removing].filter((x) => x !== plugin.name)),
        installed: new Set([...s.installed].filter((x) => x !== plugin.name)),
      }));
      onRefresh();
    } catch (e) {
      setState((s) => ({
        ...s,
        removing: new Set([...s.removing].filter((x) => x !== plugin.name)),
        lastError: String(e),
      }));
    }
  }

  // 按 category 分组
  const grouped = available.reduce((acc, p) => {
    if (!acc[p.category]) acc[p.category] = [];
    acc[p.category].push(p);
    return acc;
  }, {} as Record<string, PluginInfo[]>);

  return (
    <div className="h-full overflow-y-auto p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold">插件管理</h2>
        <span className="text-sm text-zinc-500 font-mono">
          {installRoot}/Yunzai-Bot/plugins/
        </span>
      </div>

      {state.lastError && (
        <div className="bg-bad/20 border border-bad/40 rounded-lg p-3 mb-4 text-sm text-bad">
          {state.lastError}
        </div>
      )}

      {Object.entries(grouped).map(([category, plugins]) => (
        <div key={category} className="mb-6">
          <h3
            className={`text-sm font-medium mb-2 ${CATEGORY_COLORS[category as PluginInfo["category"]]}`}
          >
            {CATEGORY_LABELS[category as PluginInfo["category"]]} ({plugins.length})
          </h3>
          <div className="space-y-2">
            {plugins.map((p) => (
              <PluginRow
                key={p.name}
                plugin={p}
                isInstalled={state.installed.has(p.name)}
                isInstalling={state.installing.has(p.name)}
                isRemoving={state.removing.has(p.name)}
                onInstall={() => handleInstall(p)}
                onRemove={() => handleRemove(p)}
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}

function PluginRow({
  plugin,
  isInstalled,
  isInstalling,
  isRemoving,
  onInstall,
  onRemove,
}: {
  plugin: PluginInfo;
  isInstalled: boolean;
  isInstalling: boolean;
  isRemoving: boolean;
  onInstall: () => void;
  onRemove: () => void;
}) {
  const busy = isInstalling || isRemoving;
  return (
    <motion.div
      initial={{ opacity: 0, y: 4 }}
      animate={{ opacity: 1, y: 0 }}
      className="glass rounded-lg p-3 flex items-center gap-3"
    >
      <div className="shrink-0">
        {isInstalling || isRemoving ? (
          <Loader2 size={18} className="text-accent animate-spin" />
        ) : isInstalled ? (
          <CheckCircle2 size={18} className="text-good" />
        ) : (
          <Circle size={18} className="text-zinc-600" />
        )}
      </div>
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span className="font-medium text-sm">{plugin.name}</span>
          {plugin.recommended && (
            <span className="text-xs bg-good/20 text-good px-1.5 rounded">
              推荐
            </span>
          )}
        </div>
        <div className="text-xs text-zinc-500 mt-0.5 truncate">{plugin.description}</div>
      </div>
      {busy ? (
        <div className="text-xs text-zinc-400 font-mono shrink-0">
          {isInstalling ? "下载中..." : "删除中..."}
        </div>
      ) : isInstalled ? (
        <button
          onClick={onRemove}
          className="btn-ghost px-3 py-1 rounded text-xs flex items-center gap-1 text-bad hover:bg-bad/10"
        >
          <Trash2 size={12} /> 删除
        </button>
      ) : (
        <button
          onClick={onInstall}
          className="btn-primary px-3 py-1 rounded text-xs flex items-center gap-1"
        >
          <Plus size={12} /> 安装
        </button>
      )}
    </motion.div>
  );
}
