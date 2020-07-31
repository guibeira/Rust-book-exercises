fn main() {
    let number = 7;

    if number != 0 {
        println!("number was something other than zero");
    }

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // returning values from loops

    let mut counter=0;
    let result = loop {
        counter +=1;
        
        if counter == 10 {
            break counter * 2; // return the value
        }
    }; 
    println!("The result is {}", result);

    // Conditional Loops with while

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -=1;
    }
    println!("LIFTOFF!!!");

    // Looping through  collection with for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { // using index
        println!("The value is {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is {}", element);
    } 

    for number in (1..10).rev() {
        println!("The reversed number is: {}", number);
    }
}