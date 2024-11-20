use raylib_rs_native::{prelude::*, tracelog};

fn main() {
    let mut rl = Core::new(1280, 720, "test");
    tracelog!(rl.tracelog, Info, "Tracelog test");
}
