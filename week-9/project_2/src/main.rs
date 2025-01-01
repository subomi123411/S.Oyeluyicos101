use std::io;
use std::io::Write;

fn main() {
    // Create vectors to store user data
    let mut names = Vec::new();
    let mut matrics = Vec::new();
    let mut departments = Vec::new();
    let mut levels = Vec::new();

    // Open a file to write the data
    let mut file = std::fs::File::create("sibas.txt").expect("Failed to create file");

    // Write headers to the file
    file.write_all(b"PAU SMIS\n").unwrap();
    file.write_all(b"Name                               Matric                             Department                        Level\n").unwrap();

    // Start getting user input
    loop {
        let mut input = String::new();

        // Get name input
        println!("Enter your name (or type 'done' to finish):");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();
        if name.to_lowercase() == "done" {
            break; // Exit the loop if the user types "done"
        }
        names.push(name);

        // Get matric input
        println!("Enter your matric number:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        matrics.push(input.trim().to_string());

        // Get department input
        println!("Enter your department:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        departments.push(input.trim().to_string());

        // Get level input
        println!("Enter your level:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let level: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid level. Please enter a number.");
                continue; // Ask again if the level is invalid
            }
        };
        levels.push(level);
    }

    // Write all the collected data to the file
    for i in 0..names.len() {
        let row = format!(
            "{:<35}{:<35}{:<35}{:<5}\n",
            names[i], matrics[i], departments[i], levels[i]
        );
        file.write_all(row.as_bytes()).unwrap();
    }

    println!("Data saved to 'sibas.txt'.");
}
