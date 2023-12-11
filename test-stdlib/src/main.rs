use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

    let v = vec![10,20,30,40,50];
    assert_eq!(v.iter().position(|&i| i == 30).unwrap(), 2);

    let nums = ["5","5h2","65"];
    let iter = nums.iter().map(|s| s.parse::<i32>());
    let converted: Vec<_> = iter.clone().collect();
    println!("{:?}",converted);
    let converted: Result<Vec<_>,_> = iter.collect();
    println!("{:?}",converted);

    let nums = ["5","52","65"];
    let iter = nums.iter().map(|s| s.parse::<i32>());
    let converted: Vec<_> = iter.clone().collect();
    println!("{:?}",converted);
    let converted: Result<Vec<_>,_> = iter.collect();
    println!("{:?}",converted);

    println!("-- Maps simple --");
    let entries = [("one","eins"),("two","zwei"),("three","drei")];
    if let Some(val) = entries.iter().find(|t| t.0 == "two") {
        assert_eq!(val.1,"zwei");
    }

    println!("-- Maps --");
    let mut map = HashMap::new();
    map.insert("one","eins");
    map.insert("two","zwei");
    map.insert("three","drei");
    assert_eq! (map.contains_key("two"), true);
    assert_eq! (map.get("two"), Some(&"zwei"));

    map.insert("four", "quattro");
    println!("{:#?}", map);
    let quattro = map.get("four").unwrap().to_string();
    println!("{}", quattro);

    let mut map = HashMap::new();
    map.insert("one",1);
    map.insert("two",2);
    map.insert("three",3);
    println!("before {}", map.get("two").unwrap());
    {
        let mref = map.get_mut("two").unwrap();
        *mref = 20;
    }
    println!("after {}", map.get("two").unwrap());

    for (k,v) in map.iter() {
        println!("key {} value {}", k,v);
    }

    println!("-- Example: Counting Words --");

    let text = "1.2.3, Provo a capire qualcosa, qualcosa !! ";
    let mut map = HashMap::new();

    for s in text.split(|c: char| ! c.is_alphabetic()) {
        let word = s.to_lowercase();
        let count = map.entry(word).or_insert(0);
        *count += 1;
        //println!("{:?}", map);
    }
    println!("{:?}", map.len());
    println!("{:?}", map);

    println!("find the twenty most common words?");

    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|a,b| b.1.cmp(&a.1));
    for e in entries.iter().take(20) {
        println!("{} {}", e.0, e.1);
    }

    println!("-- Sets --");
    let fruit = make_set("apple orange pear orange banana");
    println!("{:?}", fruit);

    let colours = make_set("brown purple orange yellow");
    println!("{:?}", colours);

    for c in fruit.intersection(&colours) {
        println!("{:?}",c);
    }

}

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}
