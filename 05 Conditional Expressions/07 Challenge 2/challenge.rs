// Problem Statement

/*
Write a code which will take:

Two variables named a and b

a character type variable called operator which will take operators (+,-,/,*,%) will be passed as input to our match statement

Use match statements to compute: addition of a and b, subtraction of b from a, multiplication of a and b, division of a by b, modulus of a by b
*/

fn main(a:i32 , operator: char , b:i32){
    match operator {
        '+' => {
            println!("{}", a + b);
        },
        '-' => {
            println!("{}", a - b);
        },
        '*' => {
            println!("{}", a * b);
        },
        '/' => {
            if b == 0{
                println!("Division by 0 is undefined");
            }
            else {
                println!("{}", a / b);
            }
        },
        '%' => {
            println!("{}", a % b);
        },
        _ => println!("{}","invalid operator"),
    }
}