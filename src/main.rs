use std::time::Instant;

mod v1;
mod v2;
mod v3;

fn main() {
    let now = Instant::now();
    v1::run_v1();
    eprintln!("Elapsed V1: {:?}", now.elapsed());

    let now = Instant::now();
    v2::run_v2();
    eprintln!("Elapsed V2: {:?}", now.elapsed());

    let now = Instant::now();
    v3::run_v3();
    eprintln!("Elapsed V3: {:?}", now.elapsed());
}
