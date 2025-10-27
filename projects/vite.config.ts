import { defineConfig } from "vite";
import { fresh } from "@fresh/plugin-vite";
import { cloudflare } from "npm:@cloudflare/vite-plugin";

export default defineConfig({
  plugins: [fresh(), cloudflare()],
});
