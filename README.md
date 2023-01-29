# TCPing

## Install
```shell
cargo install --git https://github.com/Rehtt/tcping_rs
```

## Use
```shell
Simple TCP Ping write with rust

Usage: tcping_rs.exe [OPTIONS] <TARGET> [PORT]

Arguments:
  <TARGET>  target
  [PORT]    port [default: 80]

Options:
  -w <W>         -w 5 :ping every 5 seconds [default: 1]
  -l <L>         -l 5 :send 5 pings [default: 3]
  -t <T>         -t 5 :timeout 5 seconds [default: 2]
  -h, --help     Print help
  -V, --version  Print version


tcping_rs 8.8.8.8 80
tcping_rs 8.8.8.8 80-8080
```