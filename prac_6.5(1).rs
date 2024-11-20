#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name.clone(); 

    // ONLY modify this line
    println!("{}, {}, {:?}", _name, f.data, f); 
}
