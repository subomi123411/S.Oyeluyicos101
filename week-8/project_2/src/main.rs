use std::io;

fn main() {
    let mut names = Vec::new();
    let mut experiences = Vec::new();

    println!("Enter applicant details. Type 'done' to stop.");

    loop {
        let mut name = String::new();
        println!("Enter applicant's name:");
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();

        if name.to_lowercase() == "done" {
            break;
        }
        names.insert(names.len(), name);

        let mut experience = String::new();
        println!("Enter years of programming experience:");
        io::stdin()
            .read_line(&mut experience)
            .expect("Failed to read input");

        let experience: u32 = experience
            .trim()
            .parse()
            .expect("Invalid number. Please enter a valid number.");
        experiences.insert(experiences.len(), experience);
    }

    if names.is_empty() {
        println!("No applicants entered.");
        return;
    }

    let mut max_experience = 0;
    let mut most_experienced = "";

    for i in 0..experiences.len() {
        if experiences[i] > max_experience {
            max_experience = experiences[i];
            most_experienced = &names[i]; // Use a reference
        }
    }

    println!(
        "The most experienced applicant is {} with {} years of programming experience.",
        most_experienced, max_experience
    );
}
