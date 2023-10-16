// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    data: Arc<Mutex<Vec<u32>>>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            data: Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])),
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: mpsc::Sender<u32>) {
    let q_clone = Arc::clone(&q);

    thread::spawn(move || {
        let data = q_clone.lock().unwrap();
        for val in &*data.data.lock().unwrap() {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = 10;

    send_tx(Arc::new(Mutex::new(queue)), tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
