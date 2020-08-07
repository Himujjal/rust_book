use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    println!("------b_message_passing-----");
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs_f32(0.2));
        }
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs_f32(0.2));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}
