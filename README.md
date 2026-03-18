# Packet Sniffer

A simple command-line packet sniffer written in Rust that captures and displays network traffic in real-time.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

> ⚠️ **Legal Notice:** Only use this tool on networks you own or have explicit permission to monitor. Capturing traffic on networks without authorization may violate local laws, including wiretapping statutes such as the U.S. Electronic Communications Privacy Act (ECPA). You are solely responsible for ensuring your use complies with all applicable laws and regulations.

---

## Features

- Real-time packet capture from network interfaces
- IPv4 and IPv6 protocol support
- TCP/UDP port and flag information
- ICMP and ICMPv6 detection
- ARP packet identification
- Clean, readable output format

---

## Requirements

- Rust 1.70 or later
- Root/sudo privileges required for packet capture

### Linux

```bash
sudo apt install libpcap-dev
```

### macOS

libpcap is included with macOS. No additional installation needed.

```bash
# Optionally install via Homebrew if needed
brew install libpcap
```

### Windows

Windows is not currently supported. Contributions are welcome.

---

## Installation

```bash
git clone <your-repo-url>
cd packet-sniffer
cargo build --release
```

---

## Usage

Run with `sudo` to capture packets:

```bash
# Auto-select first active network interface
sudo ./target/release/packet-sniffer

# Specify a network interface
sudo ./target/release/packet-sniffer eth0
```

---

## Example Output

```
Available network interfaces:
  [0] lo - UP
  [1] eth0 - UP

🔍 Starting packet capture on interface: eth0
Press Ctrl+C to stop

[1] IPv4: 192.168.1.100 → 142.250.185.46 | TCP: 192.168.1.100:54321 → 142.250.185.46:443 | Flags: SYN | Len: 60 bytes
[2] IPv4: 142.250.185.46 → 192.168.1.100 | TCP: 142.250.185.46:443 → 192.168.1.100:54321 | Flags: SYN,ACK | Len: 60 bytes
[3] IPv4: 8.8.8.8 → 192.168.1.100 | UDP: 8.8.8.8:53 → 192.168.1.100:34567 | Len: 89 bytes
```

### Output Format

Each captured packet is printed on a single line with the following fields:

| Field | Description |
|-------|-------------|
| `[N]` | Packet counter |
| `IPv4` / `IPv6` | Network layer protocol |
| `src → dst` | Source and destination IP addresses |
| `TCP` / `UDP` | Transport layer protocol with ports |
| `Flags` | TCP control flags (see below) |
| `Len` | Packet length in bytes |

### TCP Flags

| Flag | Meaning |
|------|---------|
| `SYN` | Synchronize — initiates a connection |
| `ACK` | Acknowledge — confirms receipt |
| `FIN` | Finish — closes a connection |
| `RST` | Reset — abruptly terminates a connection |
| `PSH` | Push — send buffered data immediately |
| `URG` | Urgent — prioritize this data |

---

## Supported Protocols

- Ethernet frames
- IPv4 / IPv6
- TCP (with flags: SYN, ACK, FIN, RST, PSH, URG)
- UDP
- ICMP / ICMPv6
- ARP

### Limitations

- No payload/content inspection
- No BPF filter support (cannot filter by IP, port, or protocol at capture time)
- No pcap file export
- Capture interface only — cannot read existing `.pcap` files

---

## Troubleshooting

**Permission denied**

Packet capture requires root privileges. Always run with `sudo`:

```bash
sudo ./target/release/packet-sniffer
```

**`libpcap` not found at build time (Linux)**

Install the development headers before building:

```bash
sudo apt install libpcap-dev
```

**No packets appearing**

- Make sure you are on the correct interface (try specifying it explicitly)
- Confirm there is active network traffic on that interface
- On some systems, the loopback interface (`lo`) may show no external traffic

**Interface not listed**

Some virtual or inactive interfaces may not appear. Run `ip link` (Linux) or `ifconfig` (macOS) to list all available interfaces on your system.

---

## Dependencies

- [`pnet`](https://crates.io/crates/pnet) — Low-level packet manipulation
- [`pnet_datalink`](https://crates.io/crates/pnet) — Data link layer capture

---

## Contributing

Contributions are welcome. Please open an issue to discuss your idea before submitting a pull request. Bug reports and feature requests are also appreciated.

---

## License

MIT — see [LICENSE](LICENSE) for details.
