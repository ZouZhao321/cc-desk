# CC Desk

CC Desk is a Tauri 2 desktop application for managing Claude Code model configurations.

[中文文档](docs/README.zh-CN.md)

## Tech Stack

| Layer       | Technology                                            |
| ----------- | ----------------------------------------------------- |
| Frontend    | Vue 3 + TypeScript + Naive UI + UnoCSS                |
| Backend     | Rust (Tauri 2)                                        |
| Build       | Vite + Cargo                                          |
| Package Mgr | pnpm                                                  |
| Code Style  | ESLint + Prettier (TS/Vue), cargo fmt + clippy (Rust) |
| Git Hooks   | Husky + lint-staged                                   |

## Getting Started

### Prerequisites

- Node.js >= 18
- pnpm
- Rust (via [rustup](https://rustup.rs/))
- Tauri 2 system dependencies (see [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/))

### Install Dependencies

```bash
pnpm install
```

### Development

```bash
# Start frontend dev server (port 1420)
pnpm dev

# Start Tauri dev mode (frontend + Rust backend)
pnpm tauri dev
```

### Build

```bash
# Frontend build (type check + bundle)
pnpm build

# Generate desktop installer
pnpm tauri build
```

### Project Structure

```
src/
  components/    # Vue components
  composables/   # Composables (useSettings, usePresets)
  types/         # TypeScript type definitions
  utils/         # Utility functions

src-tauri/src/   # Rust backend (Tauri commands, app entry)
```

## License

[AGPL-3.0](LICENSE)
