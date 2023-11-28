#[macro_use]
extern crate json;

fn main() {
    let mut doc = json::parse(r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#).expect("parse failed");

//    println!("debug {:?}", doc);
    println!("display {}", doc);

    let code = doc["code"].as_u32().unwrap_or(0);
    let success = doc["success"].as_bool().unwrap_or(false);

    assert_eq!(code, 200);
    assert_eq!(success, true);

//    let features = &doc["payload"]["features"];

    let features = &mut doc["payload"]["features"];
    features.push("cargo!").expect("couldn't push");

    for v in features.members() {
        println!("{}", v.as_str().unwrap())  // MIGHT explode
    }

    // truly beautiful use of macros to generate JSON literals => needs #[macro_use]
    let data = object!{
        "name"    => "John Doe",
        "age"     => 30,
        "numbers" => array![10,53,553]
    };
    assert_eq!(
        data.dump(),
        r#"{"name":"John Doe","age":30,"numbers":[10,53,553]}"#
    );

    println!("display {}", data);

}
