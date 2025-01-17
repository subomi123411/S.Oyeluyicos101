struct Employee {
    name:String,
    company:String,
    age: u32
}

fn main() {
    let emp1 = Employee {
        company: String::from("Ernst & Young"),
        name: String::from("Ebibliong Jessica"),
        age: 25
    };
    println!("Name = {}", emp1.name);
    println!("Company = {}", emp1.company);
    println!("Age = {}", emp1.age);
}
