//! Destruction of resources whose GPU work has finished on the relay path.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::sync::mpsc::{self, SyncSender};

// Bound how many landings can retain resources if destruction falls behind.
// The submitting and relay threads fall back to local destruction on overflow.
const QUEUED_BATCHES: usize = 4;
static WORKER: OnceLock<Option<SyncSender<Batch>>> = OnceLock::new();
static STATS: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];

#[derive(Default)]
pub(crate) struct Batch(Vec<Box<dyn Send>>);

impl Batch {
    pub(crate) fn push<T: Send + 'static>(&mut self, value: T) {
        self.0.push(Box::new(value));
    }

    /// Call after publishing a completed landing and releasing its locks.
    pub(crate) fn dispatch(self) {
        if self.0.is_empty() {
            return;
        }
        self.dispatch_to(WORKER.get_or_init(start).as_ref());
    }

    fn dispatch_to(self, worker: Option<&SyncSender<Batch>>) {
        let inline = match worker {
            Some(worker) => match worker.try_send(self) {
                Ok(()) => return,
                Err(mpsc::TrySendError::Full(batch) | mpsc::TrySendError::Disconnected(batch)) => batch,
            },
            None => self,
        };
        STATS[2].fetch_add(1, Relaxed);
        let start = std::time::Instant::now();
        drop(inline);
        STATS[3].fetch_add(start.elapsed().as_micros() as u64, Relaxed);
    }
}

fn start() -> Option<SyncSender<Batch>> {
    let (send, receive) = mpsc::sync_channel::<Batch>(QUEUED_BATCHES);
    let launched = std::thread::Builder::new().name("victory-release".into()).spawn(move || {
        use windows_sys::Win32::System::Threading::{GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_BELOW_NORMAL};
        // SAFETY: the priority change applies only to this thread.
        let _ = unsafe { SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_BELOW_NORMAL) };
        for batch in receive {
            let start = std::time::Instant::now();
            drop(batch);
            STATS[0].fetch_add(1, Relaxed);
            STATS[1].fetch_add(start.elapsed().as_micros() as u64, Relaxed);
        }
    });
    if let Err(error) = launched {
        tracing::warn!(%error, "could not start the resource release worker");
        return None;
    }
    Some(send)
}

pub(super) fn log(frames: f64) {
    let [jobs, worker_us, full, inline_us] = STATS.each_ref().map(|n| n.swap(0, Relaxed) as f64 / frames);
    if jobs + full > 0.0 {
        tracing::info!(
            "completed resources released per frame: {jobs:.1} background batches in {:.2} ms, {full:.1} inline fallbacks in {:.2} ms",
            worker_us / 1000.0,
            inline_us / 1000.0,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SendThread(mpsc::Sender<std::thread::ThreadId>);

    impl Drop for SendThread {
        fn drop(&mut self) {
            let _ = self.0.send(std::thread::current().id());
        }
    }

    #[test]
    fn dispatch_destroys_resources_on_a_worker() {
        let (send, receive) = mpsc::channel();
        let mut batch = Batch::default();
        batch.push(SendThread(send));
        batch.dispatch();
        let thread = receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert_ne!(thread, std::thread::current().id());
    }

    #[test]
    fn a_full_queue_falls_back_to_inline_destruction() {
        let (worker, _queue) = mpsc::sync_channel(1);
        assert!(worker.try_send(Batch::default()).is_ok());
        let (send, receive) = mpsc::channel();
        let mut batch = Batch::default();
        batch.push(SendThread(send));

        batch.dispatch_to(Some(&worker));
        assert_eq!(receive.try_recv().unwrap(), std::thread::current().id());
    }
}
