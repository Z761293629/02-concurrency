use std::{thread, time::Duration};

use concurrency::metric::{self, Inc, Metric};
use rand::Rng;

const M: usize = 5;

fn main() {
    let metric = Metric::<metric::DashMetric>::new();

    for idx in 0..M {
        task_worker(idx, metric.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{metric}");
    }
}

fn task_worker<T>(idx: usize, metric: Metric<T>)
where
    T: Inc + Send + 'static,
{
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metric.inc(format!("call.worker-{idx}"))?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
}
