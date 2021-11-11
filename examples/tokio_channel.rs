extern crate tokio;

use tokio::time::sleep;
use tokio::sync::{mpsc, oneshot};
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main()
{
//    let (sx, rx) = mpsc::channel(100);
    let (sx, mut rx) = mpsc::channel(100);
    sx.send(true).await.unwrap();
    let mut sig = false;
    let sig_arc = Arc::new(&sig);
// async ブロックを実行します
    let handle_act = 
        tokio::spawn(
        async move {
            let delay_time = 50;
            let sleep = sleep(Duration::from_millis(delay_time));
            tokio::pin!(sleep);
        
            loop {
                tokio::select! {
                    _ = &mut sleep => {
                        tokio::time::sleep(Duration::from_millis(delay_time)).await;
                    },
                    msg = rx.recv() => {
                        if !msg.unwrap() {
                            break;
                        }
                    }
                }
            }
        }
    );

// 即座に実行されます
    println!("hello, world");

    sleep(Duration::from_millis(30000)).await;
    sx.send(false).await.unwrap();
    sleep(Duration::from_millis(30000)).await;


// async ブロックの実行完了まで待機します
    let _ = handle_act .await.unwrap();

    println!("complete!");
}