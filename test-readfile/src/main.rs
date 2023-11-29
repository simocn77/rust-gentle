use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;

use std::env;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command,Stdio};

fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;

/*
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
*/

    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        {
            let line = buf.trim_end();
            println!("{}", line);
        }
        buf.clear();
    };
    Ok(())
}

fn write_out(f: &str) -> io::Result<()> {
    let mut out = File::create(f)?;
    write!(out, "answer is {}\n", 42)?;
    Ok(())
}

fn dump_dir(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();
        if data.is_file() {
            if let Some(ex) = path.extension() {
                if ex == "rs" && data.len() > 10 {
                    println!("{} length {}", path.display(),data.len());
                }
            }
        }
    }
    Ok(())
}

fn main() {
    println!("Read:");

    let res = read_all_lines("simo");
    println!("{:?}", res);

    let res = read_all_lines("none");
    println!("{:#?}", res);

    println!("Writing To stdout:");

    let mut stdout = io::stdout();
    write!(stdout,"answer is {}\n", 42).expect("write failed");

    println!("Writing To Files:");
    write_out("test.txt").expect("Create or write fail !");

    println!("Files, Paths and Directories");

    #[allow(deprecated)]
    let home = env::home_dir().expect("no home!");
    println!("{}", home.display());

    let mut path = PathBuf::new();
    path.push(home);
    path.push(".cargo");

    if path.is_dir() {
        println!("{}", path.display());
    }

    println!("Current dir...");
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        println!("{}", path.display());
        if ! path.pop() {
            break;
        }
    }

    println!("Trick to find file...");
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        path.push("simo");
        if path.is_file() {
            println!("find: {}", path.display());

            match path.metadata() {
                Ok(data) => {
                    println!("\t type {:?}", data.file_type());
                    println!("\t len {}", data.len());
                    println!("\t perm {:?}", data.permissions());
                    println!("\t perm {:o}",data.permissions().mode()); // Note '{:o}' for printing out in octal)
                    println!("\t modified {:#?}", data.modified());
                },
                Err(e) => println!("error {:?}", e)
            }

            break;
        } else {
            path.pop();
        }
        if ! path.pop() {
            break;
        }
    }

    println!("Here are all files with extension '.rs' and size greater than 10 bytes...");
    let _ = dump_dir("src");

    println!("-- Processes & output on terminal --");

    let status = Command::new("rustc")
        .arg("-V")
        .status()
        .expect("no rustc?");

    println!("cool {} code {}", status.success(), status.code().unwrap());

    println!("-- Processes & capture Output--");

    let output = Command::new("rustc")
        .arg("-V")
        .output()
        .expect("no rustc?");

        if output.status.success() {
            println!("ok!");
        }
        println!("len stdout {} stderr {}", output.stdout.len(), output.stderr.len());

    println!("-- Processes child as thread --");

    let mut child = Command::new("rustc")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .expect("no rustc?");

    // do something else in the meantime!

    let res = child.wait();
    println!("res {:?}", res);

}
