// Tuples

// Tuples are heterogeneous sequences of elements, meaning, each element in a tuple can have a different data type. Just like arrays, tuples are of a fixed length.

// Syntax

// let tuple_name = ("value1" , "c" , 1);
// let tuple_name :(&str , char , i32) = ("value1" , "c" , 1);

#[allow(unused_variable , unused_mut)]
fn main(){
    let person1 = ("Priyanshu" , 19 , "5.11");
    let person2 : (&str , i32 , &str) = ("Priyanshu" , 19 , "5.11");
    let (w,x,y) = person1;
    println!("My name is {}" , w);
    println!("My age is {}" ,x);
    println!("My height is {}" ,y);
}

// use mut to make tuple mutable