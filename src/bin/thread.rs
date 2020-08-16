use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Mutex, Arc};


fn main() {
    normal_channel_test();

    normal_thread_test();

    thread_channel();

    add_in_single_thread();

    add_in_multi_thread();
}

fn normal_channel_test() {
    let (tx, rx) = channel();

    tx.send(String::from("normal_channel_test@hello world")).unwrap();

    for txt in rx.recv() {
        println!("{}", txt)
    }
}

fn normal_thread_test() {
    let th = thread::spawn(move || {
        println!("thread start")
    });

    println!("normal_thread_test@hello world");

    th.join().unwrap();
}

fn thread_channel() {

    let (tx, rx) = channel();

    let th = thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
        }
    });

    th.join().unwrap();
    for i in rx {
        println!("current recv number: {}", i)
    }

}

fn add_in_multi_thread() {
    let sum = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..10 {
        let sum = Arc::clone(&sum);
        let handler = thread::spawn(move || {
            let mut num = sum.lock().unwrap();
            *num += 1;
        });
        threads.push(handler);
    }

    for handler in threads {
        handler.join().unwrap();
    }

    println!("sum = {}", *sum.lock().unwrap());
}

fn add_in_single_thread() {
    let sum = Mutex::new(5);
    {
        let mut num = sum.lock().unwrap();
        *num += 10;
    }

    println!("sum = {:?}", sum);

    let sum_counter = Arc::new(Mutex::new(0));

    // 真的很奇怪，为什么要加上这样一个作用域才能编译成功？
    {
        let sum_counter = Arc::clone(&sum_counter);
        let handler = thread::spawn(move || {
            let mut num = sum_counter.lock().unwrap();
            for i in 0..10 {
                *num += i;
            }

            println!("xx sum_counter = {:?}", *num);
        });
        handler.join().unwrap();

    }
    println!("the result of sum_counter: {:?}", sum_counter)
}
