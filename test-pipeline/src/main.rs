use pipeliner::Pipeline; // Provides a nice interface for parallel programming with iterators.
use std::process::Command;
use std::net::*;

fn main() {

    println!("-- do things in parallel and collect the results --");
    for result in (0..10).with_threads(4).map(|x| x + 100) {
        println!("result: {}", result);
    }

    println!("-- ping on a range of IP4 addresses --");
    let addresses: Vec<_> = (1..5).map(|n| format!("ping -c1 10.59.160.{}",n)).collect();
    //println!("{:#?}", addresses);
    let n = addresses.len();

    for result in addresses.with_threads(n).map(|s| shell(&s)) {
        if result.1 {
            println!("got: {}", result.0);
        }
    }

    println!("-- A Better Way to Resolve Addresses --");
    for res in "google.com:80".to_socket_addrs().expect("bad") {
        println!("got {:?}", res);
    }
/* ... da sempre success
    //
    let addresses: Vec<_> = (1..5).map(|n| format!("10.59.160.{}:80",n)).collect();
    let n = addresses.len();
    for result in addresses.with_threads(n)
        .map(|s| s.to_socket_addrs().unwrap().next().unwrap())
    {
        println!("got: {:?}", result);
    }
*/

}

fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("no shell?");

    //println!("{:#?}", output);

    //return:
    (
        String::from_utf8_lossy(&output.stdout).trim_end().to_string(),
        output.status.success()
    )
}
