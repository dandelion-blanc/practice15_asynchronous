extern crate rayon;
extern crate rand;

use rayon::prelude::*;
use rand::prelude::*;

use std::thread;
use thread::sleep;
use std::time::Duration;
use std::time::Instant;


fn fib(i: u64) -> u64 {
    match i {
        0 => 1,
        1 => 1,
        _ => fib(i-2) + fib(i-1),
    }
}

fn main()
{
/*
let cfg = rayon::Configuration::new();
rayon::initialize(cfg.set_num_threads(4)).unwrap();
*/
    rayon::ThreadPoolBuilder::new()
        //.num_threads(4)
        .build_global()
        .unwrap();

    let mut rng = thread_rng();
    let xs: Vec<_> = (0..1000000).map(|_| rng.gen_range(0..24)).collect();
    let ys = xs.clone();

    let begin = Instant::now();
    let max = xs.par_iter().map(|&x| fib(x)).max(); // `par_iter`を使って並列化
    println!("Parallel: {:?}; elapsed: {:?}", max, begin.elapsed());

    let begin = Instant::now();
    let max = ys.iter().map(|&x| fib(x)).max(); // 普通のIterator
    println!("Sync: {:?}; elapsed: {:?}", max, begin.elapsed());

}