# Multi-Platform UI Client (`dsot_desktop`, `dsot_mobile`, & `dsot_shared_ui`)

The UI layer is split into distinct projects located under `src/app/` to ensure clear separation of concerns, native bundling config per platform, and a shared presentation library:

- **`dsot_desktop`**: Native desktop executable targeting desktop platforms (GTK/WebView via muda/tao).
- **`dsot_mobile`**: Native mobile executable targeting mobile webviews.
- **`dsot_shared_ui`**: Shared presentation library containing views, widgets, and shared assets (fonts, favicon, root stylesheet) used by both platforms.

---

## Crate Layout & Key Components

```
src/app/
├── desktop/           # dsot_desktop (Executable Crate)
│   ├── assets/        # Desktop-specific styling overrides & icons
│   └── src/
│       ├── main.rs    # Desktop application entrypoint & window configuration
│       ├── layout.rs  # Desktop page layout structure with footer/topbar panels
│       ├── routes.rs  # Desktop-specific router mapping
│       └── widgets/   # Desktop-only layout panels (frame, topbar, footer, left/right panels)
│
├── mobile/            # dsot_mobile (Executable Crate)
│   ├── assets/        # Mobile-specific icons
│   └── src/
│       ├── main.rs    # Mobile application entrypoint & initialization
│       ├── layout.rs  # Mobile navigation/header layout
│       └── routes.rs  # Mobile-specific router mapping
│
└── shared_ui/         # dsot_shared_ui (Library Crate)
    ├── assets/        # Shared resources (Satoshi/Tanker fonts, root.css, favicon, logo)
    └── src/
        ├── lib.rs     # Library exports (assets, views, widgets)
        ├── assets.rs  # Shared asset mappings with `asset!` macro
        ├── views/     # Route pages (HomeView, ConfigView, InboxView)
        └── widgets/   # Reusable widgets (InboxAdd, InboxList)
```

---

## Core Initialization Lifecycle

Each platform runs its own entrypoint binary. The application state context is bound at startup using their respective `main.rs`:

```mermaid
graph TD
    A1[dsot_desktop: main.rs] --> E1[Init DsotCoreInitOptions with desktop capabilities]
    A2[dsot_mobile: main.rs] --> E2[Init DsotCoreInitOptions with mobile capabilities]
    
    E1 --> G[Initialize DsotCore]
    E2 --> G
    
    G --> H1[Configure Window & Menu]
    G --> H2[Configure Mobile Webview Options]
    
    H1 --> I1[LaunchBuilder::desktop]
    H2 --> I2[LaunchBuilder::mobile]
    
    I1 --> J[Inject DsotCore into Context]
    I2 --> J
    
    J --> K[Mount root stylesheet and Router]
```

### Context Injection
Upon launching, the client application injects the shared core context (`DsotCore` from `dsot_lib`) using Dioxus context injection (`LaunchBuilder::with_context`). This allows any down-tree widget or view in `dsot_shared_ui` to retrieve the database pool or configuration using:
```rust
let core = use_context::<DsotCore>();
```

---

## Views & Widgets (`dsot_shared_ui`)

### 1. Views (`views/`)
- **`HomeView`**: The dashboard containing library summaries, play queues, and navigation links.
- **`ConfigView`**: Interacts with `dsot_config` to view logs, custom database paths, and active profile information.
- **`InboxView`**: Displays items captured by the user that need matching. Connects to `InboxItemRepository` to list unmatched items.

### 2. Widgets (`widgets/`)
- **`inbox_add`**: A form rendering inputs to capture new files, artists, or notes. Validates inputs and inserts a serialized `InboxItem` into the repository.
- **`inbox_list`**: Queries, lists, and manages the lifecycle of inbox items, allowing actions to trigger matching pipelines or delete items.

---

## Technical Details

- **UI Framework**: Dioxus v0.7.
- **Styling**: Standard Vanilla CSS loaded from `dsot_shared_ui::assets::ROOT_CSS`.
- **Assets**: Bound using compile-time Dioxus assets hooks (`asset!()`).
