fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess  = "42".parse().expect("Not a number!"); // removing the anottation

    // Floating-Point Types
    let x = 2.0; //f64 default
    let y: f32 = 3.0; //explict f32 type

    //// Numeric Operations
    
    // addition
    let sum = 5 + 10;

    // subtraction

    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    
    // remainder
    let remainder = 43 % 5;

    ///// The boolan type
    
    let t = true;
    let f: bool = false;
    
    //// The Character Type
    
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    

    //// Compound Types (tuples or arrays)

    // The tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup; // destructuring (like unpacking in python)
    println!("The value of y is:  {}", y);

    // access a tuple element directly by using a period
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // The Array Type

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // explici type and size of array
    let first = a[0];
    let second = a[1];


}
