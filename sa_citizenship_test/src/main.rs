use inquire::Select;

#[derive(Debug)]
enum Fruit {
    Papaya,
    Strawberry,
    Apple,
    Mango,
    Chestnut,
}

fn main() {
    let options = vec!["Papaya", "Strawberry", "Apple", "Mango", "Chestnut"];

    let test = Select::new(
        "South African citizenship test. Choose your favourite fruit:",
        options,
    )
    .prompt()
    .expect("Choose one of the available fruits.");

    let answer = match test {
        "Papaya" => Fruit::Papaya,
        "Strawberry" => Fruit::Strawberry,
        "Apple" => Fruit::Apple,
        "Mango" => Fruit::Mango,
        "Chestnut" => Fruit::Chestnut,
        _ => unreachable!(),
    };

    if matches!(answer, Fruit::Mango) {
        println!("Success. You are South African.");
    } else if matches!(answer, Fruit::Chestnut) {
        println!("I think that's a nut. Can you read?");
    } else {
        println!("Failed. Please try again.");
    }
}
