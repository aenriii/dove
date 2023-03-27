use std::{thread, mem::MaybeUninit};

use super::network_result::NetworkResult;

pub struct ThreadedDownloadPool<const NUM_THREADS: usize> {
    threads: [MaybeUninit<thread::JoinHandle<NetworkResult<()>>>; NUM_THREADS],
    queue: Vec<String>,
}
impl <const NUM_THREADS: usize> ThreadedDownloadPool<NUM_THREADS> {
    pub fn new() -> Self {
        Self {
            threads: {
                const val: MaybeUninit<thread::JoinHandle<NetworkResult<()>>>  = MaybeUninit::uninit();
                [val; NUM_THREADS]
            },
           queue: Vec::new(),
        }
    }
}