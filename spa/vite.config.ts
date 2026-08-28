import { defineConfig } from "vite"
import { devtools } from "@tanstack/devtools-vite"
import { TanStackRouterVite } from "@tanstack/router-plugin/vite"
import viteReact from "@vitejs/plugin-react"
import tailwindcss from "@tailwindcss/vite"

/**
 * The three dependencies every screen needs, each in one chunk. Left to itself
 * the splitter emits dozens of sub-kilobyte files, where the request costs
 * more than the code.
 *
 * Deliberately not a catch-all: a `node_modules` group would sweep the chart
 * library and the date picker — which only two screens want — into a chunk
 * that loads on first paint.
 */
const VENDOR_GROUPS = [
  { name: "react", test: /node_modules[/\\](react|react-dom|scheduler)[/\\]/ },
  { name: "base-ui", test: /node_modules[/\\]@base-ui[/\\]/ },
  { name: "tanstack", test: /node_modules[/\\]@tanstack[/\\]/ },
]

const config = defineConfig({
  resolve: { tsconfigPaths: true },
  plugins: [devtools(), tailwindcss(), TanStackRouterVite(), viteReact()],
  build: {
    rolldownOptions: {
      output: {
        advancedChunks: {
          minSize: 12_000,
          groups: VENDOR_GROUPS.map((group, index) => ({
            ...group,
            priority: VENDOR_GROUPS.length - index,
          })),
        },
      },
    },
  },
})

export default config
