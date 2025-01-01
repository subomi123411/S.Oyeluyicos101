use std::fs::File;
use std::io::Write;

fn main() {
    // Define the drink categories and details
    let data = "Category\tDrink\n\
                Lager:\t33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n\
                Stout:\tLegend, Turbo King, Williams\n\
                Non-Alcoholic:\tMaltina, Amstel Malta, Malta Gold, Fayrouz";

    // Create and write to a file
    let mut file = File::create("drinks.txt").expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write to file");

    println!("Data has been written to drinks.txt");
}