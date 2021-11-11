extern crate rayon;

use rayon::prelude::*;
use std::thread;
use thread::sleep;
use std::time::Duration;


fn spawn_once()
{
    for i in 15..30
    {
        rayon::spawn(move || 
        {
            println!("Task executes on thread: {:?}", thread::current().id());
            sleep(Duration::from_millis(i *500));
            println!("{ } sec elapsed", i as f64 / 2.0);
        });
    }
}

fn spawn_parallel()
{
    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

    pool.scope(|s| {
        for i in 15..30
        {
            s.spawn(move |_|
                {
                    println!("Task executes on thread: {:?}", thread::current().id());
                    sleep(Duration::from_millis(i*1000));
                    println!("{ } sec elapsed", i);
                }
            );
        }
    });
    println!("spawn clear(scope)");
}

fn par_iter()
{
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let vec = 
    pool.install(||
    {
        (15_u64..30).into_par_iter().map(|i| 
            {
                println!("Task executes on thread: {:?}", thread::current().id());
                sleep(Duration::from_millis(2000));
                println!("{ }th task", i);
                i as f64
            }
        ).collect::<Vec<f64>>()
    });
    println!("spawn clear(par iter)");
    println!("task clear");

    println!("{:?}", vec);
}

fn spawn_mspc()
{
    let pool = 
        rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();

    let vec = 
    pool.scope_fifo(|s| {
        let (tx, rx) = std::sync::mpsc::channel();
        for i in 15..30
        {
            let tx = tx.clone();
            s.spawn_fifo(move |_|
                {
                    tx.send(i as f64).unwrap();     //送られた値の順序は順不同になる可能性がある
                    println!("Task executes on thread: {:?}", thread::current().id());
                    sleep(Duration::from_millis(2000));
                    println!("{ }th task", i);
                }
            );
        }
        drop(tx); 
        rx.into_iter().collect::<Vec<f64>>()
    });
    println!("spawn clear(scope)");

    println!("{:?}", vec);

}

fn main()
{
    rayon::ThreadPoolBuilder::new()
        //.num_threads(4)
        .build_global()
        .unwrap();

//    par_iter();
    spawn_mspc();
}