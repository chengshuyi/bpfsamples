include!(concat!(env!("OUT_DIR"), "/sk_lookup.rs"));
include!(concat!(env!("OUT_DIR"), "/sk_lookup.skel.rs"));
use core::slice;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

fn main() {
    let _ = thread::spawn(|| start_tcp_echo_server());
    load_sk_lookup(true, 5201);
}

fn start_tcp_echo_server() {
    let listener = TcpListener::bind("127.0.0.1:5201").unwrap();

    println!(" Echo Server Listening at 5201");
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
}


fn load_sk_lookup(debug: bool, port: u16) {
    let mut skel_builder = SkLookupSkelBuilder::default();
    skel_builder.obj_builder.debug(debug);
    let mut open_skel = skel_builder.open().unwrap();

    let mut skel = open_skel.load().unwrap();
    skel.attach().unwrap();

    
    // skel.maps_mut().ports().update(, &[0], libbpf_rs::MapFlags::ANY);

}

// fn load(debug:bool) {
//     let mut skel_builder = FentrySkelBuilder::default();
//     skel_builder.obj_builder.debug(debug);
//     let mut open_skel = skel_builder.open().unwrap();

//     let mut skel = open_skel.load().unwrap();
//     *SKEL.lock().unwrap() = Some(skel);
// }

// fn attach(debug: bool) {
//     load(debug);
//     SKEL.lock().unwrap().as_mut().map(|x| x.attach().unwrap());
// }
