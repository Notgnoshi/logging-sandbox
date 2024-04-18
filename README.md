# logging-sandbox

This is a sandbox project to experiment with Rust application logging options for a $work project
with the following requirements:

* [ ] Rust 1.68.2, Raspberry Pi 2 equivalent hardware with a _crummy_ disk (writes can block for
      ~20 seconds)
* [ ] Log to disk
    * [ ] Non-blocking I/O
    * [ ] Buffered I/O (flush on log record at most)
    * [ ] Rotate log file on application startup
    * [ ] (optional) Rotate log file when it reaches a maximum size
    * [ ] (optional) Split application logs into several files defined at build time
* [ ] Console logs
    * [ ] Disabled when stdout is not a TTY
    * [ ] Colored (surprisingly high value)
* [ ] Filtering
    * [ ] Can set per-module filters and level override at application startup
    * [ ] Filters and global log level can be set from CLI arguments and a TOML config file
    * [ ] (optional, but really nice) Lazy logging
    * [ ] (optional, but REALLY nice) Change filters while application is running
* [ ] Formatting
    * [ ] UTC ISO-8601 with millisecond precision
    * [ ] Level, module, message
    * [ ] (optional) thread ID / thread name
* [ ] (optional) stderr capture
* [ ] (optional) panic capture
* [ ] (optional) Capture Perfetto traces (How would this work for tight loop performance debugging?)
* [ ] (optional) Capture logs from C++ FFI