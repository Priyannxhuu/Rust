fn main () {
    for var in 0..10 {
        if var == 4 {
            println!("I encountered a continue statement");
            continue;
        }
        println!("var: {}", var);
        println!("I did not encounter continue statement");
    }
}