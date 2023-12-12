use std::thread;
//use std::time;

fn main() {
//    thread::spawn(|| println!("hello"));
//    thread::spawn(|| println!("dolly"));
//    println!("so fine");
    // wait a little bit
//    thread::sleep(time::Duration::from_millis(100));

    let t = thread::spawn(|| {
        println!("Hello");
    });
    println!("wait {:?}", t.join());

    let mut threads = Vec::new();

    for i in 0..5 {
        let t = thread::spawn(move || {
            println!("hello {}", i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed")
    }

    println!("all ended,
    note: Multithreading is easy; what's hard is concurrency.
     - managing and synchronizing multiple threads of execution.");

     println!(" -- Threads Don't Borrow --
     It's possible for the thread closure to capture values, but by moving, not by borrowing!,
     but : Threads can't share the same environment - by design in Rust. In particular,
     they cannot share regular references because the closures move their captured variables.,
     shared references are fine however");

     let name = "dolly";
     let t1 = thread::spawn(move || {
         println!("hello {}", name);
     });
     let t2 = thread::spawn(move || {
         println!("goodbye {}", name);
     });

     t1.join().expect("thread failed");
     t2.join().expect("thread failed");

     println!(" -- Channel -- end data between threads --");





}
