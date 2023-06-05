include!(concat!(env!("OUT_DIR"), "/sk_msg.rs"));
include!(concat!(env!("OUT_DIR"), "/sk_msg.skel.rs"));
use core::slice;
use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::AsRawFd,
    thread,
};

static mut client_fd: u64 = 0;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5201").unwrap();
    let socket_fd = listener.as_raw_fd() as u64;
    println!("Tcp Server Listening at 5201, and fd is {}", socket_fd);

    thread::spawn(|| {
        let mut client = TcpStream::connect("127.0.0.1:5201").unwrap();
        unsafe { client_fd = client.as_raw_fd() as u64 };
        println!("Tcp Client connect successfully");
        loop {
            println!("client send: i'm client");
            client.write(b"i'm client").unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let mut skel_builder = SkMsgSkelBuilder::default();
    skel_builder.obj_builder.debug(false);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();

    let mut key: u32 = 0;

    let sockmap_fd = skel.maps().sockets().fd();
    let _link = skel
        .progs_mut()
        .sk_msg_example()
        .attach_sockmap(sockmap_fd)
        .unwrap();

    let mut stream = listener.incoming().next().unwrap().unwrap();
    unsafe {
        println!(
            "insert server socketfd[{}] into sockets map with key 0",
            stream.as_raw_fd()
        );

        // add server fd into sockmap
        skel.maps_mut()
            .sockets()
            .update(
                slice::from_raw_parts(&key as *const u32 as *const u8, 4),
                slice::from_raw_parts(&(stream.as_raw_fd() as u64) as *const u64 as *const u8, 8),
                libbpf_rs::MapFlags::ANY,
            )
            .unwrap();
        thread::sleep(std::time::Duration::from_millis(100));
        println!(
            "insert client socketfd[{}] into sockets map with key 1",
            client_fd
        );
        // add client fd into sockmap
        key = 1;
        skel.maps_mut()
            .sockets()
            .update(
                slice::from_raw_parts(&key as *const u32 as *const u8, 4),
                slice::from_raw_parts(&client_fd as *const u64 as *const u8, 8),
                libbpf_rs::MapFlags::ANY,
            )
            .unwrap()
    }
    thread::spawn(move || {
        let mut buf = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buf).unwrap();
            if bytes_read == 0 {
                println!("break");
                break;
            }
            println!(
                "server receive: {}",
                String::from_utf8(buf.to_vec()).unwrap()
            );
        }
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
