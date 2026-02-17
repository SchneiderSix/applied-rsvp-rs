# Applied RSVP RS

A Rapid Serial Visual Presentation (RSVP) speed reader built in Rust with [iced](https://iced.rs/) — written entirely by hand as a learning project.

RSVP displays one word at a time at a configurable speed, training your brain to read without subvocalization. This app supports multiple file formats, custom fonts, color theming, and persistent configuration.

## Features

- **Multi-format file loading** — supports TXT, CSV, MD, HTML, and PDF via native file dialogs
- **PDF text extraction** — powered by [oxidize-pdf](https://github.com/bzsanti/oxidizePdf) with full CMap/ToUnicode encoding support for special characters
- **Custom font loading** — pick any TTF, OTF, WOFF, or WOFF2 font file from your system
- **Color theming** — customizable background, text, and primary colors with color picker
- **Adjustable reading speed** — arrow keys control words-per-minute in real time
- **Playback controls** — space to pause/resume, automatic word progression with timer subscription
- **Text caching** — processed files are cached locally to avoid re-parsing on repeated opens
- **Fullscreen mode** — F11 to toggle, Escape to exit
- **Loading indicator** — spinner widget during file processing
- **Config persistence** — all settings saved to TOML and restored on startup

## Architecture

Built on the Elm Architecture pattern (State, Message, Update, View) using iced's free functions approach:

```
src/
├── main.rs                  # App entry, iced builder with theme and subscriptions
├── app.rs                   # State struct, new(), update(), subscription()
├── message.rs               # Message enum — all application events
├── model/mod.rs             # Data model for reading state
├── view/
│   ├── mod.rs
│   └── views.rs             # RSVP display with rich_text, controls, spinner
├── infrastructure/
│   ├── mod.rs
│   ├── config.rs            # Config persistence, file dialogs, file processing
│   └── paths.rs             # Path configuration
└── style/
    ├── mod.rs
    └── theme.rs             # Color theming and theme management
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [iced](https://crates.io/crates/iced) 0.14.0 | GUI framework with tokio async runtime |
| [iced_aw](https://crates.io/crates/iced_aw) 0.13.0 | Additional widgets (spinner) |
| [oxidize-pdf](https://crates.io/crates/oxidize-pdf) 1.7.0 | PDF text extraction with encoding support |
| [html2text](https://crates.io/crates/html2text) 0.16.7 | HTML to plain text conversion |
| [rfd](https://crates.io/crates/rfd) 0.17.2 | Native file dialogs |
| [ttf-parser](https://crates.io/crates/ttf-parser) 0.25.1 | Font metadata extraction |
| [serde](https://crates.io/crates/serde) / [toml](https://crates.io/crates/toml) | Config serialization and persistence |
| [derive_more](https://crates.io/crates/derive_more) 2.1.1 | Ergonomic Display derive for error types |
| [pollster](https://crates.io/crates/pollster) 0.4.0 | Blocking on async futures |

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| Left Arrow | Decrease reading speed |
| Right Arrow | Increase reading speed |
| Space | Pause / Resume |
| F11 | Toggle fullscreen |
| Escape | Exit fullscreen |

## Future Implementation

- Custom error types with `derive_more::Display` for structured error handling
- Encoding detection for non-UTF-8 text files (Windows-1252, Latin-1)
- User feedback for partial PDF extraction ("X of Y pages extracted")
- Reading progress persistence across sessions

## How This Was Built

Every line of Rust was written by hand. No AI-generated code, no copy-paste solutions.

[Claude Code](https://docs.anthropic.com/en/docs/claude-code) served strictly as a **theory mentor** — explaining ownership and borrowing concepts, evaluating crate trade-offs (lopdf vs oxidize-pdf vs pdf-extract), walking through compiler errors without writing fixes, and discussing architecture patterns like the Elm Architecture. Claude also generated the commit messages, this README, and project documentation. The rules enforcing the no-code-generation boundary are documented in `CLAUDE.md`.

### Tools Used

| Tool | Role |
|------|------|
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | AI mentor for Rust theory, crate evaluation, and debugging guidance |
| [lite-xl](https://lite-xl.com/) | Text editor — no LSP, no autocomplete, no intellisense |
| [bacon](https://github.com/Canop/bacon) | Background Rust code checker running clippy continuously |
