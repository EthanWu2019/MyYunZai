// 内嵌 NapCat WebUI (用 WebView2 加载 http://localhost:6099/webui/)
import { useEffect, useRef, useState } from "react";
import { ExternalLink, RefreshCw } from "lucide-react";
import { api } from "../lib/tauri";

export default function NapcatWebview() {
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const [available, setAvailable] = useState<boolean | null>(null);
  const [key, setKey] = useState(0);

  useEffect(() => {
    void checkWebui();
  }, []);

  async function checkWebui() {
    // 通过 getStatus 检查 webui_connected (6099 端口 listen)
    try {
      const s = await api.getStatus();
      setAvailable(s.webui_connected);
    } catch {
      setAvailable(false);
    }
  }

  return (
    <div className="h-full flex flex-col p-6">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2 text-sm">
          <span className="text-zinc-400">NapCat WebUI</span>
          {available === false && (
            <span className="text-warn">
              ⚠️ 端口 6099 未监听 · 请先启动 NapCat
            </span>
          )}
          {available === true && (
            <span className="text-good">● 就绪</span>
          )}
        </div>
        <div className="flex gap-2">
          <button
            className="btn-ghost px-3 py-1.5 rounded-lg text-sm flex items-center gap-1.5"
            onClick={() => setKey((k) => k + 1)}
          >
            <RefreshCw size={14} /> 刷新
          </button>
          <button
            className="btn-ghost px-3 py-1.5 rounded-lg text-sm flex items-center gap-1.5"
            onClick={() => void api.openNapcatWebui()}
          >
            <ExternalLink size={14} /> 系统浏览器
          </button>
        </div>
      </div>
      <div className="flex-1 glass rounded-xl overflow-hidden">
        <iframe
          key={key}
          ref={iframeRef}
          src="http://localhost:6099/webui/"
          className="w-full h-full bg-bg-0"
          sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-modals"
        />
      </div>
    </div>
  );
}
