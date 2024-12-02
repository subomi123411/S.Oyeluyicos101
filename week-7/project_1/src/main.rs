<<<<<<< HEAD
use std::io;

fn main() {
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - 3200");
    println!("F = Fried Rice & Chicken - 3000");
    println!("A = Amala & Ewedu Soup - 2500");
    println!("E = Eba & Egusi Soup - 2000");
    println!("W = Poundo Yam/Edinkaiko Soup - 2500");

    // Ask for order
    println!("\nWhat would you like to order?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1 = input1.trim(); 

    // Ask for quantity
    println!("\nWhat quantity would you like?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let input2: f32 = input2.trim().parse().expect("Invalid quantity"); 

    // Initialize total
    let mut total = 0.0;

    if input1 == "p" {
        total = input2 * 3200.0;
    } else if input1 == "f" {
        total = input2 * 3000.0;
    } else if input1 == "a" {
        total = input2 * 2500.0;
    } else if input1 == "e" {
        total = input2 * 2000.0;
    } else if input1 == "w" {
        total = input2 * 2500.0;
    } else {
        println!("Error: Invalid order.");
    }

    if total > 0.0 {
        println!("\nYour total is {}", total);
    }
    if total > 10000.0 {
        let discount = total*0.95;
        println!("Your discount is {}", discount);
    }
=======
fn main() {
    println!("Hello, world!");
>>>>>>> fba56df ((5th week))
}
