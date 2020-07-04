use tokio::runtime::Runtime;

use std::{io::Read, time::Instant};

const URL: &str = "http://example.com";

fn main() {
    let n = 100;
    {
        let start = Instant::now();
        let res = blocking(n);
        println!("blocking {:?} {} bytes", start.elapsed(), res);
    }
    {
        let start = Instant::now();
        let mut rt = Runtime::new().unwrap();
        let res = rt.block_on(non_blocking(n));
        println!("async    {:?} {} bytes", start.elapsed(), res);
    }
}

fn blocking(n: usize) -> usize {
    (0..n)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                let mut body = ureq::get(URL).call().into_reader();
                let mut buf = Vec::new();
                body.read_to_end(&mut buf).unwrap();
                buf.len()
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|it| it.join().unwrap())
        .sum()
}

async fn non_blocking(n: usize) -> usize {
    let tasks = (0..n)
        .into_iter()
        .map(|_| {
            tokio::spawn(async move {
                let body = reqwest::get(URL).await.unwrap().bytes();
                body.await.unwrap().len()
            })
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for task in tasks {
        res += task.await.unwrap();
    }
    res
}
