use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::Instant;
use tokio::time::Interval;

pub fn tokio_runtime() -> Result<tokio::runtime::Runtime, std::io::Error> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
}

#[pin_project::pin_project]
struct EventHandler {
    #[pin]
    interval: Interval,
    now: Instant,
}

impl EventHandler {
    fn new() -> Self {
        let interval = tokio::time::interval(Duration::from_secs(2));
        Self {
            interval,
            now: Instant::now(),
        }
    }
}

impl Future for EventHandler {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        println!("polled");
        if this.interval.poll_tick(cx).is_ready() {
            println!("is ready, elapsed {:?}", this.now.elapsed());
            *this.now = Instant::now();
        }

        // cx.waker().wake_by_ref();

        Poll::Pending
    }
}

fn main() {
    let rt = tokio_runtime().unwrap();

    rt.block_on(async {
        let handler = EventHandler::new();
        handler.await;
    });
}
