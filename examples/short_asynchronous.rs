extern crate tokio;

use std::thread;
use std::time::Duration;

use tokio::time::sleep;
use tokio::runtime;


async fn some_palallel_once()
{
    // async ブロックを実行します
    let handle = 
        tokio::spawn(
        async {
                sleep(Duration::from_millis(2000)).await;
                println!("2 sec elapsed");
        }
    );

    // 即座に実行されます
    println!("hello, world");


    // async ブロックの実行完了まで待機します
    let _ = handle.await.unwrap();

    println!("complete!");
}
async fn some_palallel()
{
    let mut handles = Vec::new();
    for i in 15..30
    {
        handles.push(tokio::spawn(
            async move
            {
                println!("Task executes on thread: {:?}", thread::current().id());
                sleep(Duration::from_millis(i *1000)).await;
                println!("{ } sec elapsed", i);
            }
        ));
    }
    println!("spawn clear");

    for h in handles.into_iter()
    {
        h.await.unwrap();
    }

    println!("task clear");
}

async fn some_palallel_limit()
{
    // build runtime
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
//        .thread_name("my-custom-name")
//        .thread_stack_size(3 * 1024 * 1024)
        .enable_time()
        .build()
        .unwrap();
    println!("rt build");

    let mut handles = Vec::new();
    for i in 15..30
    {
        handles.push(rt.spawn_blocking(
            move ||
            {
                println!("Task executes on thread: {:?}", thread::current().id());
                std::thread::sleep(Duration::from_millis(i *1000));
                println!("{ } sec elapsed", i);
            }
        ));
    }
    println!("spawn clear");

    for h in handles.into_iter()
    {
        h.await.unwrap();
    }

    rt.shutdown_background();
    println!("task clear");
}

#[tokio::main]
async fn main()
{
//    some_palallel_once().await;
//    some_palallel().await;
    some_palallel_limit().await;

}