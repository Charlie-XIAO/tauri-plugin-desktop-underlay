import { defineConfig } from "tsdown";

export default defineConfig({
  entry: "guest-js/index.ts",
  format: ["esm", "cjs"],
  outDir: "dist-js",
  dts: true,
});
