//! An example of stopping a spawned thread from it's parent
use std::sync::mpsc::{self, TryRecvError::*};
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc - multiple producer, single consumer
    // tx - transmitter, rx - receiver
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Err(Empty) => {
                    println!("Spawned: sleeping");
                    thread::sleep(Duration::from_secs(1));
                }
                Err(Disconnected) => {
                    println!("Spawned: the channel is dropped");
                    break;
                }
                Ok(_) => {
                    println!("Spawned: got the stop signal");
                    break;
                }
            }
        }
        println!("Spawned: stopped");
        return "returned from thread";
    });

    thread::sleep(Duration::from_secs(5));

    tx.send(()).unwrap();
    println!("   Main: a stop signal is sent");

    println!("   Main: waiting for the spawned thread to stop");
    dbg!(handle.join().unwrap());
}
