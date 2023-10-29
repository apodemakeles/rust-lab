use std::time::Duration;

use tokio::{spawn, time};
use tokio::task::JoinHandle;

async fn sleep(sec: u64) {
    time::sleep(Duration::from_secs(sec)).await;
    println!("task of sleep {} is done", sec);
}

fn spawn_sleep(sec: u64) -> JoinHandle<()> {
    spawn(async move {
        time::sleep(Duration::from_secs(sec)).await;
        println!("task of sleep {} is done", sec);
    })
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;

    use futures::future::{join_all, try_join_all};

    use super::*;

    macro_rules! execute_block {
        ($block:block) => {
            let start = std::time::Instant::now();
            $block
            println!("Time elapsed is: {}", start.elapsed().as_secs());
        }
    }

    #[tokio::test]
    async fn future_wait_test() {
        let futures = vec![sleep(5), sleep(1)];
        execute_block!({
            for future in futures{
                let _ = future.await;
            }
        });
    }

    #[tokio::test]
    async fn spawn_wait_test() {
        let handles = vec![spawn_sleep(5), spawn_sleep(1)];
        execute_block!({
            for handle in handles {
                let _ = handle.await;
            }
        });
    }

    #[tokio::test]
    async fn future_join_test() {
        let futures = vec![sleep(5), sleep(1)];
        execute_block!({
            let _ = join_all(futures).await;
        });
    }

    #[tokio::test]
    async fn future_try_join_test() {
        let futures: Vec<Pin<Box<dyn Future<Output=Result<(), String>>>>> = vec![
            Box::pin(async {
                tokio::time::sleep(Duration::from_secs(5)).await;
                println!("sleep 5 is done");
                Err("sleep 5 is failure".to_string())
            }),
            Box::pin(async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                Err("sleep 1 is failure".to_string())
            }),
        ];

        execute_block!({
            let result = try_join_all(futures).await;
            println!("result is {:?}", result);
        });
    }
}