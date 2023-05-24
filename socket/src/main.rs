include!(concat!(env!("OUT_DIR"), "/socket.rs"));
include!(concat!(env!("OUT_DIR"), "/socket.skel.rs"));

use core::slice;
use std::{
    fs::File,
    io::{Read, Write},
    net::TcpListener,
    os::fd::AsRawFd,
    thread,
};

fn attach_socket(prog_fd: i32, socket_fd: i32) {
    unsafe {
        assert_eq!(
            libc::setsockopt(
                socket_fd,
                libc::SOL_SOCKET,
                libc::SO_ATTACH_BPF,
                &prog_fd as *const i32 as *const libc::c_void,
                4,
            ),
            0,
            "failed to bind eBPF program into socket"
        );
    };
}

fn main() {
    let mut skel_builder = SocketSkelBuilder::default();
    skel_builder.obj_builder.debug(true);
    let open_skel = skel_builder.open().unwrap();
    let skel = open_skel.load().unwrap();
    let prog_fd = skel.progs().socket_filter_example().fd();
    println!("Load socket filter eBPF program, program fd is {}", prog_fd);

    let listener = TcpListener::bind("127.0.0.1:5201").unwrap();
    let socket_fd = listener.as_raw_fd() as u64;
    println!("Tcp Server Listening at 5201, and fd is {}", socket_fd);

    let _ = thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            attach_socket(prog_fd, stream.as_raw_fd());
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

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
