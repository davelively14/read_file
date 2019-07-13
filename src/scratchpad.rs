#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    // String::from allows owned Strings to be generated from string literals
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    // Accessing fields by reference prevents use after move issues (i.e.
    // ownership issues). We indicate that we're borrowing the reference.
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
