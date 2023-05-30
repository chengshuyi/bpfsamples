include!(concat!(env!("OUT_DIR"), "/sockops.rs"));
include!(concat!(env!("OUT_DIR"), "/sockops.skel.rs"));
use cgroups_rs::cgroup_builder::*;
use cgroups_rs::*;
use core::slice;
use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::AsRawFd,
    thread,
};

fn main() {
    let cgroup = File::open("/sys/fs/cgroup/unified").unwrap();

    let mut cgroup_procs = File::create("/sys/fs/cgroup/unified/cgroup.procs").unwrap();
    write!(&mut cgroup_procs, "{}", unsafe {libc::getpid()}).unwrap();

    let mut skel_builder = SockopsSkelBuilder::default();
    skel_builder.obj_builder.debug(true);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();
    let _link = skel
        .progs_mut()
        .sockops_example()
        .attach_cgroup(cgroup.as_raw_fd())
        .unwrap();

    let listener = TcpListener::bind("127.0.0.1:5201").unwrap();
    let socket_fd = listener.as_raw_fd() as u64;
    println!("Tcp Server Listening at 5201, and fd is {}", socket_fd);

    thread::spawn(|| {
        let mut _client = TcpStream::connect("127.0.0.1:5201").unwrap();
        println!("Tcp Client connect successfully");
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    let mut _stream = listener.incoming().next().unwrap().unwrap();
    println!("Tcp connection");
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
