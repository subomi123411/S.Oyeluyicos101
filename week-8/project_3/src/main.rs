fn main() {

let v: Vec<i32> = Vec::new();
println!("{}", v.is_empty()); // This will print `true` because the vector is empty.

let v = vec![1, 2, 3];
println!("{}", v.is_empty()); // This will print `false` because the vector has elements.
}