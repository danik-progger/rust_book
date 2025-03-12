use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();

    // MOVE
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); - error, v is moved to thread

    handle.join().unwrap();

    // --- CHANNELS ---
    let (sender, receiver) = mpsc::channel();
    let sender2 = sender.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("thr1 - mes1"),
            String::from("thr1 - mes2"),
            String::from("thr1 - mes3"),
            String::from("thr1 - mes4"),
        ];
        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("thr2 - mes1"),
            String::from("thr2 - mes2"),
            String::from("thr2 - mes3"),
            String::from("thr2 - mes4"),
        ];
        for val in vals {
            sender2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    // let received = receiver.recv().unwrap(); - for single value
    for received in receiver {
        println!("Got: {}", received);
    }

    // --- MUTEX ---
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
