// Yunzai 终端风格日志面板 - 黑色 + 彩色 + 自动滚 + 搜索 + 清空
import { useEffect, useRef, useState } from "react";
import { motion } from "framer-motion";
import { Trash2, Search, ScrollText, Filter } from "lucide-react";
import type { LogEvent } from "../lib/events";

interface Props {
  logs: LogEvent[];
  source?: "yunzai" | "napcat" | "all";
}

const SOURCE_COLORS: Record<string, string> = {
  yunzai: "text-violet-300",
  napcat: "text-fuchsia-300",
};

const LOG_LINE_COLOR_REGEX = [
  { pattern: /\b(ERROR|FATAL|ERRO|Error|Error:)\b/g, className: "text-bad" },
  { pattern: /\b(WARN|Warning|WARNING)\b/g, className: "text-warn" },
  { pattern: /\b(INFO|info)\b/g, className: "text-cyan-300" },
  { pattern: /\b(DEBUG|debug)\b/g, className: "text-zinc-500" },
  { pattern: /\b(SUCCESS|✓|成功|完成)\b/g, className: "text-good" },
];

export default function LogPanel({ logs, source = "all" }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [search, setSearch] = useState("");
  const [sourceFilter, setSourceFilter] = useState<"all" | "yunzai" | "napcat">(
    "all",
  );
  const [autoScroll, setAutoScroll] = useState(true);

  // 过滤
  const filtered = logs.filter((log) => {
    if (sourceFilter !== "all" && log.source !== sourceFilter) return false;
    if (search && !log.text.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  // 自动滚到底 (仅在用户开启自动滚时)
  useEffect(() => {
    if (autoScroll && containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [filtered.length, autoScroll]);

  return (
    <div className="glass rounded-xl h-full flex flex-col bg-black/40">
      {/* Toolbar */}
      <div className="px-3 py-2 flex items-center gap-2 border-b border-white/5">
        <ScrollText size={14} className="text-zinc-400" />
        <span className="text-sm font-medium">终端</span>
        <span className="text-xs text-zinc-500 font-mono">
          {filtered.length}/{logs.length} 行
        </span>

        <div className="flex-1 flex items-center gap-2 ml-4">
          <div className="relative flex-1 max-w-xs">
            <Search
              size={12}
              className="absolute left-2 top-1/2 -translate-y-1/2 text-zinc-500"
            />
            <input
              type="text"
              placeholder="搜索..."
              className="w-full bg-black/30 border border-white/10 rounded pl-7 pr-2 py-1 text-xs font-mono focus:outline-none focus:border-accent/50"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>

          <div className="flex items-center gap-1 bg-black/30 border border-white/10 rounded px-1 py-0.5">
            {(["all", "yunzai", "napcat"] as const).map((s) => (
              <button
                key={s}
                onClick={() => setSourceFilter(s)}
                className={`px-2 py-0.5 text-xs rounded ${
                  sourceFilter === s
                    ? "bg-accent text-white"
                    : "text-zinc-400 hover:text-zinc-200"
                }`}
              >
                {s === "all" ? "全部" : s === "yunzai" ? "云崽" : "NapCat"}
              </button>
            ))}
          </div>

          <label className="flex items-center gap-1 text-xs text-zinc-400 cursor-pointer">
            <input
              type="checkbox"
              checked={autoScroll}
              onChange={(e) => setAutoScroll(e.target.checked)}
              className="accent-accent"
            />
            自动滚
          </label>
        </div>
      </div>

      {/* Log content (terminal style: 黑底 + monospace + 颜色) */}
      <div
        ref={containerRef}
        className="flex-1 overflow-y-auto p-3 font-mono text-[11px] leading-[1.5] bg-black/60"
        style={{ fontFamily: "'JetBrains Mono', 'Cascadia Code', monospace" }}
      >
        {filtered.length === 0 ? (
          <div className="text-zinc-600 italic px-3 py-6 text-center">
            {logs.length === 0
              ? "等待后端推送日志..."
              : "当前过滤条件下无日志"}
          </div>
        ) : (
          filtered.map((log, i) => (
            <motion.div
              key={`${log.source}-${log.file}-${i}`}
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex gap-2 hover:bg-white/[0.02] px-2 py-0.5 rounded"
            >
              <span
                className={`shrink-0 ${SOURCE_COLORS[log.source] || "text-zinc-500"}`}
              >
                [{log.source}:{log.file}]
              </span>
              <pre className="whitespace-pre-wrap break-all text-zinc-200 m-0">
                {highlightLogLine(log.text)}
              </pre>
            </motion.div>
          ))
        )}
      </div>
    </div>
  );
}

function highlightLogLine(text: string): React.ReactNode {
  // 简单关键词高亮 (生产可换更复杂的 ANSI parser)
  let result: (string | JSX.Element)[] = [text];
  for (const { pattern, className } of LOG_LINE_COLOR_REGEX) {
    result = result.flatMap((chunk, idx) => {
      if (typeof chunk !== "string") return [chunk];
      const parts: (string | JSX.Element)[] = [];
      let lastIdx = 0;
      let match;
      const re = new RegExp(pattern.source, "g");
      while ((match = re.exec(chunk)) !== null) {
        if (match.index > lastIdx) {
          parts.push(chunk.substring(lastIdx, match.index));
        }
        parts.push(
          <span key={`${idx}-${match.index}`} className={className}>
            {match[0]}
          </span>,
        );
        lastIdx = match.index + match[0].length;
      }
      if (lastIdx < chunk.length) {
        parts.push(chunk.substring(lastIdx));
      }
      return parts;
    });
  }
  return result;
}
