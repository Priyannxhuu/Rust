// Dereferencing Operator

// * operand 1 = operand 2 this will dereference a value

fn main(){
    let mut x = 10;
    println!("Value of x {}" , x);
    let a = &mut x;
    println!("Value of a {}" , a);
    // dereference a variable
    *a = 11;
    println!("Value of a {}" ,a);
    println!("Value of x {}" ,x); // value of x is updated
}