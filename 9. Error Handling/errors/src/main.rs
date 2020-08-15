#![allow(unused_variables)]
fn main() {
<<<<<<< HEAD
	use std::fs::File;
	use std::io;
	use std::io::Read;

	fn read_username_from_file() -> Result<String, io::Error> {
		let mut f = File::open("hello.txt")?;
		let mut s = String::new();
		f.read_to_string(&mut s)?;
		Ok(s)
	}
=======
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
>>>>>>> b5e3cad2e14b66e3088ea879a9952216ce41e53b
}

