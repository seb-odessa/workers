extern crate crossbeam;
extern crate crossbeam_queue;

use crossbeam_queue::SegQueue;
use crossbeam_utils::thread::scope;
use std::{thread, time};

fn handler_one(src: &SegQueue<usize>, dst: &SegQueue<f32>, delay: u64, label: &str) {
    let sleep = time::Duration::from_millis(delay);
    loop {
        if let Ok(value) = src.pop() {
            thread::sleep(sleep);
            if 0 == value {
                println!("[{:2}] QUIT", label);
                dst.push(0.0);
                break;
            }
            let result = value as f32 * 2.0;
            println!("[{:2}]  {:6} => {:6.2}", label, value, result);
            dst.push(result);
        }
    }
}

fn handler_two(src: &SegQueue<f32>, delay: u64, label: &str) {
    let sleep = time::Duration::from_millis(delay);
    loop {
        if let Ok(value) = src.pop() {
            thread::sleep(sleep);
            if 0.0 == value {
                println!("[{:2}] QUIT", label);
                src.push(value);
                break;
            }
            let result = value > 5.0;
            println!("[{:2}]  {:6.2} => {:6}", label, value, result);
        }
    }
}

fn producer(q: &SegQueue<usize>, count: usize) {
    for i in 1..count+1 {
        q.push(i)
    }
    q.push(0);
}


fn main() {
    println!("Begin");

    let count: usize = 8;
    let q1 = SegQueue::new();
    let q2 = SegQueue::new();
    scope(|scope| {
        scope.spawn(|_| handler_one(&q1, &q2, 1, "1"));
        scope.spawn(|_| handler_two(&q2, 3, "2"));
        scope.spawn(|_| handler_two(&q2, 7, "3"));
        scope.spawn(|_| handler_two(&q2, 5, "4"));
        scope.spawn(|_| producer(&q1, count));
    })
    .unwrap();

    println!("Done");
}
