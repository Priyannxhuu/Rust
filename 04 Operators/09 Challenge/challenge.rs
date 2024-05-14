// Problem Statement

/*
The task is to compute 
(𝑎+𝑏)^3
 
 and display it on the console. The following mathematical formula is expanded as:

(𝑎+𝑏)^3 = 𝑎^3 + 𝑏^3 + 3𝑎𝑏(𝑎+𝑏)
*/

fn main(){
    let a = 2;
    let b = 2;
    let c = i32::pow(a,3) + i32::pow(b,3) + (3*a*b*(a+b));
    println!("{}" , c);
}