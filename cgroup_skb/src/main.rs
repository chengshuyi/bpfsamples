use std::{fs::File, os::fd::AsRawFd};

include!(concat!(env!("OUT_DIR"), "/cgroup_skb.rs"));
include!(concat!(env!("OUT_DIR"), "/cgroup_skb.skel.rs"));

fn main() {
    let cgroup = File::open("/sys/fs/cgroup/unified").unwrap();
    let mut skel_builder = CgroupSkbSkelBuilder::default();
    skel_builder.obj_builder.debug(false);
    let open_skel = skel_builder.open().unwrap();
    let mut skel = open_skel.load().unwrap();
    let _link = skel
        .progs_mut()
        .cgroup_skb_ingress()
        .attach_cgroup(cgroup.as_raw_fd());

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
