# Tee Rotate

Tee Rotate is a command-line utility written in Rust that combines the functionality of the tee command with log rotation. It allows you to simultaneously write the standard input to multiple output sources while also rotating log files based on size.

## Usage: 

```
cargo run -- [OPTIONS] <output_file>...
```

### Example:

```
while true; do echo "Message"; sleep 1; done | cargo run -- -a -r -s 55 log_file
```

## Options:
 - `-h, --help`                             Print help
 - `-a, --append`                           Append to the given FILEs, do not overwrite
 - `-r, --rotate`                           Rotate files with a maximum size of MAX_SIZE_BYTES
 - `-s, --max-size-bytes <max-size-bytes>`  Maximum size of each file in bytes
 - `-n, --max-log-files <max-log-files>`    Maximum number of log files to keep
 - `--output-error[=<output-error>]`        Set write error behavior
 - `-V, --version`                          Print version