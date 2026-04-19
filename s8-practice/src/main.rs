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

        if buffer.starts_with("list") {
            let words = buffer.split_whitespace().collect::<Vec<&str>>();

            if words.len() == 2 {
                if words[1] == "all" {
                    println!("All staff: \n {:?}", staff);
                } else {
                    println!("Please enter a valid list command!")
                }
            } else if words.len() == 3 {
                let department = words[2];
                if words[1] == "all" && departments.contains(&department) {
                    if let Some(department_staff) = staff.get_mut(&String::from(department)) {
                        department_staff.sort();
                        println!("Staff in {department}: \n{:?}", department_staff);
                    }
                } else {
                    println!("Please enter a valid list command!")
                }
            } else {
                println!("Invalid list input. Try 'list all' or 'list all [department]'. ");
            }
        }

        println!("======LOOP ENDED======");
    }
    Ok(())
}
