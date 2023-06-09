include!(concat!(env!("OUT_DIR"), "/denyicmp.rs"));
include!(concat!(env!("OUT_DIR"), "/denyicmp.skel.rs"));

fn main() {
    let ifindex = nix::net::if_::if_nametoindex("lo").unwrap();
    let mut skel_builder = DenyicmpSkelBuilder::default();
    skel_builder.obj_builder.debug(true);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();
    let _link = skel.progs_mut().xdp_deny_icmp().attach_xdp(ifindex as i32);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
