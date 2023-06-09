include!(concat!(env!("OUT_DIR"), "/pingserver.rs"));
include!(concat!(env!("OUT_DIR"), "/pingserver.skel.rs"));

fn main() {
    let ifindex = nix::net::if_::if_nametoindex("lo").unwrap();
    let mut skel_builder =PingserverSkelBuilder::default();
    skel_builder.obj_builder.debug(false);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();
    let _link = skel.progs_mut().ping_server().attach_xdp(ifindex as i32);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
