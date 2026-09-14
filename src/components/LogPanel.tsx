// 日志面板
import { useEffect, useRef } from "react";
import { motion } from "framer-motion";
import { ScrollText } from "lucide-react";
import type { LogEvent } from "../lib/events";

interface Props {
  logs: LogEvent[];
}

const SOURCE_COLORS: Record<string, string> = {
  yunzai: "text-violet-300",
  napcat: "text-fuchsia-300",
};

export default function LogPanel({ logs }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);

  // 自动滚到底
  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [logs]);

  return (
    <div className="glass rounded-xl h-full flex flex-col">
      <div className="px-4 py-2.5 flex items-center justify-between border-b border-white/5">
        <div className="flex items-center gap-2">
          <ScrollText size={14} className="text-zinc-400" />
          <span className="text-sm font-medium">实时日志</span>
          <span className="text-xs text-zinc-500 font-mono">
            ({logs.length} 条)
          </span>
        </div>
      </div>
      <div
        ref={containerRef}
        className="flex-1 overflow-y-auto p-3 font-mono text-[11.5px] leading-relaxed"
      >
        {logs.length === 0 ? (
          <div className="text-zinc-600 italic px-3 py-6 text-center">
            等待后端推送日志...
          </div>
        ) : (
          logs.map((log, i) => (
            <motion.div
              key={`${log.source}-${log.file}-${i}-${log.text.length}`}
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex gap-2 hover:bg-white/[0.02] px-2 py-0.5 rounded"
            >
              <span
                className={`shrink-0 ${SOURCE_COLORS[log.source] || "text-zinc-500"}`}
              >
                [{log.source}:{log.file}]
              </span>
              <pre className="whitespace-pre-wrap break-all text-zinc-300 m-0">
                {log.text}
              </pre>
            </motion.div>
          ))
        )}
      </div>
    </div>
  );
}
