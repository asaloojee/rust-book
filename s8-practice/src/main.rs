use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let departments = ["engineering", "sales", "finance", "legal", "culinary"];
    let mut staff: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        println!("Enter some text:");
        stdin.read_line(&mut buffer)?;
        let buffer = buffer.trim();

        if buffer == "quit" {
            break;
        }

        if buffer.starts_with("add") {
            let words = buffer.split_whitespace().collect::<Vec<&str>>();
            let subject = words[1];
            let destination = words[3];

            if departments.contains(&destination) {
                staff
                    .entry(String::from(destination))
                    .or_default()
                    .push(String::from(subject));
            } else {
                println!("Please select a valid department!");
            }
            println!("Staff: {:?}", staff);
        }

        println!("User said: {buffer}. Looping again");
    }
    Ok(())
}
