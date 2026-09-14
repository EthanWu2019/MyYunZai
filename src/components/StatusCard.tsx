// 单组件状态卡
import { motion } from "framer-motion";
import type { ComponentStatus } from "../lib/tauri";

interface Props {
  name: string;
  desc: string;
  status: ComponentStatus;
  port?: number;
  color?: "amber" | "violet" | "fuchsia" | "cyan";
}

const COLOR_MAP = {
  amber: { on: "bg-amber-400", text: "text-amber-300" },
  violet: { on: "bg-violet-400", text: "text-violet-300" },
  fuchsia: { on: "bg-fuchsia-400", text: "text-fuchsia-300" },
  cyan: { on: "bg-cyan-400", text: "text-cyan-300" },
};

export default function StatusCard({
  name,
  desc,
  status,
  port,
  color = "violet",
}: Props) {
  const palette = COLOR_MAP[color];
  return (
    <motion.div
      layout
      className="glass rounded-xl p-4 flex flex-col gap-2"
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span
            className={`inline-block w-2.5 h-2.5 rounded-full ${
              status.running ? `${palette.on} pulse-dot ${palette.text}` : "bg-zinc-700"
            }`}
          />
          <span className="font-medium">{name}</span>
        </div>
        <span
          className={`text-xs font-mono ${
            status.running ? "text-good" : "text-zinc-500"
          }`}
        >
          {status.running ? "ONLINE" : "OFFLINE"}
        </span>
      </div>
      <div className="text-xs text-zinc-400 font-mono">
        {desc}
        {port && (
          <>
            {" · "}
            <span className={status.running ? palette.text : ""}>:{port}</span>
          </>
        )}
      </div>
      {status.pid && (
        <div className="text-xs text-zinc-500 font-mono">PID {status.pid}</div>
      )}
    </motion.div>
  );
}
