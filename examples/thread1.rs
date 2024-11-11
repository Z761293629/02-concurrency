use anyhow::Result;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
#[allow(dead_code)]
struct Message {
    index: usize,
    value: u64,
}

impl Message {
    fn new(index: usize, value: u64) -> Self {
        Self { index, value }
    }
}

const LEN: usize = 4;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Message>();

    for index in 0..LEN {
        let tx = tx.clone();
        thread::spawn(move || producer(index, tx));
    }
    drop(tx);

    let consumer = thread::spawn(|| {
        for message in rx {
            println!("{:?}", message);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow::anyhow!("consumer error {:?}", e))?;

    Ok(())
}

fn producer(index: usize, tx: Sender<Message>) -> Result<()> {
    loop {
        tx.send(Message::new(index, rand::random()))?;
        let millisecond = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(millisecond));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", index);
            break;
        }
    }
    Ok(())
}
