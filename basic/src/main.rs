use std::{f32::consts::PI, fs::File, io::{Read, self}};

fn square(x: f64) -> f64 {
    x * x
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn factorial_other(num: u64) -> u64 {
    (1..num + 1).fold(1, |acc, x| acc * x)
}

fn sum_array(value: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..value.len() {
        res += value[i];
    };
    res
}

#[allow(dead_code)]
fn sample_main() {
    println!("ciao");

    let answer = 45;
    println!("I'm {}", answer);
    assert_eq!(answer, 45);

    for i in 1..6 {
        if i % 2 == 0 {
            println!("n = {} -> odd", i);
        } else {
            println!("n = {} -> even", i);
        }
    }

    for i in 1..6 {
        let result = if i % 2 == 0 { "odd" } else { "even" };
        println!("n: {} is {}", i, result)
    }

    let mut sum = 0.0;
    for i in 0..6 {
        sum += i as f32;
    }
    println!("sum : {}", sum);

    println!("{}", square(5.0));
    println!("{}", factorial(5));
    println!("{}", factorial_other(5));

    let x = PI/2.0;
    let cosine = x.cos();
    println!("{}", cosine);

    let arr = [10, 20 , 30, 40];
    println!("arr length {}", arr.len());
    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
    }

    println!("sum array {}", sum_array(&arr));
    println!("sum array {}", arr.iter().sum::<i32>());
    println!("sum array {}", arr.iter().fold(0, |acc, x| acc + x));

    println!("arr {:?} , slice of arr {:?}", &arr, &arr[0..3]);


    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(arr.len());
    let morethanlast = slice.get(arr.len() + 1);

    println!("len {:?}, first {:?}, last {:?}, morethanlast {:?}", arr.len(), first , last, morethanlast);

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    println!("array {:?}", &v[..]);
    println!("slice {:?}", &v[1..]);
    println!("ref vec {:?}", &v);
    println!("vec {:?}", v);

    println!("Iterators:");
    let mut iter = 0..3;
    println!("{:?}", iter);
    assert_eq!(iter.next(), Some(0));
    println!("{:?}", iter);
    assert_eq!(iter.next(), Some(1));
    println!("{:?}", iter);
    assert_eq!(iter.next(), Some(2));
    println!("{:?}", iter);
    assert_eq!(iter.next(), None);
    println!("{:?}", iter);

    let arr = [10, 20, 30];
    println!("arr {:?}", arr);
    println!("arr.iter {:?}", arr.iter());

    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
    assert_eq!(sum, 60);

    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }

    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }

    println!("String:");
    let txt = "Simone";
    let mut str = txt.to_string();
    str.push('_');
    str.push_str("Cianni");
    str += " : Ciao!!".into();

    println!("{}", txt);
    println!("{}", &str); // Again, the borrow operator can coerce String into &str (Coercion)

    let arr = "world";
    let res = format!("hello {}", arr);
    assert_eq!(res, "hello world");

    let text = "the red fox jump over the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", words);

    let mut words = Vec::new();
    words.extend(text.split_whitespace());
    println!("{:?}", words);

    let stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect(); // filter method takes a closure,
    println!("{:?}", stripped);
}

#[allow(dead_code)]
fn sample_main1() {

    println!("Interlude: Getting Command Line Arguments");

    let first = std::env::args().nth(1).expect("please supply an argument");
    let a: i32 = first.parse().expect("not an integer!");
    let first = std::env::args().nth(2).expect("please supply a second argument");
    let b: i32 = first.parse().expect("not an integer!");
    println!("{}, {}", a, b);

    for arg in std::env::args() {
        println!("'{}'", arg);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 { // we have args!
        println!("args {:?}, {:?}", args, args.len());
    }
}

/*
fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}
*/

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    println!("Reading from Files");
/*
    let first = std::env::args().nth(1).expect("please supply a file name");
    let mut file = File::open(&first).expect("can't open the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    println!("file had {} bytes", text.len());
*/
    let file = std::env::args().nth(1).expect("please supply a filename");
    let text = read_file_to_string(&file).expect("bad file man!");
    println!("file had {} bytes", text.len());
}