use libbpf_rs::PerfBufferBuilder;

include!(concat!(env!("OUT_DIR"), "/kprobe.rs"));
include!(concat!(env!("OUT_DIR"), "/kprobe.skel.rs"));

fn handle_lost_events(cpu: i32, count: u64) {
    eprintln!("Lost {count} events on CPU {cpu}");
}

fn main() {
    let skel_builder = KprobeSkelBuilder::default();
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();
    skel.attach().unwrap();

    let handle_event = move |cpu: i32, data: &[u8]| {
        let mut data_vec = data.to_vec();
        let (head, body, tail) = unsafe { data_vec.align_to_mut::<event>() };
        assert!(head.is_empty(), "Data was not aligned");
        let mut event = body[0];
        println!(
            "cpu: {}, type: {}, ts: {}, comm: {}",
            cpu,
            event.type_,
            event.ts,
            unsafe { String::from_utf8_unchecked(event.comm.to_vec()) }
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
