include!(concat!(env!("OUT_DIR"), "/sk_skb.rs"));
include!(concat!(env!("OUT_DIR"), "/sk_skb.skel.rs"));

use core::slice;
use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::AsRawFd,
    thread,
};

static mut client_proxy_fd: u64 = 0;
static mut proxy_backend_fd: u64 = 0;

fn start_backend() {
    let listener = TcpListener::bind("127.0.0.1:5202").unwrap();
    let socket_fd = listener.as_raw_fd() as u64;
    println!(
        "Tcp Backend Server Listening at 5202, and fd is {}",
        socket_fd
    );

    thread::spawn(move || {
        let mut stream = listener.incoming().next().unwrap().unwrap();
        let mut buf = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buf).unwrap();
            if bytes_read == 0 {
                break;
            }
            println!(
                "backend server receive: {}",
                String::from_utf8(buf.to_vec()).unwrap()
            );
        }
    });
}

fn start_proxy() {
    let mut client = TcpStream::connect("127.0.0.1:5202").unwrap();
    unsafe {
        proxy_backend_fd = client.as_raw_fd() as u64;
        println!(
            "Tcp Proxy connect successfully to backend, and fd is {}",
            proxy_backend_fd
        );
    }

    let listener = TcpListener::bind("127.0.0.1:5201").unwrap();
    let socket_fd = listener.as_raw_fd() as u64;
    println!(
        "Tcp Proxy Server Listening at 5201, and fd is {}",
        socket_fd
    );

    thread::spawn(move || {
        let mut stream = listener.incoming().next().unwrap().unwrap();
        let mut buf = [0; 1024];
        unsafe { client_proxy_fd = stream.as_raw_fd() as u64 };
        loop {
            let bytes_read = stream.read(&mut buf).unwrap();
            if bytes_read == 0 {
                println!("break");
                break;
            }
            println!(
                "proxy server receive: {}",
                String::from_utf8(buf.to_vec()).unwrap()
            );

            println!(
                "proxy client send: {}",
                String::from_utf8(buf.to_vec()).unwrap()
            );
            client.write(&buf[..bytes_read]).unwrap();
        }
    });
}

fn start_client() {
    thread::spawn(|| {
        let mut client = TcpStream::connect("127.0.0.1:5201").unwrap();
        println!("Client connect successfully to proxy");
        loop {
            println!("client send: i'm client");
            client.write(b"i'm client").unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn main() {
    start_backend();
    start_proxy();
    start_client();

    let mut skel_builder = SkSkbSkelBuilder::default();
    skel_builder.obj_builder.debug(false);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();
    let mut key: u32 = 0;
    let sockmap_fd = skel.maps().sockmap().fd();

    let _link = skel
        .progs_mut()
        .stream_verdict()
        .attach_sockmap(sockmap_fd)
        .unwrap();

    let _link2 = skel
        .progs_mut()
        .stream_parser()
        .attach_sockmap(sockmap_fd)
        .unwrap();

    thread::sleep(std::time::Duration::from_secs(10));

    unsafe {
        println!(
            "insert client-to-proxy socketfd[{}] into sockets map with key 0",
            client_proxy_fd
        );

        // add server fd into sockmap
        skel.maps_mut()
            .sockmap()
            .update(
                slice::from_raw_parts(&key as *const u32 as *const u8, 4),
                slice::from_raw_parts(&(client_proxy_fd) as *const u64 as *const u8, 8),
                libbpf_rs::MapFlags::ANY,
            )
            .unwrap();
        thread::sleep(std::time::Duration::from_millis(100));
        println!(
            "insert client socketfd[{}] into sockets map with key 1",
            proxy_backend_fd
        );
        // add client fd into sockmap
        key = 1;
        skel.maps_mut()
            .sockmap()
            .update(
                slice::from_raw_parts(&key as *const u32 as *const u8, 4),
                slice::from_raw_parts(&proxy_backend_fd as *const u64 as *const u8, 8),
                libbpf_rs::MapFlags::ANY,
            )
            .unwrap()
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
