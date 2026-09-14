// 设置面板: 编辑路径 + 启动选项
import { useState } from "react";
import { motion } from "framer-motion";
import { Save, X } from "lucide-react";
import { api, type AppConfig, type AppPaths, type AppSettings } from "../lib/tauri";

interface Props {
  config: AppConfig;
  onClose: () => void;
  onSaved: (c: AppConfig) => void;
}

export default function SettingsModal({ config, onClose, onSaved }: Props) {
  const [paths, setPaths] = useState<AppPaths>(config.paths);
  const [settings, setSettings] = useState<AppSettings>(config.settings);
  const [saving, setSaving] = useState(false);

  async function save() {
    setSaving(true);
    try {
      await api.updatePaths(paths, settings);
      if (settings.autostart) {
        await api.setAutostart(true);
      } else {
        await api.setAutostart(false);
      }
      onSaved({ paths, settings });
    } finally {
      setSaving(false);
    }
  }

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-6"
      onClick={onClose}
    >
      <motion.div
        initial={{ scale: 0.95, y: 20 }}
        animate={{ scale: 1, y: 0 }}
        exit={{ scale: 0.95, y: 20 }}
        className="glass rounded-2xl max-w-2xl w-full max-h-[88vh] overflow-y-auto p-6"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between mb-5">
          <h2 className="text-lg font-semibold">设置</h2>
          <button className="btn-ghost p-1.5 rounded-lg" onClick={onClose}>
            <X size={16} />
          </button>
        </div>

        {/* Paths */}
        <section className="mb-6">
          <h3 className="text-sm font-medium text-zinc-300 mb-3">路径配置</h3>
          <div className="space-y-3">
            <PathField
              label="Redis 可执行"
              value={paths.redis_exe}
              onChange={(v) => setPaths({ ...paths, redis_exe: v })}
            />
            <PathField
              label="Redis 配置"
              value={paths.redis_conf}
              onChange={(v) => setPaths({ ...paths, redis_conf: v })}
            />
            <PathField
              label="Yunzai 目录"
              value={paths.yunzai_dir}
              onChange={(v) => setPaths({ ...paths, yunzai_dir: v })}
            />
            <PathField
              label="Yunzai node.exe (nvm 路径)"
              value={paths.yunzai_node}
              onChange={(v) => setPaths({ ...paths, yunzai_node: v })}
            />
            <PathField
              label="NapCat 目录"
              value={paths.napcat_dir}
              onChange={(v) => setPaths({ ...paths, napcat_dir: v })}
            />
            <PathField
              label="Yunzai 日志目录"
              value={paths.yunzai_log_dir}
              onChange={(v) => setPaths({ ...paths, yunzai_log_dir: v })}
            />
            <PathField
              label="NapCat 日志目录"
              value={paths.napcat_log_dir}
              onChange={(v) => setPaths({ ...paths, napcat_log_dir: v })}
            />
          </div>
        </section>

        {/* Settings */}
        <section className="mb-6">
          <h3 className="text-sm font-medium text-zinc-300 mb-3">行为</h3>
          <div className="space-y-3">
            <CheckboxField
              label="开机自启"
              checked={settings.autostart}
              onChange={(v) => setSettings({ ...settings, autostart: v })}
            />
            <CheckboxField
              label="启动 App 后自动拉起所有服务"
              checked={settings.auto_start_on_launch}
              onChange={(v) =>
                setSettings({ ...settings, auto_start_on_launch: v })
              }
            />
            <div>
              <label className="text-sm text-zinc-300 block mb-1">
                关窗行为
              </label>
              <select
                className="w-full bg-bg-2 border border-white/10 rounded-lg px-3 py-2 text-sm"
                value={settings.close_action}
                onChange={(e) =>
                  setSettings({ ...settings, close_action: e.target.value })
                }
              >
                <option value="minimize_to_tray">最小化到托盘 (推荐)</option>
                <option value="exit">直接退出</option>
              </select>
            </div>
          </div>
        </section>

        <div className="flex justify-end gap-2">
          <button className="btn-ghost px-4 py-2 rounded-lg text-sm" onClick={onClose}>
            取消
          </button>
          <button
            className="btn-primary px-5 py-2 rounded-lg text-sm font-medium flex items-center gap-1.5"
            onClick={save}
            disabled={saving}
          >
            <Save size={14} /> {saving ? "保存中..." : "保存"}
          </button>
        </div>
      </motion.div>
    </motion.div>
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
      <label className="text-xs text-zinc-400 block mb-1">{label}</label>
      <input
        type="text"
        className="w-full bg-bg-2 border border-white/10 rounded-lg px-3 py-2 text-sm font-mono focus:outline-none focus:border-accent/50"
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
    </div>
  );
}

function CheckboxField({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <label className="flex items-center gap-2 cursor-pointer text-sm text-zinc-200">
      <input
        type="checkbox"
        className="accent-accent w-4 h-4"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
      />
      {label}
    </label>
  );
}
