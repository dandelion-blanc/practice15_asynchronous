extern crate tokio;

use tokio::time::sleep;
use core::f64;
use std::time::Duration;

async fn sphere(x :&[f64]) -> f64
{
    x.iter().map(|x| *x **x).sum()
}

#[tokio::main]
async fn main()
{
    let mut handles = Vec::new();

// async ブロックを実行します
    for i in 1..100_u64
    {
        handles.push(
            tokio::spawn(
            async move {
                //let val = i.pow(i as u32);
                sleep(Duration::from_millis(i *1000)).await;
                println!("{ } sec elapsed", i);
                //println!("{ }^2 = { }", i, val);
            })
        );
    }

// 即座に実行されます
    println!("start process");


// async ブロックの実行完了まで待機します
    for handle in handles.into_iter()
    {
        let _ = handle.await.unwrap();
    }

    println!("complete!");
}