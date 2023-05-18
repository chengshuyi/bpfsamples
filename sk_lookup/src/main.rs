include!(concat!(env!("OUT_DIR"), "/sk_lookup.rs"));
include!(concat!(env!("OUT_DIR"), "/sk_lookup.skel.rs"));
use core::slice;
use std::{
    fs::File,
    io::{Read, Write},
    net::TcpListener,
    os::fd::AsRawFd,
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5201").unwrap();
    let socket_fd = listener.as_raw_fd() as u64;
    let netns = File::open("/proc/self/ns/net").unwrap();
    println!("Tcp Server Listening at 5201, and fd is {}", socket_fd);

    let _ = thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            thread::spawn(move || {
                let mut buf = [0; 1024];
                loop {
                    let bytes_read = stream.read(&mut buf).unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    stream.write(&buf[..bytes_read]).unwrap();
                }
            });
        }
    });

    let mut skel_builder = SkLookupSkelBuilder::default();
    skel_builder.obj_builder.debug(false);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();

    let key: u16 = 5202;
    let val: u8 = 0;
    println!("insert 5202 port into ports map");
    unsafe {
        skel.maps_mut()
            .ports()
            .update(
                slice::from_raw_parts(&key as *const u16 as *const u8, 2),
                slice::from_raw_parts(&val, 1),
                libbpf_rs::MapFlags::ANY,
            )
            .unwrap()
    };
    println!("insert socketfd[{}] into sockets map", socket_fd);
    let key: u32 = 0;
    unsafe {
        skel.maps_mut()
            .sockets()
            .update(
                slice::from_raw_parts(&key as *const u32 as *const u8, 4),
                slice::from_raw_parts(&socket_fd as *const u64 as *const u8, 8),
                libbpf_rs::MapFlags::ANY,
            )
            .unwrap()
    }
    println!("attach eBPF program with netns fd: {}", netns.as_raw_fd());
    let _link = skel
        .progs_mut()
        .sk_lookup_example()
        .attach_netns(netns.as_raw_fd())
        .unwrap();
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
