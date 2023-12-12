use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {

    println!("-- Test Mutex --");

    let answer = Arc::new(Mutex::new(42));
    println!("{:?}", answer);

    println!("-- Mutex : modify data in a thread --");

    let answer_ref = answer.clone();
    println!("{:?}", answer_ref);
    let t = thread::spawn(move || {
        let mut answer = answer_ref.lock().unwrap();
        *answer = 55;
    });

    t.join().unwrap();

    println!("-- Mutex is unlocked when the MutexGuard goes out of scope --");

    {
        let ar = answer.lock().unwrap();
        assert_eq!(*ar, 55);
        println!("{:?}", answer);
    }
    println!("{:?}", answer);

    println!("-- or when the MutexGuard is dropped ( goes out of scope ) --");

    let mut ar2 = answer.lock().unwrap();
    *ar2 = 66;
    assert_eq!(*ar2, 66);
    println!("{:?}", answer);
    std::mem::drop(ar2);
    println!("{:?}", answer);

}