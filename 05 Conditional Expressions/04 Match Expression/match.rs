// Match Expression
/*
Match expression checks if the current value corresponds to any value within the list of values.

Match expression are similar to switch statement in languages like C and C++. They give a more compact code when compared with the if/else construct.
*/

// Method 1
/*
match match_value{
    value 1 => {
        statements;
    },
    value 2 => {
        statements;
    },
    _ =>{
        default value;
    }
};

*/

fn main(){
    let x = 5;
    match x {
        1 => println!("Java"),
        2 => println!("Python"),
        3 => println!("C++"),
        4 => println!("C#"),
        5 => println!("Rust"),
        6 => println!("Kotlin"),
        _ => println!("Some other value"),
    };
}

// Method 2
fn main(){
    let course = "Rust";
    let found_course = match course {
       "Rust" => "Rust",
       "Java" => "Java",
       "C++" => "C Plus Plus",
       "C#" => "C Sharp",
       _ => "Unknown Language"
    };
    println!("Course name : {}",found_course);
}