# logging-sandbox

This is a sandbox project to experiment with Rust application logging options for a $work project
with the following requirements:

* [X] Rust 1.68.2, Raspberry Pi 2 equivalent hardware with a _crummy_ disk (writes can block for
      ~20 seconds)
* [O] Log to disk
    * [X] Non-blocking I/O
    * [X] Buffered I/O (flush on log record at most)
    * [X] Rotate log file on application startup
    * [X] (optional) Rotate log file when it reaches a maximum size
    * [ ] (optional) Split application logs into several files defined at build time
* [o] Console logs
    * [X] Colored (surprisingly high value)
    * [ ] (optional) Disabled when stdout is not a TTY
* [O] Filtering
    * [X] Can set per-module filters and level override at application startup (through `RUST_LOG`
          environment variable)
    * [X] Filters and global log level can be set from CLI arguments and a TOML config file
    * [ ] (optional, but really nice) Lazy logging
    * [X] (optional, but REALLY nice) Change filters while application is running
* [X] Formatting
    * [X] UTC ISO-8601 with millisecond precision
    * [X] Level, module, message
    * [X] (optional) thread ID / thread name
* [X] `log` interopt
* [ ] (optional) stderr capture
* [ ] (optional) panic capture
* [ ] (optional) Capture Perfetto traces (How would this work for tight loop performance debugging?)
* [ ] (optional) Capture logs from C++ FFI
