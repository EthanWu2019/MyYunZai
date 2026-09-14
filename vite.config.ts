import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Tauri dev workflow + 避免 vite watch 撞 cargo 锁住的 dll
export default defineConfig({
  plugins: [react()],

  // Tauri 期望一个固定端口；启动 dev server 失败要明确报错而不是挑下一个端口
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: false,
    hmr: { protocol: "ws", host: "localhost", port: 1421 },
    watch: {
      // 关键：cargo / tsc 编译期间会锁这些目录，vite watch 触发会 EBUSY
      ignored: [
        "**/src-tauri/**",
        "**/src-tauri/target/**",
        "**/node_modules/**",
        "**/dist/**",
      ],
    },
  },

  // 兼容 Tauri 在 Windows 下的环境变量
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    // Tauri 在 Windows 上对 chunk 大小比较敏感，关闭 sourcemap 减小体积
    target: "chrome105",
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
});
