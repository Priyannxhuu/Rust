fn main() {
    for i in 0..10 {
        println!("i:{}",i);
        if i == 5 {
            break;
        }
    }
}

fn main () {
    let mut i = 1;
    loop{
        println!("i:{}",i);
        if i == 5 {
            break;
        }
        i = i + 1
    }
} // fn main () {
    //     let mut i = 1;
    //     loop{
    //         println!("i:{}",i);
    //         if i == 5 {
    //             break;
    //         }
    //         i = i + 1
    //     }
    // }