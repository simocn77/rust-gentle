
use std::thread;
use std::sync::mpsc; //'Multiple Producer Single Consumer'

fn main() {

    println!("--Channel async --");

    let nthreads = 5;
    let (tx, rx) = mpsc::channel();

    for i in 0..nthreads {
        let tx = tx.clone();
        thread::spawn(move || {
            let response = format!("hello {}", i);
            //tx.send(response).unwrap();
            tx.send(response).expect("tx fail");
        });
    }

    for _ in 0..nthreads {
        println!("got {:?}", rx.recv());
    }

    println!("--Channel sync --");
    let (tx, rx) = mpsc::sync_channel(0);

    let t1 = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });

    for _ in 0..5 {
        let res = rx.recv().unwrap();
        println!("{}", res);
    }

    t1.join().unwrap();

}
