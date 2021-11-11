extern crate tokio;
extern crate async_recursion;

use async_recursion::async_recursion;
use tokio::spawn;

// macro use
#[async_recursion]
async fn fibo(n :u64) -> u64
{
    if n == 0
    {
        return 0_u64;
    }
    else if n == 1
    {
        return 1_u64;
    }

    let result1 = spawn(fibo(n - 1));
    let result2 = spawn(fibo(n - 2));
    result1.await.unwrap() + result2.await.unwrap()
}



#[tokio::main]
async fn main()
{
    let result = fibo(7).await;
    println!("{ }", result);
}