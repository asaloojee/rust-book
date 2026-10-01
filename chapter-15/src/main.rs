// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
// struct CustomSmartPointer {
//     data: String,
// }
//
// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data)
//     }
// }

use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };

    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };

    // println!("customsmartpointers created");
    //
    // drop(c);
    // println!("CustomSmartPointer dropped before the end of main");

    let a = Rc::new(Cons(5, Rc::new(Nil)));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}
