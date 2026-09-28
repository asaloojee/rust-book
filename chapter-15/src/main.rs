// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };

    println!("customsmartpointers created");

    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
