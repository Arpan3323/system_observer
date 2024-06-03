# System Observer
A cross-platform Terminal User Interface (TUI) written in Rust
![](/docs/sys-obs-demo-gif.gif)

This application uses the following Crates:
- [crossterm](https://docs.rs/crossterm/latest/crossterm/): Creates a cross-platform text-based terminal interface with a mutable buffer that can be manipulated and re-used for rendering. This crate is also used to handle key inputs via `event` module.

- [ratatui](https://docs.rs/ratatui/latest/ratatui/): Uses `crossterm` as a backend to intialize a terminal, handle events, draw UI elements (layouts and custom stateful and static widgets) by re-rendering a mutable buffer each frame update, and restore terminal state.

- [sysinfo](https://docs.rs/sysinfo/latest/sysinfo/index.html): To get the system details and add the functionality to [kill](https://docs.rs/sysinfo/latest/sysinfo/struct.Process.html#method.kill) a thread of a system process