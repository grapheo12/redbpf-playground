use std::{ptr, net::Ipv4Addr};
use futures::stream::StreamExt;
use redbpf::load::Loader;
use probes::packet_tracer::PacketInfo;

fn probe_code() -> &'static [u8] {
    include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/bpf/programs/packet_tracer/packet_tracer.elf"
    ))
}

async fn print_traces(l: &mut redbpf::load::Loaded) {
    while let Some((map_name, events)) = l.events.next().await {
        if map_name == "recv_packets" {
            for event in events {
                let info = unsafe { ptr::read(event.as_ptr() as *const PacketInfo) };
                println!("PacketInfo: Addr: {}, Port: {}, Len: {}", Ipv4Addr::from(info.host), info.port, info.len);
            }
        }
    }
}

pub async fn load_and_run_bpf() {
    let mut loaded = Loader::load(probe_code()).expect("error on Loader::load");
    let iface = "eno1";

    let socket_filter = loaded
        .socket_filter_mut("collect_packet_stats")
        .expect("error on Loaded");

    socket_filter
        .attach_socket_filter(iface)
        .expect("error on attaching");

    print_traces(&mut loaded).await;
}