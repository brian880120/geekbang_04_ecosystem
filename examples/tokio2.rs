use std::{thread, time::Duration};
use tokio::sync::mpsc;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    worker(rx);

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            i += 1;
            println!("Send task: {i}!");
            tx.send("Future 1". to_string()).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            let ret = expensive_blocking_task(s);
            println!("result: {}", ret);
        }
    })
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}
