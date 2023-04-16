use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
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
}

impl EventHandler {
    fn new() -> Self {
        let interval = tokio::time::interval(Duration::from_secs(2));
        Self { interval }
    }
}

impl Future for EventHandler {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        dbg!("polled");
        if this.interval.poll_tick(cx).is_ready() {
            dbg!("is ready");
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
