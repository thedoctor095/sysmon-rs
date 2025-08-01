 <p align="center">
    <a href="https://opensource.org/licenses/Apache-2.0" alt="Apache 2.0 License">
        <img src="https://img.shields.io/badge/License-Apache_2.0-orange.svg" /></a>
    <a href="https://www.rust-lang.org/tools/install" alt="Rust 1.88.0">
        <img src="https://img.shields.io/badge/Rust-1.88.0-orange.svg" /></a>
</p>

**sysmon** is a Rust-based binary which collects system metrics (disks usage, networks usage, sensors temperature, system usage (memory usage, CPU usage)) and writes them into InfluxDB (OSS v2) using InfluxDB HTTP API.

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
## Configuration
**sysmon** expects to find a config file in the path `$HOME/.config/sysmon.ini` and will not initialize without it.

The config file contents are listed below:
```
[sys-mon]
# the interval at which the metrics are collected and dumped in the DB
# collect & dump metrics each 60 minutes
dump_interval_seconds = 3600

[influxdb]
# the bucket into which to write
bucket=home
# the organisation in which the bucket is found
organisation=docs
# the authorization token for writing to the DB API
token=token
# the URI where the metrics are sent
uri=http://localhost:8086/api/v2/write
```

All metrics will be dumped using *milisecond* precision.

## Example usage
Note that **204 No Content** is the standard response status when writing to an InfluxDB endpoint.
```
user@pop-os:~$ sysmon

Initialized InfluxDB client for "http://localhost:8086/api/v2/write?bucket=home&org=docs&precision=ms" - dumping data in db each 3600 seconds
[InfluxDB] <204 No Content http://localhost:8086/api/v2/write?bucket=home&org=docs&precision=ms>
[InfluxDB] <204 No Content http://localhost:8086/api/v2/write?bucket=home&org=docs&precision=ms>
[InfluxDB] <204 No Content http://localhost:8086/api/v2/write?bucket=home&org=docs&precision=ms>
```

## Example data dumped in InfluxDB
```
# Disks usage data
disk_usage,name=/dev/mapper/data-root,mount=/,file_system=ext4,kind=SSD available_space=156048510976i,total_space=241369505792i,total_written=9239375872i,written_since=0i,total_read=3956081664i,read_since=0i 1735689600000
disk_usage,name=/dev/nvme0n1p2,mount=/recovery,file_system=vfat,kind=SSD available_space=1071370240i,total_space=4286562304i,total_written=1024i,written_since=0i,total_read=6729728i,read_since=0i 1735689600000
disk_usage,name=/dev/nvme0n1p1,mount=/boot/efi,file_system=vfat,kind=SSD available_space=853307392i,total_space=1069522944i,total_written=1024i,written_since=0i,total_read=4104192i,read_since=0i 1735689600000


# Network usage data
network_usage,interface=<INTERFACE_NAME>,mac_address=<<:MAC:ADD:RE:SS:>> received=0i,total_received=0i,transmitted=0i,total_transmitted=0i,packets_received=0i,total_packets_received=0i,packets_transmitted=0i,total_packets_transmitted=0i,errors_on_received=0i,total_errors_on_received=0i,errors_on_transmitted=0i,total_errors_on_transmitted=0i 1735689600000
network_usage,interface=<INTERFACE_NAME>,mac_address=<<:MAC:ADD:RE:SS:>> received=0i,total_received=38260082i,transmitted=0i,total_transmitted=38260082i,packets_received=0i,total_packets_received=51436i,packets_transmitted=0i,total_packets_transmitted=51436i,errors_on_received=0i,total_errors_on_received=0i,errors_on_transmitted=0i,total_errors_on_transmitted=0i 1735689600000
network_usage,interface=<INTERFACE_NAME>,mac_address=<<:MAC:ADD:RE:SS:>> received=0i,total_received=269045444i,transmitted=0i,total_transmitted=15399644i,packets_received=0i,total_packets_received=233000i,packets_transmitted=0i,total_packets_transmitted=95708i,errors_on_received=0i,total_errors_on_received=0i,errors_on_transmitted=0i,total_errors_on_transmitted=0i 1735689600000


# Sensors temperature
sensor_data,sensor_label=coretemp\ Core\ 0 temperature=23,max_temperature=23,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 29 temperature=25,max_temperature=25,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 12 temperature=25,max_temperature=25,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 16 temperature=24,max_temperature=24,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 8 temperature=25,max_temperature=25,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 31 temperature=23,max_temperature=23,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 28 temperature=23,max_temperature=23,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Package\ id\ 0 temperature=27,max_temperature=27,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 4 temperature=23,max_temperature=23,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 30 temperature=24,max_temperature=24,critical_temperature=100 1735689600000
sensor_data,sensor_label=coretemp\ Core\ 20 temperature=26,max_temperature=26,critical_temperature=100 1735689600000
sensor_data,sensor_label=nvme\ Sensor\ 1\ <DEVICE\ NAME> temperature=53.85,max_temperature=53.85,critical_temperature=0 1735689600000
sensor_data,sensor_label=nvme\ Sensor\ 2\ <DEVICE\ NAME> temperature=40.85,max_temperature=40.85,critical_temperature=0 1735689600000
sensor_data,sensor_label=nvme\ Composite\ <DEVICE\ NAME>  temperature=40.85,max_temperature=40.85,critical_temperature=84.85 1735689600000
sensor_data,sensor_label=acpitz\ temp1 temperature=27.8,max_temperature=27.8,critical_temperature=0 1735689600000
sensor_data,sensor_label=nvme\ Composite\ <DEVICE\ NAME> temperature=27.85,max_temperature=27.85,critical_temperature=70.85 1735689600000
sensor_data,sensor_label=nvme\ Sensor\ 2\ <DEVICE\ NAME> temperature=34.85,max_temperature=34.85,critical_temperature=0 1735689600000
sensor_data,sensor_label=nvme\ Sensor\ 1\ <DEVICE\ NAME> temperature=27.85,max_temperature=27.85,critical_temperature=0 1735689600000


# System usage
system_usage,boot_time=2025-01-01T00:00:00Z,host_name=<hostname> available_memory=21662388224i,free_memory=17692499968i,used_memory=11792936960i,free_swap=21474299904i,used_swap=0i,total_swap=21474299904i,global_cpu_usage=4.781705 1735689600000
```

## Contributing

Feel free to contribute to **sysmon** by opening issues, submitting pull requests, or suggesting improvements.

## License

This project is licensed under the Apache-2.0 - see the [LICENSE](LICENSE) file for details
