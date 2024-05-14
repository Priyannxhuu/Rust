// Character
// Note: Unlike some other languages, a character in Rust takes up 4 bytes rather than a single byte. It does so because it can store a lot more than just an ASCII value like emojis, Korean, Chinese, and Japanese characters.


// Explict
fn main(){
    let char_1:char = 'e';
    println!("Char1: {}" , char_1);
}

// Implict
fn main(){
    let char_1 = 'e';
    println!("Char1: {}" , char_1);
}