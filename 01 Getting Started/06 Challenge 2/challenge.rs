// Print this to console
/*
1
22
333
4444
55555
*/

// Method 1
fn main(){
    println!("1\n22\n333\n4444\n55555");
}

// Method 2
fn main() {
    println!("{}", 1);
    println!("{}{}", 2, 2);
    println!("{}{}{}", 3, 3, 3);
    println!("{}{}{}{}", 4, 4, 4, 4);
    println!("{}{}{}{}{}", 5, 5, 5, 5, 5);
}