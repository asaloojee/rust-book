trait Speak {
    fn speak(&self) -> String;
}

trait Named {
    fn named(&self) -> String;
}

struct Dog;
struct Cat;

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

impl Named for Dog {
    fn named(&self) -> String {
        "Kat".to_string()
    }
}

impl Named for Cat {
    fn named(&self) -> String {
        "Doug".to_string()
    }
}

fn announce<T: Speak>(animal: T) {
    println!("Announcement: {}", animal.speak());
}

fn name<T: Speak, B: Named>(animal: T) {
    println!("Dog is named {} and says {}!", animal.)
}

// Same thing, shorter syntax:
// fn announce(animal: impl Speak) { ... }

fn main() {
    announce(Dog);
    announce(Cat);
}
