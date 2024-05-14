// Assignment Operator
// Assignment operator is used to save value in the variable

// fn main(){
//     let a = 2;
//     let b = a;
//     println!("Assignment Operator");
//     println!("Value of a: {}" , a);
//     println!("Valur of b: {}" , b);
// }

// Compound Assignment Operators

fn main(){
    let mut a = 2;
    println!("a: {}" , a);
    a += 1;
    println!("a += 1: {}" , a);
    println!("a: {}" , a);
    a -= 1;
    println!("a -= 1:{}" , a);
    println!("a: {}" , a);
    a /= 1;
    println!("a/=1:{}", a);
    println!("a:{}", a);
    a *= 3;
    println!("a*=3:{}", a);
}