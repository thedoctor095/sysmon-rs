 <p align="center">
    <a href="https://opensource.org/licenses/Apache-2.0" alt="Apache 2.0 License">
        <img src="https://img.shields.io/badge/License-Apache_2.0-orange.svg" /></a>
    <a href="https://www.rust-lang.org/tools/install" alt="Rust 1.88.0">
        <img src="https://img.shields.io/badge/Rust-1.88.0-orange.svg" /></a>
</p>

**sysmon** is a Rust-based binary which collects system metrics (memory usage, CPU usage, network usage, sensors temperature) and writes them into InfluxDB (OSS v2) using InfluxDB HTTP API.

## Supported & tested platforms

The binary was developed on PopOS 22.04 and tested on Ubuntu 22.04 and Raspberry Pi OS 12 (Bookworm) 

## Prerequisites

- `cargo 1.88.0`
- `rustc 1.88.0`
- `rustup 1.28.2`

## Installation
### PopOS/Ubuntu installation
```
cargo install --git https://github.com/thedoctor095/sysmon-rs
```

### Raspberry Pi OS 12
```
cat <<EOF > .cargo/.config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

rustup target add aarch64-unknown-linux-gnu

cargo install --git https://github.com/thedoctor095/sysmon-rs --target=aarch64-unknown-linux-gnu
```

## Example usage
Note that **204 No Content** is the standard response status when writing to an InfluxDB endpoint.
```
user@pop-os:~$ sysmon

Initialized InfluxDB client for "http://localhost:8086/api/v2/write?bucket=home&org=monitoring&precision=ms" - dumping data in db each 3600 seconds
[InfluxDB] <204 No Content http://localhost:8086/api/v2/write?bucket=home&org=docs&precision=ms>
[InfluxDB] <204 No Content http://localhost:8086/api/v2/write?bucket=home&org=docs&precision=ms>
[InfluxDB] <204 No Content http://localhost:8086/api/v2/write?bucket=home&org=docs&precision=ms>
```

## Contributing

Feel free to contribute to **sysmon** by opening issues, submitting pull requests, or suggesting improvements.

## License

This project is licensed under the Apache-2.0 - see the [LICENSE](LICENSE) file for details
