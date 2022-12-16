# Memory-Dumper
A quick and dirty memory dumper written in Rust for Linux. It only reads the stack and the heap, and it outputs the contents to /tmp/output.

# Usage
```
sudo -E cargo run $PID
```
