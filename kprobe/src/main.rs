use chrono::prelude::*;
use libbpf_rs::PerfBufferBuilder;
use std::net::Ipv4Addr;
use std::time::SystemTime;

include!(concat!(env!("OUT_DIR"), "/kprobe.rs"));
include!(concat!(env!("OUT_DIR"), "/kprobe.skel.rs"));

fn handle_lost_events(cpu: i32, count: u64) {
    eprintln!("Lost {count} events on CPU {cpu}");
}

fn main() {
    let mut skel_builder = KprobeSkelBuilder::default();
    skel_builder.obj_builder.debug(true);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();
    skel.attach().unwrap();
    // let _link = skel.progs_mut().tp_napi_gro_receive_entry().attach().unwrap();

    let handle_event = move |cpu: i32, data: &[u8]| {
        let mut data_vec = data.to_vec();
        let (head, body, tail) = unsafe { data_vec.align_to_mut::<event>() };
        assert!(head.is_empty(), "Data was not aligned");
        let mut event = body[0];
        let now = SystemTime::now();
        let datetime = DateTime::<Local>::from(now);
        println!(
            "{} cpu: {}, type: {}, ts: {}, softirq_ts: {}, hardirq_ts: {}, hard_delat: {}, soft_delta: {}, comm: {}, ip: {}->{}",
            datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            cpu,
            event.type_,
            event.ts,
            event.softirq_ts,
            event.hardirq_ts,
            if event.ts > event.hardirq_ts {
                event.ts - event.hardirq_ts
            } else {
                u64::MAX
            },
            if event.ts > event.softirq_ts {
                event.ts - event.softirq_ts
            } else {
                u64::MAX
            },
            unsafe { String::from_utf8_unchecked(event.comm.to_vec()) },
            Ipv4Addr::from(u32::from_be(event.sip)),
            Ipv4Addr::from(u32::from_be(event.dip)),
        );
    };

    let perf = PerfBufferBuilder::new(&skel.maps_mut().events())
        .sample_cb(handle_event)
        .lost_cb(handle_lost_events)
        .build()
        .unwrap();

    loop {
        perf.poll(std::time::Duration::from_millis(100)).unwrap();
    }
}
