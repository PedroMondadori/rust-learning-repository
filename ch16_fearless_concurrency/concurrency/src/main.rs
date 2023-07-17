use concurrency::{channels, mutexes, sync_send};

fn main() {
    channels::mpsc_example();
    mutexes::mutex_example_single_threaded();
    mutexes::mutex_example_multi_threaded();
}
