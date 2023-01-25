#![no_std]
#![no_main]

use core::mem::{self, MaybeUninit};
use redbpf_probes::socket_filter::prelude::*;
use memoffset::offset_of;
use probes::packet_tracer::PacketInfo;

program!(0xFFFFFFFE, "GPL");

#[map(link_section = "maps/recv_packets")]
static mut recv_packets: PerfMap<PacketInfo> = PerfMap::with_max_entries(10240);

#[socket_filter]
fn collect_packet_stats(skb: SkBuff) -> SkBuffResult {
    let eth_len = mem::size_of::<ethhdr>();
    let eth_proto = skb.load::<__be16>(offset_of!(ethhdr, h_proto))? as u32;
    let mut info = PacketInfo {
        host: 0,
        len: 0,
        port: 0
    };

    if eth_proto != ETH_P_IP {
        return Ok(SkBuffAction::Ignore);
    }


    let ip_proto = skb.load::<__u8>(eth_len + offset_of!(iphdr, protocol))? as u32;
    if ip_proto != IPPROTO_TCP {
        return Ok(SkBuffAction::Ignore);
    }

    let ip_len = mem::size_of::<iphdr>();

    let port = skb.load::<__be16>(eth_len + ip_len + offset_of!(tcphdr, dest))? as u32;

    if port != 6142 {
        return Ok(SkBuffAction::Ignore);
    }

    info.host = skb.load::<__u32>(eth_len + offset_of!(iphdr, saddr))? as u32;
    info.len = skb.load::<__be16>(eth_len + offset_of!(iphdr, tot_len))? as u32;
    info.port = port;
    unsafe {
        recv_packets.insert(skb.skb as *mut __sk_buff, &info);
    }
    
    Ok(SkBuffAction::Ignore)
}

    


