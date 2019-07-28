// Silences the warning in open/1 and close/1
#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn len(self: &File) -> usize {
        self.data.len()
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }

    fn to_string(self: &File) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // Has to be mutable in order to append
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    // Not necessary, but ensures there is sufficient space and minimizes
    // allocations when data is inserted byte-by-byte
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    // Convert to String. Any invalid UTF-8 chars replaced with �
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);

    let f3 = File::new("f3.txt");

    let f3_name = &f3.name;
    let f3_length = f3.data.len();

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3_name, f3_length);

    let f4 = File::new_with_data("f4.txt", &vec![114, 117, 115, 116, 33]);

    println!("{:?}", f4);
    println!("{} is {} bytes long", f4.name, f4.len());

    let f5_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f5 = File::new_with_data("f5.txt", &f5_data);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f5);
    let f5_length = f5.read(&mut buffer);
    close(&mut f5);

    println!("{:?}", f5);
    println!("{} is {} bytes long", f5.name, f5.len());
    println!("{}", f5.to_string());
}
