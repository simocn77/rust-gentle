trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

trait Quack {
    fn quack(&self);
}

struct Duck ();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

struct RandomBird {
    is_a_parrot: bool
}

impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

// and why the hell not!
impl Quack for i32 {
    fn quack(&self) {
        for i in 0..*self {
            print!("quack {} ",i);
        }
        println!("");
    }
}

trait Named {
    fn name(&self) -> String;

    fn upper_case(&self) -> String {
        self.name().to_uppercase()
    }
}

struct Boo();

impl Named for Boo {
    fn name(&self) -> String {
        "boo".to_string()
    }
}

fn main() {

    println!("-- trait objects --");
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&dyn Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}",d.show());
    }

    println!("-- Box : A box contains a reference to data allocated on the heap,
    and acts very much like a reference - it's a smart pointer.
    When boxes go out of scope and Drop kicks in, then that memory is released --");
    let answer = Box::new(42);
    let maybe_pi = Box::new(3.14);

    let show_list: Vec<Box<dyn Show>> = vec![answer,maybe_pi];
    for d in &show_list {
        println!("show {}",d.show());
    }

    println!("-- other... --");
    let duck1 = Duck();
    let duck2 = RandomBird{is_a_parrot: false};
    let parrot = RandomBird{is_a_parrot: true};
    let nint = 4;

    let ducks: Vec<&dyn Quack> = vec![&duck1, &duck2, &parrot, &nint];

    for d in ducks.iter() {
        d.quack();
    }

    println!("-- one more ... --");
    let f = Boo();
    assert_eq!(f.name(),"boo".to_string());
    assert_eq!(f.upper_case(),"BOO".to_string());
    println!("{} -> {}", f.name(), f.upper_case());

    println!("-- sample of generics ... --");
    let d = Duck();
    quack(&d);

    let ducks: Vec<Box<dyn Quack>> = vec![Box::new(duck1),Box::new(duck2),Box::new(parrot),Box::new(nint)];
    quack_everyone(ducks.into_iter());

    println!("-- inheritance ... --");




}

fn quack<Q> (q: &Q)
where Q: Quack {
    q.quack();
}

fn quack_everyone <I> (iter: I)
where I: Iterator<Item=Box<dyn Quack>> {
    for d in iter {
        d.quack();
    }
}

