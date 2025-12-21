use pnet::datalink;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::{TcpPacket, TcpFlags};
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::env;

fn main() {
    // Get all available network interfaces
    let interfaces = datalink::interfaces();
    
    println!("Available network interfaces:");
    for (i, iface) in interfaces.iter().enumerate() {
        println!("  [{}] {} - {}", i, iface.name, 
                 if iface.is_up() { "UP" } else { "DOWN" });
    }
    
    // Select interface from command line or use first one
    let interface_name = env::args().nth(1);
    let interface = if let Some(name) = interface_name {
        interfaces.into_iter()
            .find(|iface| iface.name == name)
            .expect(&format!("No interface named '{}'", name))
    } else {
        interfaces.into_iter()
            .find(|iface| iface.is_up() && !iface.is_loopback())
            .expect("No suitable network interface found")
    };
    
    println!("\n🔍 Starting packet capture on interface: {}", interface.name);
    println!("Press Ctrl+C to stop\n");
    
    // Create a channel to receive packets
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to create datalink channel: {}", e),
    };
    
    let mut packet_count = 0;
    
    // Start capturing packets
    loop {
        match rx.next() {
            Ok(packet) => {
                packet_count += 1;
                
                if let Some(ethernet) = EthernetPacket::new(packet) {
                    handle_ethernet_packet(&ethernet, packet_count);
                }
            }
            Err(e) => {
                eprintln!("Error receiving packet: {}", e);
            }
        }
    }
}

fn handle_ethernet_packet(ethernet: &EthernetPacket, count: usize) {
    print!("[{}] ", count);
    
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
                handle_ipv4_packet(&ipv4);
            }
        }
        EtherTypes::Ipv6 => {
            if let Some(ipv6) = Ipv6Packet::new(ethernet.payload()) {
                handle_ipv6_packet(&ipv6);
            }
        }
        _ => {
            println!("Other Ethernet type: {:?}", ethernet.get_ethertype());
        }
    }
}

fn handle_ipv4_packet(ipv4: &Ipv4Packet) {
    let source = ipv4.get_source();
    let destination = ipv4.get_destination();
    
    print!("IPv4: {} → {} ", source, destination);
    
    match ipv4.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => {
            if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                println!("| TCP: {}:{} → {}:{} | Flags: {} | Len: {} bytes",
                         source, tcp.get_source(),
                         destination, tcp.get_destination(),
                         get_tcp_flags(&tcp),
                         ipv4.get_total_length());
            }
        }
        IpNextHeaderProtocols::Udp => {
            if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                println!("| UDP: {}:{} → {}:{} | Len: {} bytes",
                         source, udp.get_source(),
                         destination, udp.get_destination(),
                         ipv4.get_total_length());
            }
        }
        IpNextHeaderProtocols::Icmp => {
            println!("| ICMP | Len: {} bytes", ipv4.get_total_length());
        }
        _ => {
            println!("| Protocol: {:?} | Len: {} bytes",
                     ipv4.get_next_level_protocol(),
                     ipv4.get_total_length());
        }
    }
}

fn handle_ipv6_packet(ipv6: &Ipv6Packet) {
    let source = ipv6.get_source();
    let destination = ipv6.get_destination();
    
    print!("IPv6: {} → {} ", source, destination);
    
    match ipv6.get_next_header() {
        IpNextHeaderProtocols::Tcp => {
            if let Some(tcp) = TcpPacket::new(ipv6.payload()) {
                println!("| TCP: {}:{} → {}:{} | Flags: {}",
                         source, tcp.get_source(),
                         destination, tcp.get_destination(),
                         get_tcp_flags(&tcp));
            }
        }
        IpNextHeaderProtocols::Udp => {
            if let Some(udp) = UdpPacket::new(ipv6.payload()) {
                println!("| UDP: {}:{} → {}:{}",
                         source, udp.get_source(),
                         destination, udp.get_destination());
            }
        }
        IpNextHeaderProtocols::Icmpv6 => {
            println!("| ICMPv6");
        }
        _ => {
            println!("| Protocol: {:?}", ipv6.get_next_header());
        }
    }
}

fn get_tcp_flags(tcp: &TcpPacket) -> String {
    let mut flags = Vec::new();
    let flag_bits = tcp.get_flags();
    
    if flag_bits & TcpFlags::SYN != 0 { flags.push("SYN"); }
    if flag_bits & TcpFlags::ACK != 0 { flags.push("ACK"); }
    if flag_bits & TcpFlags::FIN != 0 { flags.push("FIN"); }
    if flag_bits & TcpFlags::RST != 0 { flags.push("RST"); }
    if flag_bits & TcpFlags::PSH != 0 { flags.push("PSH"); }
    if flag_bits & TcpFlags::URG != 0 { flags.push("URG"); }
    
    if flags.is_empty() {
        "NONE".to_string()
    } else {
        flags.join(",")
    }
}

