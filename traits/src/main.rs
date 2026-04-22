trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;
struct Cow;

impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}

impl Speak for Cow {
    fn speak(&self) -> String {
        "Moo!".to_string()
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    let cow = Cow;

    println!("Dog says: {}", dog.speak());
    println!("Cat says: {}", cat.speak());
    print!("Cow says: {}", cow.speak());
}
