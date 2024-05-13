fn main(){
    print!("Rust Programming");
    print!(" Course");
}

// The print!() macro simply prints the output to the console.

fn main(){
    println!("Rust Programming");
    println!("Course");
}

// The println!() macro appends a new line at the end of the string.

fn main(){
    eprint!("Rust Programming");
    eprint!(" Course");
}

// The eprint!() macro displays the output as an error and appends a new line(\n) at the end of it.

fn main(){
    eprintln!("Rust Programming");
    eprintln!("Course");
}

// eprint!() and eprintln!() come in handy when we want to indicate to the user that an error condition has occurred.