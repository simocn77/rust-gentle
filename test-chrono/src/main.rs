use chrono::prelude::*;

fn main() {

    let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now();

    println!("{}", utc);
    println!("{}", local);

    let date = Local.with_ymd_and_hms(2010,3,14, 00, 70, 00);
    println!("date was {:#?}", date);

    let date = Local.with_ymd_and_hms(2010,3,14, 00, 59, 00);
    println!("date was {:#?}", date);
    // date was Single(2010-03-14+02:00)

    let date = Local.with_ymd_and_hms(2014,24,52, 00, 00, 00);
    println!("date was {:?}", date);
    // date was None
}
