use tokio::time::{sleep, Duration};
use std::error::Error;

async fn sleep_short() {
    sleep(Duration::from_millis(100)).await;
    println!("Short sleep completed");
}

async fn sleep_long() {
    sleep(Duration::from_millis(1000)).await;
    println!("Long sleep completed");    
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    
    for i in 0..10{
        if i & 1 == 0 {
            sleep_short().await;
        }else{
            sleep_long().await;
        }
    }

    Ok(())
}