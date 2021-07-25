# cmd_timer

This is a simple timer that can launch from your command line.

- clone the repo
- `cargo build --release`
- cp the binary (located at `/target/release/timer`) to your binary path. You can Check `copy.sh` as an example

Now you can launch timer from the command line.

```bash
timer 1h "focus study" &

# or
timer 15m sleep &

# or
timer 5m30s meditation &
```