# Packet Sniffer

A simple command-line packet sniffer written in Rust that captures and displays network traffic in real-time.

## Features

- Real-time packet capture from network interfaces
- IPv4 and IPv6 protocol support
- TCP/UDP port and flag information
- ICMP and ICMPv6 detection
- ARP packet identification
- Clean, readable output format

## Requirements

- Rust 1.70 or later
- Linux with libpcap: `sudo apt install libpcap-dev`
- **Root/sudo privileges** required for packet capture

## Installation

```bash
git clone <your-repo-url>
cd packet-sniffer
cargo build --release
```

## Usage

Run with sudo to capture packets:

```bash
# Auto-select first active network interface
sudo ./target/release/packet-sniffer

# Specify a network interface
sudo ./target/release/packet-sniffer eth0
```

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

## Supported Protocols

- Ethernet frames
- IPv4 / IPv6
- TCP (with flags: SYN, ACK, FIN, RST, PSH, URG)
- UDP
- ICMP / ICMPv6
- ARP

## Dependencies

- `pnet` - Low-level packet manipulation
- `pnet_datalink` - Data link layer capture

## License

MIT
