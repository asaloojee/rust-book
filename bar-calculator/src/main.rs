use std::io;

fn main() {
    println!("Welcome to the barbell weight calculator!");

    let mut fourty_five_plates = String::new();
    let mut ten_plates = String::new();
    let mut five_plates = String::new();

    println!("Please enter the number of 45-lbs plates on each side of the bar:");

    io::stdin()
        .read_line(&mut fourty_five_plates)
        .expect("Please enter a number.");

    let fourty_five_plates: u32 = fourty_five_plates
        .trim()
        .parse()
        .expect("Please enter a number.");

    println!("Please enter the number of 10-lbs plates on each side of the bar:");

    io::stdin()
        .read_line(&mut ten_plates)
        .expect("Please enter a number.");

    let ten_plates: u32 = ten_plates.trim().parse().expect("Please enter a number.");

    println!("Please enter the number of 5-lbs plates on each side of the bar:");

    io::stdin()
        .read_line(&mut five_plates)
        .expect("Please enter a number.");

    let five_plates: u32 = five_plates.trim().parse().expect("Please enter a number.");

    let total_plate_weight: u32 =
        (fourty_five_plates * 2 * 45) + (ten_plates * 2 * 10) + (five_plates * 2 * 5);

    let total_plate_weight = total_plate_weight as f32;

    let total_lbs = total_plate_weight + 45.0;
    let total_kgs: f32 = total_lbs / 2.2;

    println!("Your total weight is: {total_lbs} lbs / {total_kgs} kgs.");
}
