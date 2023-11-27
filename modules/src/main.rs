mod foo;
mod boo;

fn main() {
    let f = foo::Foo::new("hello");
    let res = boo::answer();
    let q = boo::bar::question();
    println!("{:?} {} {}", f, res, q);

    {
        use boo::bar;
        let q = bar::question();
        println!("{}", q);
    }
    {
        use boo::bar::question;
        let q = question();
        println!("{}", q);
    }
}