// Integers


// Fixed Size
/*
Below is the list of fixed length integer types:

i8: The 8-bit signed integer type.

i16: The 16-bit signed integer type.

i32: The 32-bit signed integer type.

i64: The 64-bit signed integer type.

u8: The 8-bit unsigned integer type.

u16: The 16-bit unsigned integer type.

u32: The 32-bit unsigned integer type.

u64: The 64-bit unsigned integer type.
*/

// Variable Size
/*
isize: The pointer-sized signed integer type.
usize: The pointer-sized unsigned integer type.
*/

fn main(){

    // Explicitly define an integer
    let a:i32 = 24;
    let b:u64 = 23;
    let c:usize = 26;
    let d:isize = 29;
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}

fn main() {

    //implicitly define an integer
    let a = 21; 
    let b = 1;
    let c = 54;
    let d = 343434;
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}

// Floating Point

/*
f32: The 32-bit floating point type.
f64: The 64-bit floating point type.
*/

fn main() {
    //explicitly define a float type
    let f1:f32 = 32.9;
    let f2:f64 = 6789.89;
    println!("f1: {}", f1);
    println!("f2: {}", f2);
}

fn main() {
    //implicitly define a float type
    let pi = 3.14;
    let e = 2.17828;
    println!("pi: {}", pi);
    println!("e: {}", e);
}