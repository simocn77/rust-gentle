#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;

fn run() -> Result<()> {
    use std::fs::File;
    File::open("file")?;
    Ok(())
}

fn run2() -> Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = args().skip(1).next()
        .ok_or(Error::from("provide file name as arg "))?;

    // non-specific error
    //let f = File::open(&file)?;

    // a specific chained error
    let f = File::open(&file).chain_err(|| "unable to read the damn file")?;

    let mut l = 0;
    for line in BufReader::new(f).lines() {
        //let line = line?;
        let line = line.chain_err(|| "cannot read a line")?;
        println!("{}", line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("error: {}", e);
        //std::process::exit(1);
    }

    if let Err(e) = run2() {
        println!("error: {}", e);
        //std::process::exit(1);
    }

    println!("-- Chaining Errors --");



}
// error: No such file or directory (os error 2)
