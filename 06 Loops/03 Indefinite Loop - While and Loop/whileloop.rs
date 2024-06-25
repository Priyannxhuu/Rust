fn main() {
    let mut var = 1;
    let mut found = false;

    while !found {
        var = var + 1;
        println!("{}",var);
        if var % 3 == 1 {
            found = true;
        }
        println!("Loop runs");
    }
}