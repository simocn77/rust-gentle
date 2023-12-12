
use std::thread;
use std::time;

pub mod client;
pub mod server2;

fn main() {

    // server
    let t1 = thread::spawn(move || {
        server2::listen();
    });
    thread::sleep(time::Duration::from_secs(1)); //wait server start

    // client
    for _ in 0..2 {
        client::connect();
        thread::sleep(time::Duration::from_secs(1));
    }

    //
    t1.join().expect("thread failed");
}
