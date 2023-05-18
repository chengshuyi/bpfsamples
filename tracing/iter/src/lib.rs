include!(concat!(env!("OUT_DIR"), "/iter.rs"));
include!(concat!(env!("OUT_DIR"), "/iter.skel.rs"));

use once_cell::sync::Lazy;
use std::sync::Mutex;

static SKEL: Lazy<Mutex<Option<IterSkel>>> = Lazy::new(|| Mutex::new(None));


fn load(debug:bool) {
    let mut skel_builder = IterSkelBuilder::default();
    skel_builder.obj_builder.debug(debug);
    let mut open_skel = skel_builder.open().unwrap();

    let mut skel = open_skel.load().unwrap();
    *SKEL.lock().unwrap() = Some(skel);
}

fn attach(debug: bool) {
    load(debug);
    SKEL.lock().unwrap().as_mut().map(|x| x.attach().unwrap());
}


pub fn run(debug: bool) {
    attach(debug);
}
