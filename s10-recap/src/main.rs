use std::fmt::Display;

fn main() {
    // this function uses lifetimes ('a), generics (<T>), and trait bounds
    fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
    }

    let string1 = "bobert";
    let string2 = "billiam";

    let result = longest_with_announcement(string1, string2, "I like burgers!");
    println!("The longest string is {result}");
}
