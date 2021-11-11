extern crate tokio;
extern crate tokio_stream as stream;

use tokio::time::sleep;
use tokio_stream::*;
use std::time::Duration;

#[tokio::main]
async fn main()
{
    println!("program start!");
    let delay  = vec![2_u64, 4, 6, 8];

// streamの生成
    let mut stream = stream::iter(&delay);
    
// async ブロックを実行します
//    let mut handles = Vec::new();
 while let Some(time) = stream.next().await
    {
        sleep(Duration::from_millis(time * 1000)).await;
        println!("{ } sec elapsed", time);
    }


// 即座に実行されます
    println!("hello, world");

// async ブロックの実行完了まで待機します
//    let _ = handle.await.unwrap();

    println!("complete!");
}