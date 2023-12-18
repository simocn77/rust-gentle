use std::fmt::Debug;

fn main() {

    // a vector of char ['h','e','l','l','o']
    let v: Vec<_> = "hello".chars().collect();
    println!("got {:?}", v);
    // a string "doy"
    let m: String = "dolly".chars().filter(|&c| c != 'l').collect();
    println!("got {:?}", m);


    let answer = 42;
    let message = "hello";
    let float = 2.7212;

    let display: Vec<&dyn Debug> = vec![&message, &answer, &float];

    for d in display {
        println!("got {:#?}", d);
    }

    let s1 = "hello".to_string();
    let s2 = s1.clone();
    assert!(s1 == s2);  // cool
    assert!(s1 == "hello"); // fine
    assert!(s1 == s2.as_str()); // ok


}
