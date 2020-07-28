fn main() {
    let mut x = 5; // using mut you can change the value
    println!("The value of x  is: {}", x);
    x = 6;
    println!("The value of x  is: {}", x);

    //shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // creating new var and changing his type
   
    // let mut spaces = "    ";
    // spaces = spaces.len(); // error because you are trying to change the type
}
