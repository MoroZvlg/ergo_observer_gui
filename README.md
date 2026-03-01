# Funding Rates GUI

A desktop/web application for tracking cryptocurrency funding rate arbitrage opportunities. Built with Rust + egui for native performance and cross-platform deployment.

See [VISION.md](VISION.md) for product vision and roadmap.

## Prerequisites

- **Rust nightly** (install from https://rustup.rs/)
- **WASM target**: `rustup target add wasm32-unknown-unknown`
- **wasm-bindgen-cli**: `cargo install wasm-bindgen-cli`
- **Python 3** (for local web server)

## Development Commands

```bash
# Format code
make fmt

# Run linter
make lint

# Run formatter + linter + tests
make check

# Run tests
make test

# Run native app
make run

# Run in dev mode (with optimizations)
make run-dev

# Build for production (native)
make build

# Build for WebAssembly
make build-wasm

# Build and serve WASM locally (http://localhost:8080)
make run-wasm

# Watch for changes and auto-rebuild
make watch

# Clean build artifacts
make clean
```

## Project Structure

Uses **modern Rust 2018+ module system** (no `mod.rs` files).

```
gui/
├── Cargo.toml              # Rust dependencies and config
├── Makefile                # Development commands
├── rustfmt.toml            # Code formatter config
├── clippy.toml             # Linter config
├── index.html              # WASM entry point
├── VISION.md               # Product vision and roadmap
├── README.md               # This file
│
└── src/
    ├── main.rs             # Platform entry point (native vs WASM)
    ├── app.rs              # Root App struct, main update loop
    │
    ├── widgets.rs          # Widget trait + re-exports
    └── widgets/
        ├── rates_table.rs  # Main funding rates table
        ├── chart.rs        # Historical chart widget
        ├── opportunities.rs # Spread & fast opportunities
        └── settings.rs     # Settings widget
    │
    ├── ui.rs               # UI components re-exports
    └── ui/
        ├── header.rs       # Top bar with widget selector
        ├── footer.rs       # Tech stats (FPS, connection, latency)
        └── layout.rs       # Tiling layout manager (BSP tree)
    │
    ├── api.rs              # API module re-exports
    └── api/
        ├── client.rs       # HTTP client wrapper
        └── types.rs        # API data models (matches OpenAPI)
    │
    ├── state.rs            # State module re-exports
    └── state/
        ├── app_state.rs    # Global app state
        ├── theme.rs        # Theme colors & customization
        └── persistence.rs  # Save/load layout & settings
    │
    ├── utils.rs            # Utilities re-exports
    └── utils/
        ├── formatting.rs   # Number formatting, percentages
        └── time.rs         # Timestamp handling
```

## Architecture Overview

### Widget System

All widgets implement a common trait:

```rust
pub trait Widget {
    fn title(&self) -> &str;
    fn update(&mut self, ui: &mut egui::Ui, state: &mut AppState);
    fn on_close(&mut self);
}
```

Widgets are managed by a **tiling layout manager** (binary space partitioning):
- No floating windows
- Widgets split available space horizontally or vertically
- Resizable dividers between splits
- Pure Rust state management

### Styling

**egui uses immediate-mode rendering** - no separate CSS files.

Styling happens in two places:

1. **Global Theme** (`state/theme.rs`):
   - Colors, fonts, spacing applied to entire app
   - Dark/light themes
   - Customizable color schemes (Phase 2)

2. **Component-Level** (inline with widget code):
   - Widget-specific layout constants
   - Conditional styling based on data (e.g., red for negative values)
   - Uses global theme colors via `AppState`

Example:
```rust
// state/theme.rs - Global colors
pub struct Theme {
    pub bg_primary: Color32,
    pub text_primary: Color32,
    pub positive: Color32,      // Green for positive rates
    pub negative: Color32,      // Red for negative rates
}

// widgets/rates_table.rs - Component usage
impl Widget for RatesTableWidget {
    fn update(&mut self, ui: &mut egui::Ui, state: &AppState) {
        let color = if value > 0.0 {
            state.theme.error
        } else {
            state.theme.success
        };
        ui.colored_label(color, format!("{:.2}%", value));
    }
}
```

### Data Flow

```
API (HTTP) → ApiClient → AppState → Widgets → UI Rendering
                            ↓
                        Theme (colors, fonts)
```

- **AppState**: Single source of truth for all app data
- **ApiClient**: Async HTTP client (native: reqwest, WASM: fetch API)
- **Widgets**: Stateful components that read/modify AppState
- **Theme**: Global styling injected into egui context

### Platform Support

- **Native**: Single binary (macOS/Linux/Windows) via eframe
- **WASM**: Browser-based via WebAssembly + wasm-bindgen
- Same Rust code compiles to both targets

## API Backend

The app connects to a Go backend (see `../funding/README.md`):

**Endpoints**:
- `GET /api/v1/funding_rates` - Current rates for all instruments
- `GET /api/v1/funding_rates/history/{instrument}` - 7-day history
- `GET /api/v1/funding_rates/top_spread_opportunities` - Top 20 spreads
- `GET /api/v1/funding_rates/top_fast_opportunities` - Fast payback times

See `../funding/api/openapi.yaml` for full API spec.

## Development Phases

### Phase 1: MVP (In Progress)
- [x] Dev environment setup
- [ ] Basic app skeleton (header, footer, main area)
- [ ] Single widget: Rates Table
- [ ] HTTP client for `/api/v1/funding_rates`
- [ ] Table rendering with sorting
- [ ] Dark theme only
- [ ] Footer tech stats (FPS, connection)

### Phase 2: Tiling System
- [ ] Binary space partitioning widget layout
- [ ] Add/remove widgets dynamically
- [ ] Draggable dividers for resizing
- [ ] Widget selector in header

### Phase 3: More Widgets
- [ ] Historical chart widget
- [ ] Top spread opportunities widget
- [ ] Fast opportunities widget

### Phase 4: Polish
- [ ] Client-side filtering
- [ ] Layout persistence
- [ ] Settings widget
- [ ] Light theme
- [ ] Keyboard shortcuts

### Phase 5: Advanced Features
- [ ] WebSocket real-time updates
- [ ] Full theme customization
- [ ] Widget state persistence
- [ ] Export data (CSV, JSON)

## Design Principles

- **Data-dense**: Information-first design, minimal chrome
- **Keyboard-friendly**: Power users should never need a mouse
- **Performant**: 60fps rendering, instant filtering/sorting
- **Hackable**: Users customize everything (themes, layouts, data sources)

## License

TBD
