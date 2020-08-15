use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Deu ruim ao abrir o arquivo");
}

