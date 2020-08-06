fn main() {
    let my_string = String::from("hello world");
    
    let word = first_word(&my_string[..]);
    
    let my_string_literal = "hello world";
    
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let slice = &a[1..3];

    println!("{:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
