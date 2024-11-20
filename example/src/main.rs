use raylib_rs_native::{prelude::*, tracelog};

fn main() {
    println!("start");
    tracelog!(TraceLogLevel::Warning, "test");
    println!("finish");
}
