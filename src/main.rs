//! Simulating files one step at a time.

extern crate rand;
use rand::Rng;
use std::fmt;
use std::fmt::Display;

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

// Represents the allowed states of a "file".
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Represents a "file", which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    /// Creates a new, empty 'File'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Creates a new file with the provided name and value for the data.
    ///
    /// # Example
    ///
    /// ```rust
    /// let f = File::new_with_data("f.txt", &vec![123, 111]);s
    /// ```
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    // Returns the file's length in bytes.
    pub fn len(self: &File) -> usize {
        self.data.len()
    }

    // Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    // Reads the given file and saves it to the mutable vector provided.
    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }

    // Returns the data of a file in string format.
    pub fn to_string(self: &File) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f = File::new_with_data("f.txt", &f_data);

    let mut buffer: Vec<u8> = vec![];

    if f.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{} is {} bytes long", &f.name, f.len());
    println!("{} is f_length", f_length);
    println!("{} is File::to_string", f.to_string());
    // Following along in the book, but this will actually be: rust!rust!
    // Mutable is mutable, so the error check reads it
    println!("{} is buffer", text);

    println!("{:?}", f);
    println!("{}", f);
}
