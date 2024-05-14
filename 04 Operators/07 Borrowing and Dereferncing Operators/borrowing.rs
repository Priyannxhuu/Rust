// Borrowing Operators
// Borrowing means to reference the original data binding or to share the data
// References are like pointers in C

// Syntax let operand 1 = & , &mut operand 2

// & this is share borrowing and &mut this is mutable borrowing


fn main(){
    let x = 10;
    let mut y = 13;
    // immutable reference
    let a = &x;
    println!("Value of a {}" , a);
    println!("Value of x {}" , x); // x value will be remains same since it is immutable borrowing
    // mutable borrowing
    let b = &mut y;
    println!("Value of b {}" , b);
    *b = 11; // derefencing
    println!("Value of b {}" , b); // updated value of b
    println!("Value of y {}" , y); // y value can be changed as it is mutable
}